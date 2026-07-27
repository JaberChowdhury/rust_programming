use crate::constants::*;
use macroquad::prelude::*;

// ── Spatial grid ──────────────────────────────────────────────────────────────
pub struct SpatialGrid {
    pub cells: Vec<Vec<(u16, u16)>>,
    pub cols: usize,
    pub rows: usize,
    pub cell: f32,
}
impl SpatialGrid {
    pub fn new(width: f32, height: f32) -> Self {
        let cols = (width / GRID_CELL).ceil() as usize + 1;
        let rows = (height / GRID_CELL).ceil() as usize + 1;
        Self {
            cells: vec![Vec::new(); cols * rows],
            cols,
            rows,
            cell: GRID_CELL,
        }
    }
    #[inline]
    pub fn idx(&self, cx: usize, cy: usize) -> usize {
        cy * self.cols + cx
    }
    #[inline]
    pub fn cell_of(&self, p: Vec2) -> (usize, usize) {
        (
            ((p.x / self.cell) as usize).min(self.cols - 1),
            ((p.y / self.cell) as usize).min(self.rows - 1),
        )
    }
    pub fn insert(&mut self, pos: Vec2, si: usize, ki: usize) {
        let (cx, cy) = self.cell_of(pos);
        let idx = self.idx(cx, cy);
        self.cells[idx].push((si as u16, ki as u16));
    }
    pub fn query_radius<'a>(&'a self, pos: Vec2, r: f32) -> impl Iterator<Item = (u16, u16)> + 'a {
        let rc = (r / self.cell).ceil() as i32 + 1;
        let (cx, cy) = self.cell_of(pos);
        let (cx, cy) = (cx as i32, cy as i32);
        let (cols, rows) = (self.cols as i32, self.rows as i32);
        (cy - rc..=cy + rc)
            .filter(move |&ry| ry >= 0 && ry < rows)
            .flat_map(move |ry| {
                (cx - rc..=cx + rc)
                    .filter(move |&rx| rx >= 0 && rx < cols)
                    .flat_map(move |rx| {
                        self.cells[self.idx(rx as usize, ry as usize)]
                            .iter()
                            .copied()
                    })
            })
    }
    pub fn crowding(&self, pos: Vec2, r: f32) -> usize {
        self.query_radius(pos, r).count()
    }
}

// ── Food grid (clustering) ────────────────────────────────────────────────────
pub struct FoodGrid {
    pub cells: Vec<u16>,
    pub cols: usize,
    pub rows: usize,
    pub cell: f32,
}
impl FoodGrid {
    pub fn new(w: f32, h: f32) -> Self {
        let c = CLUSTER_BONUS_RADIUS / 2.0;
        let cols = (w / c).ceil() as usize + 1;
        let rows = (h / c).ceil() as usize + 1;
        Self {
            cells: vec![0; cols * rows],
            cols,
            rows,
            cell: c,
        }
    }
    pub fn insert(&mut self, pos: Vec2) {
        let cx = ((pos.x / self.cell) as usize).min(self.cols - 1);
        let cy = ((pos.y / self.cell) as usize).min(self.rows - 1);
        self.cells[cy * self.cols + cx] = self.cells[cy * self.cols + cx].saturating_add(1);
    }
    pub fn count_nearby(&self, pos: Vec2, r: f32) -> u32 {
        let rc = (r / self.cell).ceil() as i32 + 1;
        let cx = ((pos.x / self.cell) as i32).clamp(0, self.cols as i32 - 1);
        let cy = ((pos.y / self.cell) as i32).clamp(0, self.rows as i32 - 1);
        let mut tot = 0u32;
        for ry in (cy - rc).max(0)..=(cy + rc).min(self.rows as i32 - 1) {
            for rx in (cx - rc).max(0)..=(cx + rc).min(self.cols as i32 - 1) {
                tot += self.cells[ry as usize * self.cols + rx as usize] as u32;
            }
        }
        tot
    }
}
