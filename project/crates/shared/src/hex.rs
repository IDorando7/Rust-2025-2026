use crate::types::Coord;

pub fn neighbors(c: Coord) -> [Coord; 6] {
    let (q, r) = (c.q, c.r);
    [
        Coord { q: q + 1, r },
        Coord { q: q - 1, r },
        Coord { q, r: r + 1 },
        Coord { q, r: r - 1 },
        Coord { q: q + 1, r: r - 1 },
        Coord { q: q - 1, r: r + 1 },
    ]
}

pub fn hex_distance(a: Coord, b: Coord) -> i32 {
    let ax = a.q;
    let az = a.r;
    let ay = -ax - az;
    let bx = b.q;
    let bz = b.r;
    let by = -bx - bz;
    ((ax - bx).abs().max((ay - by).abs())).max((az - bz).abs())
}

pub fn inside_board(c: Coord, radius: i32) -> bool {
    hex_distance(Coord { q: 0, r: 0 }, c) <= radius
}

pub fn is_border(c: Coord, radius: i32) -> bool {
    hex_distance(Coord { q: 0, r: 0 }, c) == radius
}
