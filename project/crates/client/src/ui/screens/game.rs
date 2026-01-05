use eframe::egui;

use shared::net::ClientMsg;
use shared::types::{Action, Coord, GameStatus, Turn};

use crate::net::ws::NetCmd;
use crate::ui::widgets::hex_board::{BoardClickPolicy, HexBoard};
use crate::ui::{Screen, UiState};

pub fn ui_game(ui: &mut egui::Ui, st: &mut UiState, send: &mut dyn FnMut(NetCmd)) {
    ui.heading("Game");

    let is_vs_bot = st
        .current_room_id
        .as_deref()
        .and_then(|id| st.rooms.iter().find(|r| r.room_id == id).map(|r| r.vs_bot))
        .unwrap_or(false);

    if is_vs_bot {
        ui.label(egui::RichText::new("Playing vs bot").strong());
    }

    let role: Turn = match st.your_role {
        Some(r) => r,
        None => {
            ui.label("No role yet.");
            if ui.button("Back to lobby").clicked() {
                st.screen = Screen::Lobby;
            }
            return;
        }
    };

    let gs = match st.game_state.clone() {
        Some(s) => s,
        None => {
            ui.label("No game state yet.");
            if ui.button("Back to lobby").clicked() {
                st.screen = Screen::Lobby;
            }
            return;
        }
    };

    let game_over = gs.status != GameStatus::Running;

    if game_over {
        let you_won = match gs.status {
            GameStatus::TrapperWon => role == Turn::Trapper,
            GameStatus::MouseWon => role == Turn::Mouse,
            GameStatus::Running => false,
        };

        ui.separator();
        ui.label(egui::RichText::new("Game over").size(26.0).strong());
        ui.label(
            egui::RichText::new(if you_won {
                "YOU WON ✅"
            } else {
                "YOU LOST ❌"
            })
            .size(22.0)
            .strong(),
        );

        if ui.button("Leave room & back to lobby").clicked() {
            send(NetCmd::Send(ClientMsg::LeaveRoom));

            st.current_room_id = None;
            st.your_role = None;
            st.game_state = None;
            st.screen = Screen::Lobby;

            send(NetCmd::Send(ClientMsg::ListRooms));
            st.push_log("Left room after game end");
        }

        ui.separator();
    }

    ui.horizontal(|ui| {
        ui.label(format!("Role: {role:?}"));
        ui.separator();
        ui.label(format!("Turn: {:?}", gs.turn));
        ui.separator();
        ui.label(format!("Status: {:?}", gs.status));
    });

    ui.horizontal(|ui| {
        ui.checkbox(&mut st.use_hex_view, "Hex view");
        ui.add(
            egui::Slider::new(&mut st.hex_size, 12.0..=48.0)
                .text("hex size")
                .clamping(egui::SliderClamping::Always),
        );
    });

    ui.label(format!("Mouse: (q={}, r={})", gs.mouse.q, gs.mouse.r));
    ui.label(format!("Blocks: {}", gs.blocks.len()));
    ui.separator();

    let policy = BoardClickPolicy {
        your_role: role,
        current_turn: gs.turn,
        status: gs.status.clone(),
        mouse: gs.mouse,
        blocks: &gs.blocks,
        radius: gs.cfg.radius,
    };

    let mut board = HexBoard {
        hex_size: st.hex_size,
        use_hex_view: st.use_hex_view,
    };

    let clicked: Option<Coord> = board.draw(ui, &gs, &policy);

    if !game_over {
        if let Some(c) = clicked {
            match (role, gs.turn) {
                (Turn::Trapper, Turn::Trapper) => {
                    send(NetCmd::Send(ClientMsg::PlayerAction {
                        action: Action::PlaceBlock { at: c },
                    }));
                }
                (Turn::Mouse, Turn::Mouse) => {
                    send(NetCmd::Send(ClientMsg::PlayerAction {
                        action: Action::MoveMouse { to: c },
                    }));
                }
                _ => st.push_log("Clicked, but it's not your turn."),
            }
        }
    }

    ui.separator();
    if ui.button("Leave room").clicked() {
        send(NetCmd::Send(ClientMsg::LeaveRoom));
        st.current_room_id = None;
        st.your_role = None;
        st.game_state = None;
        st.screen = Screen::Lobby;
        send(NetCmd::Send(ClientMsg::ListRooms));
        st.push_log("LeaveRoom pressed");
    }

    let room_label = st
        .current_room_id
        .clone()
        .unwrap_or_else(|| "(none)".to_string());
    ui.label(format!("Current room: {room_label}"));
}
