pub mod screens;
pub mod widgets;

pub mod popups;

use shared::net::{RoomInfo, ServerMsg};
use shared::types::{GameState, Turn};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Connect,
    Lobby,
    Game,
}

#[derive(Debug, Clone)]
pub struct PopupState {
    pub title: String,
    pub message: String,
    pub back_to_lobby: bool,
}

pub struct UiState {
    pub screen: Screen,

    pub url_input: String,
    pub connected: bool,
    pub status_line: String,

    pub rooms: Vec<RoomInfo>,
    pub selected_room: Option<usize>,
    pub create_name: String,
    pub create_vs_bot: bool,
    pub current_room_id: Option<String>,

    pub your_role: Option<Turn>,
    pub game_state: Option<GameState>,

    pub use_hex_view: bool,
    pub hex_size: f32,

    pub log: Vec<String>,

    pub lobby_auto_refresh: bool,
    pub lobby_refresh_every: Duration,
    pub lobby_next_refresh_at: Instant,

    pub popup: Option<PopupState>,
}

impl UiState {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            screen: Screen::Connect,

            url_input: "ws://127.0.0.1:3000/ws".to_string(),
            connected: false,
            status_line: "Disconnected".to_string(),

            rooms: Vec::new(),
            selected_room: None,
            create_name: "room".to_string(),
            create_vs_bot: false,
            current_room_id: None,

            your_role: None,
            game_state: None,

            use_hex_view: true,
            hex_size: 22.0,

            log: Vec::new(),

            lobby_auto_refresh: true,
            lobby_refresh_every: Duration::from_secs(2),
            lobby_next_refresh_at: now,

            popup: None,
        }
    }

    pub fn push_log(&mut self, s: impl Into<String>) {
        self.log.push(s.into());
        if self.log.len() > 200 {
            let drop_count = self.log.len() - 200;
            self.log.drain(0..drop_count);
        }
    }

    pub fn on_connected(&mut self) {
        self.connected = true;
        self.status_line = "Connected".to_string();
        self.screen = Screen::Lobby;
        self.lobby_next_refresh_at = Instant::now();
        self.push_log("Connected");
    }

    pub fn on_disconnected(&mut self) {
        self.connected = false;
        self.status_line = "Disconnected".to_string();

        self.screen = Screen::Connect;

        self.current_room_id = None;
        self.your_role = None;
        self.game_state = None;

        self.rooms.clear();
        self.selected_room = None;

        self.popup = None;

        self.lobby_next_refresh_at = Instant::now();

        self.push_log("Disconnected: returned to Connect and cleared session state");
    }

    pub fn on_server_msg(&mut self, msg: ServerMsg) {
        match msg {
            ServerMsg::RoomList { rooms } => {
                self.rooms = rooms;
                if self.selected_room.is_some_and(|i| i >= self.rooms.len()) {
                    self.selected_room = None;
                }
                self.push_log(format!("RoomList updated ({} rooms)", self.rooms.len()));
            }
            ServerMsg::LobbyState {
                room_id,
                players,
                vs_bot,
            } => {
                if let Some(r) = self.rooms.iter_mut().find(|r| r.room_id == room_id) {
                    r.players = players;
                    r.vs_bot = vs_bot;
                }
                self.push_log(format!("LobbyState: players={players}, vs_bot={vs_bot}"));
            }
            ServerMsg::GameStart { state, your_role } => {
                self.your_role = Some(your_role);
                self.game_state = Some(state);
                self.screen = Screen::Game;
                self.push_log(format!("GameStart: role={your_role:?}"));
            }
            ServerMsg::GameUpdate { state } => {
                self.game_state = Some(state);
            }
            ServerMsg::Error { message } => {
                self.push_log(format!("Server Error: {message}"));

                let m = message.to_ascii_lowercase();

                let is_opponent_left = m.contains("opponent left");
                let is_game_not_started = m.contains("game not started");

                let in_running_game = self.screen == Screen::Game
                    && self
                        .game_state
                        .as_ref()
                        .is_some_and(|gs| gs.status == shared::types::GameStatus::Running);

                if self.popup.is_none()
                    && in_running_game
                    && (is_opponent_left || is_game_not_started)
                {
                    self.popup = Some(PopupState {
                        title: "Session ended".to_string(),
                        message,
                        back_to_lobby: true,
                    });
                }
            }
        }
    }
}
