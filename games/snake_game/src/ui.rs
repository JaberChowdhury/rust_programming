use crate::constants::*;
use macroquad::prelude::*;
#[derive(PartialEq)]
pub enum CamMode {
    Follow,
    FullMap,
}

pub fn apply_camera(mode: &CamMode, target: Vec2, sw: f32, sh: f32) -> Camera2D {
    let cam = match mode {
        CamMode::FullMap => Camera2D {
            target: vec2(ARENA_W / 2.0, ARENA_H / 2.0),
            // Zoom to fit the entire arena on the screen (maintain aspect ratio if needed, but here we just map it)
            // It's better to fit the larger dimension to avoid clipping
            zoom: vec2(2.0 / ARENA_W, -2.0 / ARENA_H),
            ..Default::default()
        },
        CamMode::Follow => {
            let hw = sw / (2.0 * FOLLOW_ZOOM);
            let hh = sh / (2.0 * FOLLOW_ZOOM);
            Camera2D {
                target: vec2(
                    target.x.clamp(hw, ARENA_W - hw),
                    target.y.clamp(hh, ARENA_H - hh),
                ),
                zoom: vec2(2.0 * FOLLOW_ZOOM / sw, -2.0 * FOLLOW_ZOOM / sh),
                ..Default::default()
            }
        }
    };
    set_camera(&cam);
    cam
}

// ── UI helpers ────────────────────────────────────────────────────────────────
pub fn tp<'a>(font: &'a Font, size: u16, col: Color) -> TextParams<'a> {
    TextParams {
        font: Some(font),
        font_size: size,
        color: col,
        ..Default::default()
    }
}

pub fn draw_glass_rect(x: f32, y: f32, w: f32, h: f32, alpha: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.07, 0.09, 0.15, alpha));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.35, 0.45, 0.80, 0.35));
}

pub fn centered_text(text: &str, font: &Font, size: u16, col: Color, rect: Rect) {
    let d = measure_text(text, Some(font), size, 1.0);
    let tx = rect.x + (rect.w - d.width) / 2.0;
    let ty = rect.y + (rect.h + d.height) / 2.0;
    draw_text_ex(text, tx, ty, tp(font, size, col));
}
