use std::thread;

use crossbeam_channel::{Receiver, Sender};
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

use futures_util::{SinkExt, StreamExt};

use shared::net::{ClientMsg, ServerMsg};

#[derive(Debug, Clone)]
pub enum NetCmd {
    Connect { url: String },
    Disconnect,
    Send(ClientMsg),
}

#[derive(Debug, Clone)]
pub enum NetEvent {
    Connected,
    Disconnected,
    Server(ServerMsg),
    Error(String),
}

pub fn start_ws_worker() -> (Sender<NetCmd>, Receiver<NetEvent>) {
    let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded::<NetCmd>();
    let (event_tx, event_rx) = crossbeam_channel::unbounded::<NetEvent>();

    thread::spawn(move || {
        let rt = Builder::new_multi_thread().enable_all().build();
        let rt = match rt {
            Ok(rt) => rt,
            Err(e) => {
                let _ = event_tx.send(NetEvent::Error(format!("Tokio runtime build failed: {e}")));
                return;
            }
        };

        let (async_cmd_tx, async_cmd_rx) = mpsc::unbounded_channel::<NetCmd>();

        let bridge_handle = thread::spawn(move || {
            while let Ok(cmd) = cmd_rx.recv() {
                if async_cmd_tx.send(cmd).is_err() {
                    break;
                }
            }
        });

        rt.block_on(async move {
            ws_main_loop(async_cmd_rx, event_tx).await;
        });

        let _ = bridge_handle.join();
    });

    (cmd_tx, event_rx)
}

type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

struct WsConn {
    write: futures_util::stream::SplitSink<WsStream, Message>,
    read: futures_util::stream::SplitStream<WsStream>,
}

impl WsConn {
    fn new(stream: WsStream) -> Self {
        let (write, read) = stream.split();
        Self { write, read }
    }

    async fn send_client_msg(&mut self, msg: ClientMsg) -> Result<(), String> {
        let text =
            serde_json::to_string(&msg).map_err(|e| format!("Serialize ClientMsg failed: {e}"))?;
        self.write
            .send(Message::Text(text))
            .await
            .map_err(|e| format!("WebSocket send failed: {e}"))?;
        Ok(())
    }

    async fn close(&mut self) -> Result<(), String> {
        self.write
            .send(Message::Close(None))
            .await
            .map_err(|e| format!("WebSocket close failed: {e}"))?;
        Ok(())
    }

    async fn read_one(&mut self) -> WsRead {
        match self.read.next().await {
            Some(Ok(Message::Text(t))) => WsRead::Text(t),
            Some(Ok(Message::Close(_))) => WsRead::Closed,
            Some(Ok(_)) => WsRead::Ignored,
            Some(Err(e)) => WsRead::Error(e.to_string()),
            None => WsRead::Closed,
        }
    }
}

enum WsRead {
    Text(String),
    Closed,
    Ignored,
    Error(String),
}

async fn ws_main_loop(mut cmd_rx: mpsc::UnboundedReceiver<NetCmd>, event_tx: Sender<NetEvent>) {
    let mut conn: Option<WsConn> = None;

    loop {
        if let Some(c) = conn.as_mut() {
            tokio::select! {
                cmd = cmd_rx.recv() => {
                    let Some(cmd) = cmd else {
                        let _ = c.close().await;
                        let _ = event_tx.send(NetEvent::Disconnected);
                        return;
                    };
                    handle_cmd(cmd, &event_tx, &mut conn).await;
                }
                incoming = c.read_one() => {
                    match incoming {
                        WsRead::Text(text) => {
                            match serde_json::from_str::<ServerMsg>(&text) {
                                Ok(msg) => { let _ = event_tx.send(NetEvent::Server(msg)); }
                                Err(e) => { let _ = event_tx.send(NetEvent::Error(format!("Bad ServerMsg JSON: {e}"))); }
                            }
                        }
                        WsRead::Closed => {
                            let _ = event_tx.send(NetEvent::Disconnected);
                            conn = None;
                        }
                        WsRead::Ignored => {}
                        WsRead::Error(e) => {
                            let _ = event_tx.send(NetEvent::Error(format!("WebSocket read error: {e}")));
                            let _ = event_tx.send(NetEvent::Disconnected);
                            conn = None;
                        }
                    }
                }
            }
        } else {
            let cmd = cmd_rx.recv().await;
            let Some(cmd) = cmd else {
                return;
            };
            handle_cmd(cmd, &event_tx, &mut conn).await;
        }
    }
}

async fn handle_cmd(cmd: NetCmd, event_tx: &Sender<NetEvent>, conn: &mut Option<WsConn>) {
    match cmd {
        NetCmd::Connect { url } => {
            if let Some(c) = conn.as_mut() {
                let _ = c.close().await;
                *conn = None;
            }

            let url = match Url::parse(&url) {
                Ok(u) => u,
                Err(e) => {
                    let _ = event_tx.send(NetEvent::Error(format!("Invalid URL: {e}")));
                    return;
                }
            };

            let url_str = url.to_string();

            match connect_async(url_str).await {
                Ok((stream, _)) => {
                    *conn = Some(WsConn::new(stream));
                    let _ = event_tx.send(NetEvent::Connected);

                    if let Some(c) = conn.as_mut() {
                        let _ = c.send_client_msg(ClientMsg::ListRooms).await;
                    }
                }
                Err(e) => {
                    let _ = event_tx.send(NetEvent::Error(format!("Connect failed: {e}")));
                    let _ = event_tx.send(NetEvent::Disconnected);
                }
            }
        }

        NetCmd::Disconnect => {
            if let Some(c) = conn.as_mut() {
                let _ = c.close().await;
                *conn = None;
                let _ = event_tx.send(NetEvent::Disconnected);
            }
        }

        NetCmd::Send(msg) => {
            let Some(c) = conn.as_mut() else {
                let _ = event_tx.send(NetEvent::Error("Not connected".to_string()));
                return;
            };

            if let Err(e) = c.send_client_msg(msg).await {
                let _ = event_tx.send(NetEvent::Error(e));
                let _ = event_tx.send(NetEvent::Disconnected);
                *conn = None;
            }
        }
    }
}
