// use macroquad::prelude::*;

// const MAX_PARTICLES: usize = 200;
// const TRAIL_LENGTH: usize = 15;
// const CONNECTION_DIST: f32 = 150.0;
// const GLOW_RADIUS_MULT: f32 = 4.0;

// #[derive(Clone, Copy, PartialEq)]
// enum PhysicsMode {
//     Gravity,
//     Orbit,
//     Chaos,
//     ZeroG,
// }

// struct Particle {
//     pos: Vec2,
//     vel: Vec2,
//     radius: f32,
//     base_radius: f32,
//     color: Color,
//     trail: Vec<Vec2>,
//     pulse: f32,
//     heat: f32,
//     mass: f32,
// }

// impl Particle {
//     fn new(x: f32, y: f32, vx: f32, vy: f32, radius: f32, color: Color) -> Self {
//         Self {
//             pos: vec2(x, y),
//             vel: vec2(vx, vy),
//             radius,
//             base_radius: radius,
//             color,
//             trail: Vec::with_capacity(TRAIL_LENGTH),
//             pulse: rand::gen_range(0.0, std::f32::consts::PI * 2.0),
//             heat: 0.0,
//             mass: radius * radius * 0.01,
//         }
//     }

//     fn update(
//         &mut self,
//         dt: f32,
//         mode: PhysicsMode,
//         mouse_pos: Vec2,
//         mouse_down: bool,
//         mouse_right: bool,
//     ) {
//         self.pulse += dt * 3.0;

//         // Trail management
//         if self.trail.len() >= TRAIL_LENGTH {
//             self.trail.remove(0);
//         }
//         self.trail.push(self.pos);

//         // Mouse interaction
//         let to_mouse = mouse_pos - self.pos;
//         let dist = to_mouse.length();
//         if mouse_down && dist > 5.0 {
//             let force = 800.0 / (dist + 10.0);
//             self.vel += to_mouse.normalize() * force * dt;
//             self.heat = (self.heat + dt * 3.0).min(1.0);
//         }
//         if mouse_right && dist > 5.0 {
//             let force = 1200.0 / (dist + 10.0);
//             self.vel -= to_mouse.normalize() * force * dt;
//             self.heat = (self.heat + dt * 2.0).min(1.0);
//         }

//         // Mode physics
//         match mode {
//             PhysicsMode::Gravity => {
//                 self.vel.y += 400.0 * dt;
//                 self.vel *= 0.999;
//             }
//             PhysicsMode::Orbit => {
//                 let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
//                 let to_center = center - self.pos;
//                 let dist_sq = to_center.length_squared().max(100.0);
//                 self.vel += to_center.normalize() * (2000.0 / dist_sq) * dt;
//                 self.vel *= 0.995;
//             }
//             PhysicsMode::Chaos => {
//                 self.vel += vec2(
//                     rand::gen_range(-100.0, 100.0),
//                     rand::gen_range(-100.0, 100.0),
//                 ) * dt;
//                 self.vel *= 0.99;
//             }
//             PhysicsMode::ZeroG => {
//                 self.vel *= 0.998;
//             }
//         }

//         // Update position
//         self.pos += self.vel * dt;

//         // Boundaries with damping
//         let damp = if mode == PhysicsMode::ZeroG {
//             1.0
//         } else {
//             0.85
//         };
//         if self.pos.x - self.radius < 0.0 {
//             self.pos.x = self.radius;
//             self.vel.x *= -damp;
//         }
//         if self.pos.x + self.radius > screen_width() {
//             self.pos.x = screen_width() - self.radius;
//             self.vel.x *= -damp;
//         }
//         if self.pos.y - self.radius < 0.0 {
//             self.pos.y = self.radius;
//             self.vel.y *= -damp;
//         }
//         if self.pos.y + self.radius > screen_height() {
//             self.pos.y = screen_height() - self.radius;
//             self.vel.y *= -damp;
//         }

//         // Heat decay and breathing
//         self.heat *= 0.98;
//         self.radius = self.base_radius + (self.pulse.sin() * 1.5);
//     }

//     fn draw(&self, show_trails: bool, show_glow: bool) {
//         // Glow
//         if show_glow {
//             let glow_size = self.radius * GLOW_RADIUS_MULT * (1.0 + self.heat);
//             let alpha = (0.15 + self.heat * 0.2) * 255.0;
//             draw_circle(
//                 self.pos.x,
//                 self.pos.y,
//                 glow_size,
//                 Color::new(self.color.r, self.color.g, self.color.b, alpha / 255.0),
//             );
//         }

