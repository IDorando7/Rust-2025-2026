use crate::hex::{inside_board, is_border, neighbors};
use crate::types::{Action, GameState, GameStatus, Turn};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("game already ended")]
    GameEnded,
    #[error("wrong turn")]
    WrongTurn,
    #[error("cell not on board")]
    OutsideBoard,
    #[error("cell is blocked")]
    Blocked,
    #[error("cannot block mouse cell")]
    BlockMouse,
    #[error("mouse move must be to a neighbor")]
    NotNeighbor,
}

fn mouse_has_legal_moves(s: &GameState) -> bool {
    neighbors(s.mouse)
        .into_iter()
        .any(|n| inside_board(n, s.cfg.radius) && !s.blocks.contains(&n))
}

pub fn apply_action(mut s: GameState, a: Action) -> Result<GameState, GameError> {
    if s.status != GameStatus::Running {
        return Err(GameError::GameEnded);
    }

    match a {
        Action::PlaceBlock { at } => {
            if s.turn != Turn::Trapper {
                return Err(GameError::WrongTurn);
            }
            if !inside_board(at, s.cfg.radius) {
                return Err(GameError::OutsideBoard);
            }
            if s.blocks.contains(&at) {
                return Err(GameError::Blocked);
            }
            if at == s.mouse {
                return Err(GameError::BlockMouse);
            }
            s.blocks.insert(at);
            s.turn = Turn::Mouse;
        }
        Action::MoveMouse { to } => {
            if s.turn != Turn::Mouse {
                return Err(GameError::WrongTurn);
            }
            if !inside_board(to, s.cfg.radius) {
                return Err(GameError::OutsideBoard);
            }
            if s.blocks.contains(&to) {
                return Err(GameError::Blocked);
            }
            if !neighbors(s.mouse).contains(&to) {
                return Err(GameError::NotNeighbor);
            }
            s.mouse = to;

            if is_border(s.mouse, s.cfg.radius) {
                s.status = GameStatus::MouseWon;
            } else if !mouse_has_legal_moves(&s) {
                s.status = GameStatus::TrapperWon;
            } else {
                s.turn = Turn::Trapper;
            }
        }
    }

    Ok(s)
}
