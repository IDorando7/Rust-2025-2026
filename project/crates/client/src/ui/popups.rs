use eframe::egui;

use shared::net::ClientMsg;

use crate::net::ws::NetCmd;
use crate::ui::{Screen, UiState};

pub fn ui_popups(ctx: &egui::Context, st: &mut UiState, send: &mut dyn FnMut(NetCmd)) {
    let Some(popup) = st.popup.as_ref() else {
        return;
    };

    let title = popup.title.clone();
    let message = popup.message.clone();
    let back_to_lobby = popup.back_to_lobby;

    let mut open = true;
    let mut go_lobby = false;
    let mut close_popup = false;

    egui::Window::new(title)
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label(message);
            ui.separator();

            ui.horizontal(|ui| {
                if back_to_lobby && ui.button("Back to lobby").clicked() {
                    go_lobby = true;
                }
                if ui.button("Close").clicked() {
                    close_popup = true;
                }
            });
        });

    if go_lobby {
        if st.connected && st.current_room_id.is_some() {
            send(NetCmd::Send(ClientMsg::LeaveRoom));
        }

        st.current_room_id = None;
        st.your_role = None;
        st.game_state = None;

        st.screen = if st.connected {
            Screen::Lobby
        } else {
            Screen::Connect
        };

        if st.connected {
            send(NetCmd::Send(ClientMsg::ListRooms));
        }

        st.push_log("Popup: back to lobby");
        st.popup = None;
        return;
    }

    if close_popup || !open {
        st.popup = None;
    }
}