//         // Trail
//         if show_trails && self.trail.len() > 1 {
//             for i in 1..self.trail.len() {
//                 let alpha = (i as f32 / self.trail.len() as f32) * 0.3;
//                 let t = self.trail[i];
//                 draw_circle(
//                     t.x,
//                     t.y,
//                     self.radius * 0.4 * (i as f32 / self.trail.len() as f32),
//                     Color::new(self.color.r, self.color.g, self.color.b, alpha),
//                 );
//             }
//         }

//         // Core
//         draw_circle(self.pos.x, self.pos.y, self.radius, self.color);

//         // Highlight
//         let hl_alpha = 0.4 + self.heat * 0.4;
//         draw_circle(
//             self.pos.x - self.radius * 0.3,
//             self.pos.y - self.radius * 0.3,
//             self.radius * 0.35,
//             Color::new(1.0, 1.0, 1.0, hl_alpha),
//         );
//     }
// }

// fn aurora_color() -> Color {
//     let palette = [
//         Color::new(0.0, 0.94, 1.0, 1.0),   // Cyan
//         Color::new(1.0, 0.0, 0.43, 1.0),   // Pink
//         Color::new(1.0, 0.75, 0.04, 1.0),  // Gold
//         Color::new(0.51, 0.22, 0.93, 1.0), // Purple
//         Color::new(0.23, 0.51, 1.0, 1.0),  // Blue
//         Color::new(0.02, 1.0, 0.65, 1.0),  // Mint
//         Color::new(1.0, 0.33, 0.0, 1.0),   // Orange
//     ];
//     palette[rand::gen_range(0, palette.len())]
// }

// fn spawn_burst(x: f32, y: f32, count: i32, speed: f32) -> Vec<Particle> {
//     let mut particles = Vec::new();
//     for _ in 0..count {
//         let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
//         let spd = rand::gen_range(0.0, speed);
//         particles.push(Particle::new(
//             x + rand::gen_range(-10.0, 10.0),
//             y + rand::gen_range(-10.0, 10.0),
//             angle.cos() * spd,
//             angle.sin() * spd,
//             rand::gen_range(3.0, 9.0),
//             aurora_color(),
//         ));
//     }
//     particles
// }

// fn window_conf() -> Conf {
//     Conf {
//         window_title: "Æther Flow — Creative Particle System".to_string(),
//         window_width: 1600,
//         window_height: 900,
//         window_resizable: true,
//         ..Default::default()
//     }
// }

// #[macroquad::main(window_conf)]
// async fn main() {
//     let mut particles: Vec<Particle> =
//         spawn_burst(screen_width() / 2.0, screen_height() / 3.0, 80, 300.0);
//     let mut mode = PhysicsMode::Gravity;
//     let mut show_trails = true;
//     let mut show_glow = true;
//     let mut show_connect = true;

//     loop {
//         let dt = get_frame_time().min(0.05);
//         let mouse_pos = mouse_position();
//         let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);
//         let mouse_down = is_mouse_button_down(MouseButton::Left);
//         let mouse_right = is_mouse_button_down(MouseButton::Right);

//         clear_background(Color::new(0.04, 0.04, 0.06, 1.0));

//         // Input handling
//         if is_mouse_button_pressed(MouseButton::Left) {
//             let new_particles = spawn_burst(mouse_vec.x, mouse_vec.y, 5, 200.0);
//             particles.extend(new_particles);
//         }
//         if is_mouse_button_pressed(MouseButton::Right) {
//             let new_particles = spawn_burst(mouse_vec.x, mouse_vec.y, 10, 400.0);
//             particles.extend(new_particles);
//         }

//         if is_key_pressed(KeyCode::Space) {
//             let new_particles =
//                 spawn_burst(rand::gen_range(50.0, screen_width() - 50.0), 50.0, 1, 100.0);
//             particles.extend(new_particles);
//         }
//         if is_key_pressed(KeyCode::C) {
//             particles.clear();
//         }
//         if is_key_pressed(KeyCode::G) {
//             mode = PhysicsMode::Gravity;
//         }
//         if is_key_pressed(KeyCode::O) {
//             mode = PhysicsMode::Orbit;
//         }
//         if is_key_pressed(KeyCode::H) {
//             mode = PhysicsMode::Chaos;
//         }
//         if is_key_pressed(KeyCode::Z) {
//             mode = PhysicsMode::ZeroG;
//         }
//         if is_key_pressed(KeyCode::T) {
//             show_trails = !show_trails;
//         }
//         if is_key_pressed(KeyCode::L) {
//             show_connect = !show_connect;
//         }
//         if is_key_pressed(KeyCode::B) {
//             show_glow = !show_glow;
//         }
//         if is_key_pressed(KeyCode::E) {
//             // Explode
//             let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
//             for p in &mut particles {
//                 let away = p.pos - center;
//                 let dist = away.length().max(1.0);
//                 p.vel += away.normalize() * (2000.0 / dist);
//                 p.heat = 1.0;
//             }
//             let burst = spawn_burst(center.x, center.y, 20, 600.0);
//             particles.extend(burst);
//         }
//         if is_key_pressed(KeyCode::I) {
//             // Implode
//             let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
//             for p in &mut particles {
//                 let toward = center - p.pos;
//                 let dist = toward.length().max(1.0);
//                 p.vel += toward.normalize() * (3000.0 / dist);
//             }
//         }

