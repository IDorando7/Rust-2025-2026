use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coord {
    pub q: i32,
    pub r: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Turn {
    Trapper,
    Mouse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    Running,
    TrapperWon,
    MouseWon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardConfig {
    pub radius: i32,
    pub initial_blocks: usize,
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub cfg: BoardConfig,
    pub mouse: Coord,
    pub blocks: std::collections::HashSet<Coord>,
    pub turn: Turn,
    pub status: GameStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    PlaceBlock { at: Coord },
    MoveMouse { to: Coord },
}
