use eframe::egui;

use crate::net::ws::NetCmd;
use crate::ui::UiState;

pub fn ui_connect(ui: &mut egui::Ui, st: &mut UiState, send: &mut dyn FnMut(NetCmd)) {
    ui.heading("Connect");

    ui.horizontal(|ui| {
        ui.label("WS URL:");
        ui.text_edit_singleline(&mut st.url_input);

        if ui.button("Connect").clicked() {
            let url = st.url_input.clone();
            send(NetCmd::Connect { url });
        }
    });

    ui.label(&st.status_line);
}
