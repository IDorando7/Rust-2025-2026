use crate::types::{Action, GameState, Turn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomInfo {
    pub room_id: String,
    pub name: String,
    pub players: u8,
    pub vs_bot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMsg {
    CreateRoom { name: String, vs_bot: bool },
    JoinRoom { room_id: String },
    LeaveRoom,
    PlayerAction { action: Action },
    ListRooms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMsg {
    RoomList {
        rooms: Vec<RoomInfo>,
    },
    LobbyState {
        room_id: String,
        players: u8,
        vs_bot: bool,
    },
    GameStart {
        state: GameState,
        your_role: Turn,
    },
    GameUpdate {
        state: GameState,
    },
    Error {
        message: String,
    },
}
