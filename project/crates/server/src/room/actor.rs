use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::{mpsc, oneshot, watch};
use uuid::Uuid;

use shared::ai::choose_mouse_move;
use shared::hex::inside_board;
use shared::net::ServerMsg;
use shared::rules::apply_action;
use shared::types::{Action, BoardConfig, Coord, GameState, GameStatus, Turn};

#[derive(Debug, Clone)]
pub struct RoomSnapshot {
    pub players: u8,
    pub started: bool,
}

#[derive(Clone)]
pub struct RoomHandle {
    pub room_id: String,
    pub name: String,
    pub vs_bot: bool,
    pub cmd_tx: mpsc::UnboundedSender<RoomCmd>,
    pub snapshot_rx: watch::Receiver<RoomSnapshot>,
}

pub enum RoomCmd {
    Join {
        client_id: Uuid,
        client_tx: mpsc::UnboundedSender<ServerMsg>,
        reply: oneshot::Sender<Result<(), String>>,
    },
    Leave {
        client_id: Uuid,
    },
    Action {
        client_id: Uuid,
        action: Action,
    },
}

#[derive(Clone)]
struct Player {
    id: Uuid,
    tx: mpsc::UnboundedSender<ServerMsg>,
}

pub fn spawn_room(room_id: String, name: String, vs_bot: bool) -> RoomHandle {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<RoomCmd>();
    let (snapshot_tx, snapshot_rx) = watch::channel(RoomSnapshot {
        players: 0,
        started: false,
    });

    tokio::spawn(room_loop(
        room_id.clone(),
        name.clone(),
        vs_bot,
        cmd_rx,
        snapshot_tx,
    ));

    RoomHandle {
        room_id,
        name,
        vs_bot,
        cmd_tx,
        snapshot_rx,
    }
}