//         // Auto-spawn in chaos mode
//         if mode == PhysicsMode::Chaos && particles.len() < 80 && rand::gen_range(0.0, 1.0) < 0.02 {
//             let new_p = spawn_burst(
//                 rand::gen_range(0.0, screen_width()),
//                 rand::gen_range(0.0, screen_height()),
//                 1,
//                 100.0,
//             );
//             particles.extend(new_p);
//         }

//         // Update
//         for p in &mut particles {
//             p.update(dt, mode, mouse_vec, mouse_down, mouse_right);
//         }

//         // Mass-aware collisions
//         for i in 0..particles.len() {
//             for j in (i + 1)..particles.len() {
//                 let dist = particles[i].pos.distance(particles[j].pos);
//                 let min_dist = particles[i].radius + particles[j].radius;

//                 if dist < min_dist && dist > 0.0 {
//                     let overlap = min_dist - dist;
//                     let normal = (particles[j].pos - particles[i].pos).normalize();

//                     particles[i].pos -= normal * overlap * 0.5;
//                     particles[j].pos += normal * overlap * 0.5;

//                     let rel_vel = particles[j].vel - particles[i].vel;
//                     let vel_along = rel_vel.dot(normal);

//                     if vel_along > 0.0 {
//                         let m1 = particles[i].mass;
//                         let m2 = particles[j].mass;
//                         let impulse = 2.0 * vel_along / (m1 + m2);

//                         particles[i].vel += normal * impulse * m2;
//                         particles[j].vel -= normal * impulse * m1;

//                         particles[i].heat = (particles[i].heat + 0.3).min(1.0);
//                         particles[j].heat = (particles[j].heat + 0.3).min(1.0);
//                     }
//                 }
//             }
//         }

//         // Draw connections
//         if show_connect {
//             for i in 0..particles.len() {
//                 for j in (i + 1)..particles.len() {
//                     let dist = particles[i].pos.distance(particles[j].pos);
//                     if dist < CONNECTION_DIST {
//                         let alpha = (1.0 - dist / CONNECTION_DIST) * 0.3;
//                         let color = Color::new(
//                             particles[i].color.r,
//                             particles[i].color.g,
//                             particles[i].color.b,
//                             alpha,
//                         );
//                         draw_line(
//                             particles[i].pos.x,
//                             particles[i].pos.y,
//                             particles[j].pos.x,
//                             particles[j].pos.y,
//                             1.0 + (1.0 - dist / CONNECTION_DIST) * 1.5,
//                             color,
//                         );
//                     }
//                 }
//             }
//         }

//         // Draw particles
//         for p in &particles {
//             p.draw(show_trails, show_glow);
//         }

//         // UI
//         let mode_str = match mode {
//             PhysicsMode::Gravity => "Gravity",
//             PhysicsMode::Orbit => "Orbit",
//             PhysicsMode::Chaos => "Chaos",
//             PhysicsMode::ZeroG => "Zero-G",
//         };

//         let energy: f32 = particles
//             .iter()
//             .map(|p| p.vel.length_squared() * p.mass)
//             .sum();

//         draw_text(
//             &format!(
//                 "Æther Flow | Particles: {} | Mode: {} | Energy: {:.0}\n\
//                  [G]ravity [O]rbit C[H]aos [Z]ero-G | [T]rails [{}] [L]ines [{}] [B]loom [{}]\n\
//                  [E]xplode [I]mplode [C]lear | LMB: Attract | RMB: Repel",
//                 particles.len(),
//                 mode_str,
//                 energy / 1000.0,
//                 if show_trails { "ON" } else { "OFF" },
//                 if show_connect { "ON" } else { "OFF" },
//                 if show_glow { "ON" } else { "OFF" },
//             ),
//             15.0,
//             25.0,
//             18.0,
//             Color::new(0.7, 0.7, 0.7, 1.0),
//         );

