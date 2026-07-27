use macroquad::prelude::*;
// use macroquad::rand::gen_range;

// Ball physics properties
const GRAVITY: f32 = 1.0;
const BOUNCE_DAMPING: f32 = 1.0;
const FRICTION: f32 = 1.0;
const MIN_BOUNCE_VELOCITY: f32 = 50.0;

struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    color: Color,
}
// fn random_color() -> Color {
//     Color::from_rgba(
//         gen_range(0, 255), // Red
//         gen_range(0, 255), // Green
//         gen_range(0, 255), // Blue
//         255,               // Alpha (fully opaque)
//     )
// }
impl Ball {
    fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            pos: vec2(x, y),
            vel: vec2(
                rand::gen_range(-300.0, 300.0),
                rand::gen_range(-500.0, -100.0),
            ),
            radius: rand::gen_range(5.0, 5.0),
            color: color,
        }
    }

    fn update(&mut self, dt: f32) {
        // Apply gravity
        self.vel.y += GRAVITY * dt;
        self.vel.x *= FRICTION;

        // Update position
        self.pos += self.vel * dt;

        // Floor bounce
        if self.pos.y + self.radius > screen_height() {
            self.pos.y = screen_height() - self.radius;
            if self.vel.y.abs() > MIN_BOUNCE_VELOCITY {
                self.vel.y *= -BOUNCE_DAMPING;
            } else {
                self.vel.y = 0.0;
            }
        }

        // Ceiling bounce
        if self.pos.y - self.radius < 0.0 {
            self.pos.y = self.radius;
            self.vel.y *= -BOUNCE_DAMPING;
        }

        // Wall bounces
        if self.pos.x + self.radius > screen_width() {
            self.pos.x = screen_width() - self.radius;
            self.vel.x *= -BOUNCE_DAMPING;
        }
        if self.pos.x - self.radius < 0.0 {
            self.pos.x = self.radius;
            self.vel.x *= -BOUNCE_DAMPING;
        }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
        // Add a subtle highlight for depth
        draw_circle(
            self.pos.x - self.radius * 0.3,
            self.pos.y - self.radius * 0.3,
            self.radius * 0.25,
            Color::from_rgba(255, 255, 255, 120),
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Bouncing Balls".to_string(),
        window_width: 3560,
        window_height: 2000,
        window_resizable: false,
        ..Default::default()
    }
}

fn generate_ball() -> Vec<Ball> {
    let mut balls: Vec<Ball> = vec![];
    for i in 1..50 {
        balls.push(Ball::new(
            rand::gen_range(50.0, screen_width() - 50.0) + i as f32,
            50.0 * i as f32,
            WHITE,
        ));
    }
    for i in 1..50 {
        balls.push(Ball::new(
            rand::gen_range(50.0, screen_width() - 50.0) + i as f32,
            50.0 * i as f32,
            BLACK,
        ));
    }
    balls
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut balls: Vec<Ball> = generate_ball();
    let mut line = true;

    loop {
        let dt = get_frame_time() * 0.5;

        // Sage green background
        clear_background(Color::from_rgba(204, 213, 174, 255));

        // Spawn ball on click
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            balls.push(Ball::new(mx, my, WHITE));
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            let new_collection = generate_ball();
            for nb in new_collection {
                balls.push(nb);
            }
        }

        // Spawn ball on spacebar
        if is_key_pressed(KeyCode::Space) {
            balls.push(Ball::new(
                rand::gen_range(50.0, screen_width() - 50.0),
                50.0,
                BLACK,
            ));
        }

        // Clear all with C
        if is_key_pressed(KeyCode::C) {
            balls.clear();
        }
        if is_key_pressed(KeyCode::L) {
            line = !line;
        }

        // Update and draw balls
        for ball in balls.iter_mut() {
            ball.update(dt);
            ball.draw();
        }

        let total_balls = balls.len();
        for i in 0..total_balls {
            let (left, right) = balls.split_at_mut(i + 1);
            let first_ball = &mut left[i];
            for ball in right {
                let dist = ball.pos.distance(first_ball.pos);
                // Check for overlap, not exact zero
                if dist < 10.0 && dist > 0.0 {
                    // Optional: Normalize and separate them to prevent sticking
                    let overlap = 10.0 - dist;
                    let normal = (ball.pos - first_ball.pos).normalize();

                    ball.pos += normal * (overlap / 2.0);
                    first_ball.pos -= normal * (overlap / 2.0);

                    // Swap velocities (elastic collision approximation)
                    std::mem::swap(&mut ball.vel, &mut first_ball.vel);
                } else if dist <= 100.0 {
                    if line {
                        draw_line(
                            ball.pos[0],
                            ball.pos[1],
                            first_ball.pos[0],
                            first_ball.pos[1],
                            0.30,
                            ball.color,
                        );
                        // std::mem::swap(&mut first_ball.color, &mut ball.color);
                    }
                }
            }
        }

        // UI text
        draw_text(
            &format!(
                "Balls: {} | Click to spawn | Space = random | C = clear | L = Line toggle ",
                balls.len(),
            ),
            15.0,
            30.0,
            24.0,
            DARKGRAY,
        );

        next_frame().await;
    }
}
