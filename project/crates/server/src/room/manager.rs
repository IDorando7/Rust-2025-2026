use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use shared::net::RoomInfo;
use shared::types::Action;

use crate::room::actor::{spawn_room, RoomCmd, RoomHandle};

#[derive(Clone)]
pub struct RoomManager {
    rooms: Arc<RwLock<HashMap<String, RoomHandle>>>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn list_rooms(&self) -> Vec<RoomInfo> {
        let rooms = self.rooms.read().await;
        rooms
            .values()
            .map(|h| {
                let snap = h.snapshot_rx.borrow().clone();
                RoomInfo {
                    room_id: h.room_id.clone(),
                    name: h.name.clone(),
                    players: snap.players,
                    vs_bot: h.vs_bot,
                }
            })
            .collect()
    }

    pub async fn create_room(&self, name: String, vs_bot: bool) -> RoomInfo {
        let room_id = Uuid::new_v4().to_string();
        let handle = spawn_room(room_id.clone(), name.clone(), vs_bot);

        {
            let mut rooms = self.rooms.write().await;
            rooms.insert(room_id.clone(), handle);
        }

        RoomInfo {
            room_id,
            name,
            players: 0,
            vs_bot,
        }
    }

    pub async fn join_room(
        &self,
        room_id: &str,
        client_id: Uuid,
        client_tx: tokio::sync::mpsc::UnboundedSender<shared::net::ServerMsg>,
    ) -> Result<(), String> {
        let handle = {
            let rooms = self.rooms.read().await;
            rooms
                .get(room_id)
                .cloned()
                .ok_or_else(|| "Room not found".to_string())?
        };

        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
        handle
            .cmd_tx
            .send(RoomCmd::Join {
                client_id,
                client_tx,
                reply: reply_tx,
            })
            .map_err(|_| "Room task is dead".to_string())?;

        reply_rx
            .await
            .map_err(|_| "Room did not reply".to_string())?
    }

    pub async fn leave_room(&self, room_id: &str, client_id: Uuid) -> Result<(), String> {
        let handle = {
            let rooms = self.rooms.read().await;
            rooms
                .get(room_id)
                .cloned()
                .ok_or_else(|| "Room not found".to_string())?
        };

        handle
            .cmd_tx
            .send(RoomCmd::Leave { client_id })
            .map_err(|_| "Room task is dead".to_string())?;

        Ok(())
    }

    pub async fn player_action(
        &self,
        room_id: &str,
        client_id: Uuid,
        action: Action,
    ) -> Result<(), String> {
        let handle = {
            let rooms = self.rooms.read().await;
            rooms
                .get(room_id)
                .cloned()
                .ok_or_else(|| "Room not found".to_string())?
        };

        handle
            .cmd_tx
            .send(RoomCmd::Action { client_id, action })
            .map_err(|_| "Room task is dead".to_string())?;

        Ok(())
    }
}