//         next_frame().await;
//     }
// }
use macroquad::prelude::*;

const MAX_FIREFLIES: usize = 160;
const CONNECT_DIST: f32 = 110.0;
const MOUSE_INFLUENCE_RADIUS: f32 = 140.0;
const BLOOM_RADIUS: f32 = 220.0;

#[derive(Clone, Copy)]
struct Palette {
    name: &'static str,
    bg: Color,
    glow: Color,
}

struct Firefly {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    phase: f32,
    blink_speed: f32,
}

impl Firefly {
    fn new(x: f32, y: f32) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let speed = rand::gen_range(10.0, 30.0);
        Self {
            pos: vec2(x, y),
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            radius: rand::gen_range(1.6, 3.2),
            phase: rand::gen_range(0.0, std::f32::consts::TAU),
            blink_speed: rand::gen_range(0.6, 1.5),
        }
    }
}

struct Bloom {
    pos: Vec2,
    born: f64,
    life: f64,
    strength: f32, // positive = attracts fireflies, negative = scatters them
}

fn spawn_firefly(fireflies: &mut Vec<Firefly>, x: f32, y: f32) {
    if fireflies.len() >= MAX_FIREFLIES {
        fireflies.remove(0);
    }
    fireflies.push(Firefly::new(x, y));
}