async fn room_loop(
    room_id: String,
    name: String,
    vs_bot: bool,
    mut cmd_rx: mpsc::UnboundedReceiver<RoomCmd>,
    snapshot_tx: watch::Sender<RoomSnapshot>,
) {
    let mut trapper: Option<Player> = None;
    let mut mouse: Option<Player> = None;

    let mut started = false;
    let mut state: Option<GameState> = None;

    let radius: i32 = 6;
    let initial_blocks: usize = 8;

    let update_snapshot = |trapper: &Option<Player>, mouse: &Option<Player>, started: bool| {
        let players = (trapper.is_some() as u8) + (mouse.is_some() as u8);
        let _ = snapshot_tx.send(RoomSnapshot { players, started });
    };

    let broadcast = |msg: ServerMsg, trapper: &Option<Player>, mouse: &Option<Player>| {
        if let Some(p) = trapper.as_ref() {
            let _ = p.tx.send(msg.clone());
        }
        if let Some(p) = mouse.as_ref() {
            let _ = p.tx.send(msg);
        }
    };

    let send_to = |who: Uuid, msg: ServerMsg, trapper: &Option<Player>, mouse: &Option<Player>| {
        if let Some(p) = trapper.as_ref().filter(|p| p.id == who) {
            let _ = p.tx.send(msg);
        } else if let Some(p) = mouse.as_ref().filter(|p| p.id == who) {
            let _ = p.tx.send(msg);
        }
    };

    let run_bot_if_needed = |mut cur: GameState| -> GameState {
        loop {
            if !vs_bot || cur.status != GameStatus::Running || cur.turn != Turn::Mouse {
                break;
            }

            let Some(to) = choose_mouse_move(&cur) else {
                cur.status = GameStatus::TrapperWon;
                break;
            };

            let before = cur.clone();

            match apply_action(cur, Action::MoveMouse { to }) {
                Ok(next) => {
                    cur = next;
                }
                Err(_) => {
                    cur = before;
                    cur.status = GameStatus::TrapperWon;
                    break;
                }
            }
        }

        cur
    };

    while let Some(cmd) = cmd_rx.recv().await {
        match cmd {
            RoomCmd::Join {
                client_id,
                client_tx,
                reply,
            } => {
                if started {
                    let _ = reply.send(Err("Game already started in this room".to_string()));
                    continue;
                }

                if trapper.as_ref().is_some_and(|p| p.id == client_id)
                    || mouse.as_ref().is_some_and(|p| p.id == client_id)
                {
                    let _ = reply.send(Err("Already in this room".to_string()));
                    continue;
                }

                if trapper.is_none() {
                    trapper = Some(Player {
                        id: client_id,
                        tx: client_tx,
                    });
                    let _ = reply.send(Ok(()));
                } else if mouse.is_none() && !vs_bot {
                    mouse = Some(Player {
                        id: client_id,
                        tx: client_tx,
                    });
                    let _ = reply.send(Ok(()));
                } else {
                    let _ = reply.send(Err("Room is full".to_string()));
                    continue;
                }

                update_snapshot(&trapper, &mouse, started);

                let players = (trapper.is_some() as u8) + (mouse.is_some() as u8);
                broadcast(
                    ServerMsg::LobbyState {
                        room_id: room_id.clone(),
                        players,
                        vs_bot,
                    },
                    &trapper,
                    &mouse,
                );

                let should_start_pvp = !vs_bot && trapper.is_some() && mouse.is_some();
                let should_start_bot = vs_bot && trapper.is_some();

                if (should_start_pvp || should_start_bot) && !started {
                    started = true;

                    let seed = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_nanos() as u64)
                        .unwrap_or_else(|_| 0x9E3779B97F4A7C15);

                    let gs = make_initial_state(radius, initial_blocks, seed);
                    state = Some(gs.clone());

                    update_snapshot(&trapper, &mouse, started);
                    if let Some(t) = trapper.as_ref() {
                        let _ = t.tx.send(ServerMsg::GameStart {
                            state: gs.clone(),
                            your_role: Turn::Trapper,
                        });
                    }
                    if let Some(m) = mouse.as_ref() {
                        let _ = m.tx.send(ServerMsg::GameStart {
                            state: gs.clone(),
                            your_role: Turn::Mouse,
                        });
                    }

                    if vs_bot && gs.turn == Turn::Mouse && gs.status == GameStatus::Running {
                        let after_bot = run_bot_if_needed(gs);
                        state = Some(after_bot.clone());
                        broadcast(ServerMsg::GameUpdate { state: after_bot }, &trapper, &mouse);
                    }
                }
            }

            RoomCmd::Leave { client_id } => {
                let was_started = started;

                let running_game = state
                    .as_ref()
                    .is_some_and(|gs| gs.status == GameStatus::Running);

                let mut changed = false;

                if trapper.as_ref().is_some_and(|p| p.id == client_id) {
                    trapper = None;
                    changed = true;
                }
                if mouse.as_ref().is_some_and(|p| p.id == client_id) {
                    mouse = None;
                    changed = true;
                }

                if !changed {
                    continue;
                }

                if was_started && running_game {
                    let msg = ServerMsg::Error {
                        message: "Opponent left mid-game".to_string(),
                    };

                    if let Some(p) = trapper.as_ref().filter(|p| p.id != client_id) {
                        let _ = p.tx.send(msg.clone());
                    }
                    if let Some(p) = mouse.as_ref().filter(|p| p.id != client_id) {
                        let _ = p.tx.send(msg.clone());
                    }

                    trapper = None;
                    mouse = None;
                    started = false;
                    let _ = state.take();
                    update_snapshot(&trapper, &mouse, started);
                    break;
                }

                started = false;
                state = None;

                update_snapshot(&trapper, &mouse, started);

                let players = (trapper.is_some() as u8) + (mouse.is_some() as u8);
                broadcast(
                    ServerMsg::LobbyState {
                        room_id: room_id.clone(),
                        players,
                        vs_bot,
                    },
                    &trapper,
                    &mouse,
                );
            }

            RoomCmd::Action { client_id, action } => {
                let Some(gs_ref) = state.as_ref() else {
                    send_to(
                        client_id,
                        ServerMsg::Error {
                            message: "Game not started".to_string(),
                        },
                        &trapper,
                        &mouse,
                    );
                    continue;
                };

                let gs = gs_ref.clone();

                let allowed = match gs.turn {
                    Turn::Trapper => trapper.as_ref().is_some_and(|p| p.id == client_id),
                    Turn::Mouse => {
                        if vs_bot {
                            false
                        } else {
                            mouse.as_ref().is_some_and(|p| p.id == client_id)
                        }
                    }
                };

                if !allowed {
                    send_to(
                        client_id,
                        ServerMsg::Error {
                            message: "Not your turn".to_string(),
                        },
                        &trapper,
                        &mouse,
                    );
                    continue;
                }

                match apply_action(gs, action) {
                    Ok(new_state) => {
                        state = Some(new_state.clone());

                        broadcast(
                            ServerMsg::GameUpdate {
                                state: new_state.clone(),
                            },
                            &trapper,
                            &mouse,
                        );

                        if new_state.status != GameStatus::Running {
                            trapper = None;
                            mouse = None;
                            started = false;
                            let _ = state.take();
                            update_snapshot(&trapper, &mouse, started);
                            break;
                        }

                        if vs_bot
                            && new_state.status == GameStatus::Running
                            && new_state.turn == Turn::Mouse
                        {
                            let after_bot = run_bot_if_needed(new_state);
                            state = Some(after_bot.clone());
                            broadcast(ServerMsg::GameUpdate { state: after_bot }, &trapper, &mouse);
                        }
                    }
                    Err(e) => {
                        send_to(
                            client_id,
                            ServerMsg::Error {
                                message: e.to_string(),
                            },
                            &trapper,
                            &mouse,
                        );
                    }
                }
            }
        }
    }

    tracing::info!("Room task ended: {} ({})", room_id, name);
}

fn make_initial_state(radius: i32, initial_blocks: usize, seed: u64) -> GameState {
    let cfg = BoardConfig {
        radius,
        initial_blocks,
        seed,
    };

    let mouse = Coord { q: 0, r: 0 };

    let mut all = Vec::new();
    for q in -radius..=radius {
        for r in -radius..=radius {
            let c = Coord { q, r };
            if inside_board(c, radius) && c != mouse {
                all.push(c);
            }
        }
    }

    let mut rng = XorShift64::new(seed);
    fisher_yates(&mut all, &mut rng);

    let blocks: HashSet<Coord> = all.into_iter().take(initial_blocks).collect();

    GameState {
        cfg,
        mouse,
        blocks,
        turn: Turn::Trapper,
        status: GameStatus::Running,
    }
}

struct XorShift64 {
    x: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self {
            x: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.x;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.x = x;
        x
    }

    fn gen_usize(&mut self, upper_exclusive: usize) -> usize {
        (self.next_u64() as usize) % upper_exclusive.max(1)
    }
}

fn fisher_yates<T>(v: &mut [T], rng: &mut XorShift64) {
    for i in (1..v.len()).rev() {
        let j = rng.gen_usize(i + 1);
        v.swap(i, j);
    }
}
