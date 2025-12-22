use crate::hex::{inside_board, is_border, neighbors};
use crate::types::{Coord, GameState};
use std::collections::{HashMap, VecDeque};

pub fn choose_mouse_move(s: &GameState) -> Option<Coord> {
    let start = s.mouse;
    let mut q = VecDeque::new();
    let mut prev: HashMap<Coord, Coord> = HashMap::new();

    q.push_back(start);
    prev.insert(start, start);

    let mut target: Option<Coord> = None;

    while let Some(cur) = q.pop_front() {
        if cur != start && is_border(cur, s.cfg.radius) {
            target = Some(cur);
            break;
        }
        for n in neighbors(cur) {
            if !inside_board(n, s.cfg.radius) {
                continue;
            }
            if s.blocks.contains(&n) {
                continue;
            }
            if prev.contains_key(&n) {
                continue;
            }
            prev.insert(n, cur);
            q.push_back(n);
        }
    }

    if let Some(t) = target {
        let mut cur = t;
        while let Some(p) = prev.get(&cur).copied() {
            if p == start {
                return Some(cur);
            }
            if p == cur {
                break;
            }
            cur = p;
        }
    }

    neighbors(start)
        .into_iter()
        .find(|n| inside_board(*n, s.cfg.radius) && !s.blocks.contains(n))
}