fn plant_bloom(
    blooms: &mut Vec<Bloom>,
    fireflies: &mut Vec<Firefly>,
    pos: Vec2,
    big: bool,
    t: f64,
) {
    blooms.push(Bloom {
        pos,
        born: t,
        life: if big { 3.4 } else { 5.2 },
        strength: if big { -1.0 } else { 1.0 },
    });
    let n = if big { 14 } else { 6 };
    for _ in 0..n {
        spawn_firefly(
            fireflies,
            pos.x + rand::gen_range(-10.0, 10.0),
            pos.y + rand::gen_range(-10.0, 10.0),
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Firefly Garden".to_string(),
        window_width: 3560,
        window_height: 2000,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let palettes = [
        Palette {
            name: "amber dusk",
            bg: Color::from_rgba(11, 31, 34, 255),
            glow: Color::from_rgba(255, 196, 110, 255),
        },
        Palette {
            name: "violet night",
            bg: Color::from_rgba(21, 15, 38, 255),
            glow: Color::from_rgba(205, 175, 255, 255),
        },
        Palette {
            name: "rose evening",
            bg: Color::from_rgba(35, 16, 24, 255),
            glow: Color::from_rgba(255, 160, 170, 255),
        },
    ];
    let mut palette_index = 0usize;

    let mut fireflies: Vec<Firefly> = (0..60)
        .map(|_| {
            Firefly::new(
                rand::gen_range(0.0, screen_width()),
                rand::gen_range(0.0, screen_height()),
            )
        })
        .collect();
    let mut blooms: Vec<Bloom> = vec![];
    let mut show_lines = true;
    let mut left_press_time: f64 = 0.0;
    let (mx0, my0) = mouse_position();
    let mut mouse_prev = vec2(mx0, my0);

    clear_background(palettes[palette_index].bg);

    loop {
        let dt = get_frame_time().min(0.05);
        let t = get_time();
        let palette = palettes[palette_index];

        // Translucent overlay instead of clear_background: leaves soft trails
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(palette.bg.r, palette.bg.g, palette.bg.b, 0.35),
        );

        let (mx, my) = mouse_position();
        let mouse_pos = vec2(mx, my);
        let mouse_vel = mouse_pos - mouse_prev;
        mouse_prev = mouse_pos;

        if is_mouse_button_pressed(MouseButton::Left) {
            left_press_time = t;
        }
        if is_mouse_button_released(MouseButton::Left) && t - left_press_time < 0.26 {
            plant_bloom(&mut blooms, &mut fireflies, mouse_pos, false, t);
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            plant_bloom(&mut blooms, &mut fireflies, mouse_pos, true, t);
        }

        if is_key_pressed(KeyCode::Space) {
            let side = rand::gen_range(0, 4);
            let (x, y) = match side {
                0 => (rand::gen_range(0.0, screen_width()), -10.0),
                1 => (screen_width() + 10.0, rand::gen_range(0.0, screen_height())),
                2 => (rand::gen_range(0.0, screen_width()), screen_height() + 10.0),
                _ => (-10.0, rand::gen_range(0.0, screen_height())),
            };
            spawn_firefly(&mut fireflies, x, y);
        }
        if is_key_pressed(KeyCode::C) {
            fireflies.clear();
            blooms.clear();
        }
        if is_key_pressed(KeyCode::L) {
            show_lines = !show_lines;
        }
        if is_key_pressed(KeyCode::P) {
            palette_index = (palette_index + 1) % palettes.len();
        }

        blooms.retain(|b| t - b.born < b.life);

        for f in fireflies.iter_mut() {
            let flow_x = (f.pos.y * 0.006 + t as f32 * 0.35).sin() * 6.0;
            let flow_y = (f.pos.x * 0.006 + t as f32 * 0.45).cos() * 6.0;
            f.vel.x += flow_x * dt;
            f.vel.y += flow_y * dt;

            let to_mouse = mouse_pos - f.pos;
            let dist_mouse = to_mouse.length();
            if dist_mouse < MOUSE_INFLUENCE_RADIUS {
                let influence = 1.0 - dist_mouse / MOUSE_INFLUENCE_RADIUS;
                f.vel += mouse_vel * influence * 0.6;
            }

            for b in blooms.iter() {
                let to_bloom = b.pos - f.pos;
                let d = to_bloom.length();
                if d > 0.001 && d < BLOOM_RADIUS {
                    let age = (t - b.born) / b.life;
                    let strength = (1.0 - age as f32) * 40.0 * b.strength;
                    f.vel += (to_bloom / d) * strength * dt;
                }
            }

            f.vel *= 0.985;
            let speed = f.vel.length();
            if speed > 90.0 {
                f.vel = f.vel / speed * 90.0;
            }

            f.pos += f.vel * dt;

            if f.pos.x < -20.0 {
                f.pos.x = screen_width() + 20.0;
            }
            if f.pos.x > screen_width() + 20.0 {
                f.pos.x = -20.0;
            }
            if f.pos.y < -20.0 {
                f.pos.y = screen_height() + 20.0;
            }
            if f.pos.y > screen_height() + 20.0 {
                f.pos.y = -20.0;
            }
        }

        // Same collision technique as the original sketch: split_at_mut + velocity swap
        let total = fireflies.len();
        for i in 0..total {
            let (left, right) = fireflies.split_at_mut(i + 1);
            let a = &mut left[i];
            for b in right {
                let dist = b.pos.distance(a.pos);
                let min_d = a.radius + b.radius + 2.0;
                if dist < min_d && dist > 0.0 {
                    let overlap = (min_d - dist) / 2.0;
                    let normal = (b.pos - a.pos).normalize();
                    b.pos += normal * overlap;
                    a.pos -= normal * overlap;
                    std::mem::swap(&mut a.vel, &mut b.vel);
                } else if show_lines && dist < CONNECT_DIST {
                    let alpha = (1.0 - dist / CONNECT_DIST) * 0.5;
                    draw_line(
                        a.pos.x,
                        a.pos.y,
                        b.pos.x,
                        b.pos.y,
                        1.0,
                        Color::new(palette.glow.r, palette.glow.g, palette.glow.b, alpha),
                    );
                }
            }
        }

        for f in fireflies.iter() {
            let blink = 0.55 + 0.45 * (t as f32 * f.blink_speed + f.phase).sin();
            for step in 0..3 {
                let layer = 3 - step;
                let r = f.radius * (2.0 + layer as f32 * 2.5);
                let alpha = 0.16 * blink / layer as f32;
                draw_circle(
                    f.pos.x,
                    f.pos.y,
                    r,
                    Color::new(palette.glow.r, palette.glow.g, palette.glow.b, alpha),
                );
            }
            draw_circle(
                f.pos.x,
                f.pos.y,
                f.radius,
                Color::new(1.0, 0.98, 0.92, 0.75 * blink + 0.15),
            );
        }

        for b in blooms.iter() {
            let age = ((t - b.born) / b.life) as f32;
            let ring_r = 20.0 + age * 160.0;
            let alpha = ((1.0 - age) * 0.35).max(0.0);
            draw_circle_lines(
                b.pos.x,
                b.pos.y,
                ring_r,
                1.4,
                Color::new(palette.glow.r, palette.glow.g, palette.glow.b, alpha),
            );
        }

        draw_text(
            &format!(
                "{} fireflies | {} | click: bloom  right-click: supernova  space: firefly  c: clear  l: lines  p: palette",
                fireflies.len(),
                palette.name
            ),
            15.0,
            30.0,
            24.0,
            Color::new(palette.glow.r, palette.glow.g, palette.glow.b, 0.85),
        );

        next_frame().await;
    }
}
