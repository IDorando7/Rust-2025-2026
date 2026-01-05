use eframe::egui;

use shared::net::ClientMsg;

use crate::net::ws::NetCmd;
use crate::ui::UiState;

pub fn ui_rooms(ui: &mut egui::Ui, st: &mut UiState, send: &mut dyn FnMut(NetCmd)) {
    ui.push_id("rooms_component", |ui| {
        ui.heading("Rooms");

        if st.rooms.is_empty() {
            ui.label("No rooms yet. Press Refresh or Create one.");
            return;
        }

        let mut new_selected = st.selected_room;

        egui::ScrollArea::vertical()
            .id_salt("rooms_scroll")
            .id_salt("rooms_scroll")
            .max_height(260.0)
            .show(ui, |ui| {
                for (i, r) in st.rooms.iter().enumerate() {
                    let is_full = r.players >= 2 && !r.vs_bot;

                    let mut label = format!("{} | players: {}", r.name, r.players);
                    if r.vs_bot {
                        label.push_str(" | BOT");
                    }
                    if is_full {
                        label.push_str(" | FULL");
                    }

                    let selected = new_selected == Some(i);
                    if ui.selectable_label(selected, label).clicked() {
                        new_selected = Some(i);
                    }
                }
            });

        st.selected_room = new_selected;

        let selected_room = st.selected_room.and_then(|i| st.rooms.get(i)).cloned();

        let join_allowed = selected_room
            .as_ref()
            .is_some_and(|r| r.vs_bot || r.players < 2);

        if let Some(r) = selected_room {
            let join_btn = ui.add_enabled(join_allowed, egui::Button::new("Join selected"));
            if join_btn.clicked() {
                let room_id = r.room_id.clone();

                send(NetCmd::Send(ClientMsg::JoinRoom {
                    room_id: room_id.clone(),
                }));
                st.current_room_id = Some(room_id);

                st.push_log(format!("Join requested: {}", r.name));
            }

            if !join_allowed && !r.vs_bot {
                ui.label(egui::RichText::new("Room is full (PvP).").strong());
            }
        } else if ui.button("Join selected").clicked() {
            st.push_log("No room selected");
        }
    });
}
