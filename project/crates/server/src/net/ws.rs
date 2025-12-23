use axum::
{
    extract::
    {
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};

use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;

use shared::net::{ClientMsg, ServerMsg};

use crate::app::AppState;

pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse
{
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}

async fn handle_socket(state: AppState, socket: WebSocket)
{
    let client_id = Uuid::new_v4();

    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<ServerMsg>();
    let (mut ws_tx, mut ws_rx) = socket.split();

    let sender = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            let Ok(text) = serde_json::to_string(&msg) else {
                continue;
            };
            if ws_tx.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    });

    {
        let rooms = state.manager.list_rooms().await;
        let _ = out_tx.send(ServerMsg::RoomList { rooms });
    }

    let mut current_room: Option<String> = None;

    while let Some(Ok(msg)) = ws_rx.next().await {
        match msg {
            Message::Text(text) => {
                let parsed = serde_json::from_str::<ClientMsg>(&text);
                let Ok(cmd) = parsed else {
                    let _ = out_tx.send(ServerMsg::Error {
                        message: "Invalid JSON or message shape".to_string(),
                    });
                    continue;
                };

                match cmd {
                    ClientMsg::ListRooms => {
                        let rooms = state.manager.list_rooms().await;
                        let _ = out_tx.send(ServerMsg::RoomList { rooms });
                    }

                    ClientMsg::CreateRoom { name, vs_bot } => {
                        if let Some(r) = current_room.take() {
                            let _ = state.manager.leave_room(&r, client_id).await;
                        }

                        let info = state.manager.create_room(name, vs_bot).await;
                        let room_id = info.room_id.clone();

                        match state
                            .manager
                            .join_room(&room_id, client_id, out_tx.clone())
                            .await
                        {
                            Ok(()) => current_room = Some(room_id),
                            Err(e) => {
                                let _ = out_tx.send(ServerMsg::Error { message: e });
                            }
                        }
                    }

                    ClientMsg::JoinRoom { room_id } => {
                        if let Some(r) = current_room.take() {
                            let _ = state.manager.leave_room(&r, client_id).await;
                        }

                        match state
                            .manager
                            .join_room(&room_id, client_id, out_tx.clone())
                            .await
                        {
                            Ok(()) => current_room = Some(room_id),
                            Err(e) => {
                                let _ = out_tx.send(ServerMsg::Error { message: e });
                            }
                        }
                    }

                    ClientMsg::LeaveRoom => {
                        if let Some(r) = current_room.take() {
                            let _ = state.manager.leave_room(&r, client_id).await;
                        }
                    }

                    ClientMsg::PlayerAction { action } => {
                        let Some(r) = current_room.as_deref() else {
                            let _ = out_tx.send(ServerMsg::Error {
                                message: "Not in a room".to_string(),
                            });
                            continue;
                        };

                        if let Err(e) = state.manager.player_action(r, client_id, action).await {
                            let _ = out_tx.send(ServerMsg::Error { message: e });
                        }
                    }
                }
            }

            Message::Close(_) => break,
            _ => {}
        }
    }


    if let Some(r) = current_room {
        let _ = state.manager.leave_room(&r, client_id).await;
    }

    drop(out_tx);
    let _ = sender.await;
}
