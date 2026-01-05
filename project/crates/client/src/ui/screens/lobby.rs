use eframe::egui;
use std::time::Instant;

use shared::net::ClientMsg;

use crate::net::ws::NetCmd;
use crate::ui::screens::rooms::ui_rooms;
use crate::ui::UiState;

pub fn ui_lobby(ui: &mut egui::Ui, st: &mut UiState, send: &mut dyn FnMut(NetCmd)) {
    ui.heading("Lobby");

    if st.connected && st.lobby_auto_refresh {
        let now = Instant::now();
        if now >= st.lobby_next_refresh_at {
            send(NetCmd::Send(ClientMsg::ListRooms));
            st.lobby_next_refresh_at = now + st.lobby_refresh_every;
        }
    }

    ui.horizontal(|ui| {
        if ui.button("Refresh rooms").clicked() {
            send(NetCmd::Send(ClientMsg::ListRooms));
            st.lobby_next_refresh_at = Instant::now() + st.lobby_refresh_every;
        }

        if ui.button("Leave room").clicked() {
            send(NetCmd::Send(ClientMsg::LeaveRoom));
            st.current_room_id = None;
            st.push_log("LeaveRoom requested");
        }

        if ui.button("Disconnect").clicked() {
            send(NetCmd::Disconnect);
            st.on_disconnected();
        }
    });

    ui.separator();

    ui.heading("Create room");
    ui.horizontal(|ui| {
        ui.label("name:");
        ui.text_edit_singleline(&mut st.create_name);
    });
    ui.checkbox(&mut st.create_vs_bot, "vs_bot");

    if ui.button("Create & join").clicked() {
        let name = st.create_name.clone();
        let vs_bot = st.create_vs_bot;
        send(NetCmd::Send(ClientMsg::CreateRoom { name, vs_bot }));
        st.push_log("CreateRoom requested");
    }

    ui.separator();

    ui_rooms(ui, st, send);

    ui.separator();

    let current_meta = st.current_room_id.as_deref().and_then(|id| {
        st.rooms
            .iter()
            .find(|r| r.room_id == id)
            .map(|r| (r.name.clone(), r.players, r.vs_bot))
    });

    match (st.current_room_id.as_deref(), current_meta) {
        (None, _) => {
            ui.label("Current room: (none)");
        }
        (Some(id), None) => {
            ui.label(format!("Current room: {id}"));
            ui.label("Refreshing room info...");
        }
        (Some(_id), Some((name, players, vs_bot))) => {
            ui.label(format!("Current room: {name}"));

            if !vs_bot && players == 1 {
                ui.label(egui::RichText::new("Waiting for opponent…").strong());
            }
        }
    }
}
