use crate::room::manager::RoomManager;

#[derive(Clone)]
pub struct AppState {
    pub manager: RoomManager,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            manager: RoomManager::new(),
        }
    }
}
