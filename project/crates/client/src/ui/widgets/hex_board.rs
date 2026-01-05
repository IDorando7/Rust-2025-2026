use eframe::egui;

use shared::hex::{inside_board, neighbors};
use shared::types::{Coord, GameState, GameStatus, Turn};

pub struct HexBoard {
    pub hex_size: f32,
    pub use_hex_view: bool,
}

pub struct BoardClickPolicy<'a> {
    pub your_role: Turn,
    pub current_turn: Turn,
    pub status: GameStatus,
    pub mouse: Coord,
    pub blocks: &'a std::collections::HashSet<Coord>,
    pub radius: i32,
}

impl<'a> BoardClickPolicy<'a> {
    pub fn is_clickable(&self, c: Coord) -> bool {
        if self.status != GameStatus::Running {
            return false;
        }
        if self.your_role != self.current_turn {
            return false;
        }
        if !inside_board(c, self.radius) {
            return false;
        }
        if self.blocks.contains(&c) {
            return false;
        }

        match self.your_role {
            Turn::Trapper => c != self.mouse,
            Turn::Mouse => neighbors(self.mouse).contains(&c),
        }
    }

    pub fn is_mouse(&self, c: Coord) -> bool {
        c == self.mouse
    }

    pub fn is_block(&self, c: Coord) -> bool {
        self.blocks.contains(&c)
    }
}

impl HexBoard {
    pub fn draw(
        &mut self,
        ui: &mut egui::Ui,
        gs: &GameState,
        policy: &BoardClickPolicy<'_>,
    ) -> Option<Coord> {
        if self.use_hex_view {
            self.draw_hex(ui, gs, policy)
        } else {
            self.draw_table(ui, gs, policy)
        }
    }

    fn draw_table(
        &mut self,
        ui: &mut egui::Ui,
        gs: &GameState,
        policy: &BoardClickPolicy<'_>,
    ) -> Option<Coord> {
        let mut clicked: Option<Coord> = None;
        let rmax = gs.cfg.radius;

        let cell = egui::vec2(44.0, 24.0);
        let visuals = ui.visuals().clone();

        for r in (-rmax..=rmax).rev() {
            let qmin = (-rmax).max(-rmax - r);
            let qmax = rmax.min(rmax - r);

            ui.horizontal(|ui| {
                let indent = (qmin - (-rmax)) as f32 * (cell.x * 0.5);
                if indent > 0.0 {
                    ui.add_space(indent);
                }

                for q in qmin..=qmax {
                    let c = Coord { q, r };

                    let mut fill = visuals.widgets.inactive.bg_fill;
                    if policy.is_block(c) {
                        fill = visuals.widgets.noninteractive.bg_fill;
                    }
                    if policy.is_mouse(c) {
                        fill = visuals.selection.bg_fill;
                    }

                    if policy.is_clickable(c) {
                        fill = visuals.widgets.active.bg_fill;
                    }

                    let label = if policy.is_mouse(c) {
                        "M".to_string()
                    } else if policy.is_block(c) {
                        "X".to_string()
                    } else {
                        ".".to_string()
                    };

                    let btn = egui::Button::new(label).fill(fill);
                    let resp = ui.add_sized(cell, btn);

                    if resp.clicked() && policy.is_clickable(c) {
                        clicked = Some(c);
                    }
                }
            });
        }

        clicked
    }

    fn draw_hex(
        &mut self,
        ui: &mut egui::Ui,
        gs: &GameState,
        policy: &BoardClickPolicy<'_>,
    ) -> Option<Coord> {
        let size = self.hex_size.max(6.0);
        let coords = all_coords(gs.cfg.radius);

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for &c in &coords {
            let p = axial_to_pixel(c, size);
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }

        let pad = size * 1.2;
        let w = (max_x - min_x) + pad * 2.0;
        let h = (max_y - min_y) + pad * 2.0;

        let (rect, _) = ui.allocate_exact_size(egui::vec2(w, h), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        let visuals = ui.visuals().clone();

        let click_pos = ui.input(|i| {
            if i.pointer.primary_clicked() {
                i.pointer.interact_pos()
            } else {
                None
            }
        });

        let hover_pos = ui.input(|i| i.pointer.hover_pos());

        let center_model_x = (min_x + max_x) * 0.5;
        let center_model_y = (min_y + max_y) * 0.5;
        let origin = rect.center() - egui::vec2(center_model_x, center_model_y);

        let mut clicked_cell: Option<Coord> = None;

        for &c in &coords {
            let center = axial_to_pixel(c, size) + origin.to_vec2();
            let poly = hex_polygon(center, size);

            let is_hover = hover_pos.is_some_and(|hp| point_in_poly(hp, &poly));
            let is_clickable = policy.is_clickable(c);

            let mut fill = visuals.widgets.inactive.bg_fill;
            if policy.is_block(c) {
                fill = visuals.widgets.noninteractive.bg_fill;
            }
            if policy.is_mouse(c) {
                fill = visuals.selection.bg_fill;
            } else if is_clickable {
                fill = visuals.widgets.active.bg_fill;
            }

            let mut stroke = visuals.widgets.inactive.fg_stroke;
            if is_clickable {
                stroke = visuals.widgets.active.fg_stroke;
                stroke.width = 2.2;
            }
            if is_hover && is_clickable {
                stroke.width = 3.0;
            }
            if policy.is_mouse(c) {
                stroke = visuals.selection.stroke;
                stroke.width = stroke.width.max(2.6);
            }

            painter.add(egui::Shape::convex_polygon(poly.to_vec(), fill, stroke));

            if clicked_cell.is_none() {
                if let Some(cp) = click_pos {
                    if point_in_poly(cp, &poly) && is_clickable {
                        clicked_cell = Some(c);
                    }
                }
            }
        }

        clicked_cell
    }
}

fn all_coords(radius: i32) -> Vec<Coord> {
    let mut v = Vec::new();
    for q in -radius..=radius {
        for r in -radius..=radius {
            let c = Coord { q, r };
            if inside_board(c, radius) {
                v.push(c);
            }
        }
    }
    v.sort_by_key(|c| (-(c.r), c.q));
    v
}

fn axial_to_pixel(c: Coord, size: f32) -> egui::Pos2 {
    const SQRT_3: f32 = 1.732_050_8;
    let q = c.q as f32;
    let r = c.r as f32;

    egui::pos2(size * (SQRT_3 * q + (SQRT_3 * 0.5) * r), size * (1.5 * r))
}

fn hex_polygon(center: egui::Pos2, size: f32) -> [egui::Pos2; 6] {
    let mut pts = [egui::Pos2::ZERO; 6];
    for (i, pt) in pts.iter_mut().enumerate() {
        let ang_deg = 60.0 * (i as f32) - 30.0;
        let ang = ang_deg.to_radians();
        let x = center.x + size * ang.cos();
        let y = center.y + size * ang.sin();
        *pt = egui::pos2(x, y);
    }
    pts
}

fn point_in_poly(p: egui::Pos2, poly: &[egui::Pos2; 6]) -> bool {
    let mut inside = false;
    let mut j = poly.len() - 1;

    for i in 0..poly.len() {
        let pi = poly[i];
        let pj = poly[j];

        let intersect = ((pi.y > p.y) != (pj.y > p.y))
            && (p.x < (pj.x - pi.x) * (p.y - pi.y) / ((pj.y - pi.y).max(1e-6)) + pi.x);

        if intersect {
            inside = !inside;
        }
        j = i;
    }

    inside
}
