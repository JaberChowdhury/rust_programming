use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Chain".to_string(),
        window_width: 3800,
        window_height: 1900,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

struct Circle {
    pos: Vec2,
    radius: f32,
    color: Color,
}

impl Circle {
    fn new(x: f32, y: f32, rad: f32, color: Color) -> Self {
        Self {
            pos: vec2(x, y),
            radius: rad,
            color: color,
        }
    }

    fn draw(&self) {
        draw_circle_lines(self.pos.x, self.pos.y, self.radius, 2.0, BLACK);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut circles: Vec<Circle> = vec![];

    let total_segments = 400;
    let segment_length = 10.0; // Distance between circles
    let radius = 5.0;

    // Initialize in a vertical line
    for i in 0..total_segments {
        circles.push(Circle::new(
            screen_width() / 2.0,
            100.0 + i as f32 * segment_length,
            radius,
            if i == 0 { RED } else { WHITE }, // Head is red
        ));
    }

    loop {
        clear_background(Color::from_rgba(204, 213, 174, 255));

        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);

        // 1. Move Head to Mouse
        circles[0].pos = mouse_pos;

        // 2. Move Body Segments (Inverse Kinematics / Drag Follow)
        for i in 1..circles.len() {
            let prev = circles[i - 1].pos;
            let curr = circles[i].pos;

            // Vector from current to previous
            let mut dir = curr - prev;
            let dist = dir.length();

            // Only move if we are further away than the segment length
            // (or if we are too close, this logic pulls them apart to maintain length)
            if dist > 0.1 {
                dir = dir.normalize();

                // Target position is exactly 'segment_length' away from previous
                let target_pos = prev + dir * segment_length;

                // Smoothly interpolate or snap to target
                // Snapping (perfect IK):
                circles[i].pos = target_pos;

                // Optional: Smooth lerp for "looser" rope feel
                // circles[i].pos = curr.lerp(target_pos, 0.5);
            }
        }

        // 3. Draw Connections
        for i in 0..circles.len() - 1 {
            draw_line(
                circles[i].pos.x,
                circles[i].pos.y,
                circles[i + 1].pos.x,
                circles[i + 1].pos.y,
                4.0,
                BLACK,
            );
        }

        // 4. Draw Circles
        for c in &circles {
            c.draw();
        }

        next_frame().await;
    }
}
