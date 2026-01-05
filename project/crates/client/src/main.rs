mod net;
mod ui;

use eframe::egui;

use crate::net::ws::{start_ws_worker, NetCmd, NetEvent};

fn main() -> eframe::Result<()> {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .parse_default_env()
        .try_init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Trap The Mouse - Client",
        options,
        Box::new(|_cc| Ok(Box::new(ClientApp::new()))),
    )
}

struct ClientApp {
    cmd_tx: crossbeam_channel::Sender<NetCmd>,
    event_rx: crossbeam_channel::Receiver<NetEvent>,
    st: ui::UiState,
}

impl ClientApp {
    fn new() -> Self {
        let (cmd_tx, event_rx) = start_ws_worker();
        Self {
            cmd_tx,
            event_rx,
            st: ui::UiState::new(),
        }
    }

    fn drain_events(&mut self) {
        while let Ok(ev) = self.event_rx.try_recv() {
            match ev {
                NetEvent::Connected => self.st.on_connected(),
                NetEvent::Disconnected => self.st.on_disconnected(),
                NetEvent::Error(e) => self.st.push_log(format!("Error: {e}")),
                NetEvent::Server(msg) => self.st.on_server_msg(msg),
            }
        }
    }
}

impl eframe::App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drain_events();

        let st = &mut self.st;

        let cmd_tx = self.cmd_tx.clone();
        let mut send = move |cmd: NetCmd| {
            let _ = cmd_tx.send(cmd);
        };

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(&st.status_line);
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Screens");
                ui.label(format!("Screen: {:?}", st.screen));
                ui.separator();

                match st.screen {
                    ui::Screen::Connect => {
                        crate::ui::screens::connect::ui_connect(ui, st, &mut send);
                    }
                    ui::Screen::Lobby => {
                        crate::ui::screens::lobby::ui_lobby(ui, st, &mut send);
                    }
                    ui::Screen::Game => {
                        ui.label("Game is shown in center panel.");
                        if ui.button("Disconnect").clicked() {
                            send(NetCmd::Disconnect);
                            st.on_disconnected();
                        }
                    }
                }

                ui.separator();
                ui.heading("Log");

                ui.push_id("log_component", |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("log_scroll")
                        .max_height(250.0)
                        .show(ui, |ui| {
                            for line in st.log.iter().rev() {
                                ui.label(line);
                            }
                        });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| match st.screen {
            ui::Screen::Game => {
                crate::ui::screens::game::ui_game(ui, st, &mut send);
            }
            _ => {
                ui.heading("Trap The Mouse");
                ui.label("Connect first, then join a room.");
            }
        });

        crate::ui::popups::ui_popups(ctx, st, &mut send);

        ctx.request_repaint();
    }
}
