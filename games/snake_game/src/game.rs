use macroquad::prelude::*;

use crate::constants::*;
use crate::grid::*;
use crate::snake::*;
use crate::ui::*;

struct Notification {
    text: String,
    color: Color,
    timer: f32,
    max_timer: f32,
}

// ── Main ──────────────────────────────────────────────────────────────────────
pub async fn run_game() {
    macroquad::rand::srand(macroquad::miniquad::date::now() as _);

    // Load custom Orbitron font; fall back gracefully if missing
    let font = load_ttf_font("assets/Orbitron-Regular.ttf")
        .await
        .expect("Font not found. Place assets/Orbitron-Regular.ttf in the game folder.");

    let _mat_params = MaterialParams {
        uniforms: vec![UniformDesc::new("time", UniformType::Float1)],
        ..Default::default()
    };
    let shaders = vec![
        load_material(
            ShaderSource::Glsl {
                vertex: crate::shaders::VERTEX_SHADER,
                fragment: crate::shaders::FRAGMENT_SHADER_ZEBRA,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("time", UniformType::Float1),
                    UniformDesc::new("base_color", UniformType::Float3),
                ],
                ..Default::default()
            },
        )
        .unwrap(),
        load_material(
            ShaderSource::Glsl {
                vertex: crate::shaders::VERTEX_SHADER,
                fragment: crate::shaders::FRAGMENT_SHADER_PLASMA,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("time", UniformType::Float1),
                    UniformDesc::new("base_color", UniformType::Float3),
                ],
                ..Default::default()
            },
        )
        .unwrap(),
    ];

    let mut snakes: Vec<Snake> = vec![];
    let mut foods: Vec<Food> = vec![];
    let mut notifications: Vec<Notification> = Vec::new();
    let mut next_id: u32 = 0;
    let mut selected_id: Option<u32> = None;
    let mut cam_mode = CamMode::FullMap;
    let mut show_ui = true;
    let mut death_cam_override: Option<(Vec2, f32, u32)> = None;
    let mut actual_cam_target = vec2(ARENA_W / 2.0, ARENA_H / 2.0);

    let mut obstacles: Vec<Obstacle> = vec![];
    for _ in 0..OBSTACLE_COUNT {
        let is_pond = macroquad::rand::gen_range(0, 3) == 0;
        let kind = if is_pond {
            ObstacleKind::Pond
        } else {
            ObstacleKind::Stone
        };
        let radius = macroquad::rand::gen_range(40.0, 100.0);
        let pos = vec2(
            macroquad::rand::gen_range(radius, ARENA_W - radius),
            macroquad::rand::gen_range(radius, ARENA_H - radius),
        );
        let mut points = vec![];
        let num_pts = if is_pond {
            16
        } else {
            macroquad::rand::gen_range(5, 9)
        };
        for i in 0..num_pts {
            let a = (i as f32 / num_pts as f32) * std::f32::consts::TAU;
            let r = radius
                * if is_pond {
                    macroquad::rand::gen_range(0.85, 1.0)
                } else {
                    macroquad::rand::gen_range(0.6, 1.0)
                };
            points.push(vec2(a.cos() * r, a.sin() * r));
        }
        obstacles.push(Obstacle {
            pos,
            radius,
            kind,
            points,
        });
    }

    loop {
        let sw = screen_width();
        let sh = screen_height();
        let dt = get_frame_time().min(0.05);

        // ── Replenish ──────────────────────────────────────────────────────────
        while foods.len() < MIN_FOOD {
            let r = macroquad::rand::gen_range(0, 100);
            let kind = if r < 2 {
                FoodKind::Skin(macroquad::rand::gen_range(0, 2))
            } else if r < 6 {
                FoodKind::Ghost
            } else if r < 18 {
                FoodKind::Speed
            } else {
                FoodKind::Normal
            };
            let mut pos = vec2(0.0, 0.0);
            let mut valid = false;
            while !valid {
                pos = vec2(
                    macroquad::rand::gen_range(20.0, ARENA_W - 20.0),
                    macroquad::rand::gen_range(20.0, ARENA_H - 20.0),
                );
                valid = true;
                for obs in &obstacles {
                    if pos.distance(obs.pos) < obs.radius {
                        valid = false;
                        break;
                    }
                }
            }

            foods.push(Food {
                pos,
                color: FOOD_COLORS[macroquad::rand::gen_range(0, FOOD_COLORS.len())],
                kind,
                visual_id: macroquad::rand::gen_range(0, 1000),
                lifetime: None, // permanent
            });
        }
        while snakes.len() < MAX_SNAKES {
            let color = FOOD_COLORS[macroquad::rand::gen_range(0, FOOD_COLORS.len())];
            let x = macroquad::rand::gen_range(80.0, ARENA_W - 80.0);
            let y = macroquad::rand::gen_range(80.0, ARENA_H - 80.0);
            let new_snake = Snake::new(x, y, 10, color, next_id);
            
            // Only notify if we aren't completely initializing the game
            if next_id > MAX_SNAKES as u32 {
                notifications.push(Notification {
                    text: format!("{} joined the arena!", new_snake.name),
                    color: Color::new(0.3, 1.0, 0.4, 1.0),
                    timer: 3.0,
                    max_timer: 3.0,
                });
            }
            
            snakes.push(new_snake);
            next_id += 1;
        }

        // ── Build spatial grids ────────────────────────────────────────────────
        let mut grid = SpatialGrid::new(ARENA_W, ARENA_H);
        for (si, s) in snakes.iter().enumerate() {
            if !s.dead {
                for (ki, seg) in s.segments.iter().enumerate() {
                    grid.insert(*seg, si, ki);
                }
            }
        }
        let mut food_grid = FoodGrid::new(ARENA_W, ARENA_H);
        for f in &foods {
            food_grid.insert(f.pos);
        }

        let all_segs: Vec<Vec<Vec2>> = snakes
            .iter()
            .map(|s| if s.dead { vec![] } else { s.segments.clone() })
            .collect();
        let all_angles: Vec<f32> = snakes.iter().map(|s| s.angle).collect();
        let all_ids: Vec<u32> = snakes.iter().map(|s| s.id).collect();

        // ── Rankings ───────────────────────────────────────────────────────────
        let mut ranked: Vec<usize> = (0..snakes.len()).collect();
        ranked.sort_unstable_by(|&a, &b| snakes[b].segments.len().cmp(&snakes[a].segments.len()));

        // ── Update AI ──────────────────────────────────────────────────────────
        let leader_id = ranked
            .first()
            .and_then(|&i| snakes.get(i))
            .map(|s| s.id)
            .unwrap_or(u32::MAX);

        for (i, snake) in snakes.iter_mut().enumerate() {
            let is_leader = snake.id == leader_id;
            let my_len = snake.segments.len();
            let rank = all_segs.iter().filter(|s| s.len() > my_len).count() + 1;
            snake.update(
                dt,
                &mut foods,
                &food_grid,
                ARENA_W,
                ARENA_H,
                &grid,
                &all_segs,
                &all_angles,
                &obstacles,
                &all_ids,
                i,
                is_leader,
                rank,
            );
        }

        // ── Collisions ─────────────────────────────────────────────────────────
        let mut killers = vec![];
        let mut dead_ids = vec![];

        for i in 0..snakes.len() {
            if snakes[i].dead {
                continue;
            }
            let head = snakes[i].segments[0];
            let my_radius = snakes[i].radius();
            let query_r = my_radius * 1.5 + 20.0; // max other radius approx 20
            'col: for (si, ki) in grid.query_radius(head, query_r) {
                let si = si as usize;
                if si == i {
                    continue;
                }
                if let Some(seg) = all_segs.get(si).and_then(|s| s.get(ki as usize)) {
                    let other_radius = snakes[si].radius();
                    let actual_hit_r = my_radius + other_radius * 0.4;
                    if head.distance(*seg) < actual_hit_r {
                        snakes[i].dead = true;
                        killers.push((si, snakes[i].segments.len(), snakes[i].id));
                        dead_ids.push(snakes[i].id);
                        
                        notifications.push(Notification {
                            text: format!("{} was eliminated by {}!", snakes[i].name, snakes[si].name),
                            color: Color::new(1.0, 0.4, 0.4, 1.0),
                            timer: 3.5,
                            max_timer: 3.5,
                        });
                        
                        break 'col;
                    }
                }
            }

            // Obstacles just push segments away now (handled in snake.rs). No longer cause death.
        }

        // Apply glow effect and bonus points to killers
        for &(si, dead_len, dead_id) in &killers {
            if !snakes[si].dead {
                snakes[si].grow_queue += dead_len / 2;
                snakes[si].glow_timer = 5.0; // Happy glow for 5 seconds
                snakes[si].happy_timer = 3.0; // Happy emote
                snakes[si].hunt_cooldown = 30.0;
                snakes[si].state = SnakeState::Foraging;

                // Transfer camera to killer if followed snake died
                if Some(dead_id) == selected_id {
                    selected_id = Some(snakes[si].id); // Immediately jump to killer! No lingering.
                }
            }
        }

        // Apply happy effect to snakes whose targets died
        for s in &mut snakes {
            if let SnakeState::Hunting { target_id, .. } = s.state
                && dead_ids.contains(&target_id)
            {
                s.happy_timer = 3.0; // Happy for 3 seconds
                s.hunt_cooldown = 60.0; // 1 minute cooldown before hunting again
            }
        }

        // ── Deaths → food chain reaction ────────────────────────────────────────
        let mut nf: Vec<Food> = vec![];
        for s in &mut snakes {
            if s.dead {
                // Instantly pop all segments into food
                while !s.segments.is_empty() {
                    let seg = s.segments.pop().unwrap();

                    let total = BODY_FOOD_BASE;
                    let pos = seg
                        + vec2(
                            macroquad::rand::gen_range(-2.0, 2.0),
                            macroquad::rand::gen_range(-2.0, 2.0),
                        );

                    let mut valid = true;
                    for obs in &obstacles {
                        if pos.distance(obs.pos) < obs.radius {
                            valid = false;
                            break;
                        }
                    }
                    if !valid {
                        continue;
                    }

                    nf.push(Food {
                        pos,
                        color: s.color,
                        kind: FoodKind::Normal,
                        visual_id: macroquad::rand::gen_range(0, 1000), // Random visual snack
                        lifetime: Some((total, total)),
                    });
                }
            }
        }
        foods.extend(nf);
        snakes.retain(|s| !s.dead);

        // ── Tick body-food lifetimes & purge expired ───────────────────────────
        foods.retain_mut(|f| {
            if let Some((ref mut left, _total)) = f.lifetime {
                *left -= dt;
                *left > 0.0
            } else {
                true // permanent food never removed by timer
            }
        });

        // ── Rankings ───────────────────────────────────────────────────────────
        let mut ranked: Vec<usize> = (0..snakes.len()).collect();
        ranked.sort_unstable_by(|&a, &b| snakes[b].segments.len().cmp(&snakes[a].segments.len()));

        // Check if selected snake is dead or invalid
        if selected_id.is_some() && !snakes.iter().any(|s| Some(s.id) == selected_id) {
            selected_id = None;
        }

        // If no snake is selected AND we are not lingering on a death cam, select top snake
        if selected_id.is_none() && death_cam_override.is_none() {
            selected_id = ranked.first().and_then(|&i| snakes.get(i)).map(|s| s.id);
        }

        let follow_idx = selected_id.and_then(|id| snakes.iter().position(|s| s.id == id));
        let desired_cam_target;
        if let Some((pos, ref mut timer, killer_id)) = death_cam_override {
            *timer -= dt;
            desired_cam_target = pos;
            if *timer <= 0.0 {
                selected_id = Some(killer_id);
                death_cam_override = None;
            }
        } else {
            desired_cam_target = follow_idx
                .and_then(|i| snakes.get(i))
                .and_then(|s| s.segments.first())
                .copied()
                .unwrap_or(vec2(ARENA_W / 2.0, ARENA_H / 2.0));
        }

        // Smooth camera transition (framerate independent lerp)
        let lerp_factor = 1.0 - (-10.0 * dt).exp();
        actual_cam_target = actual_cam_target.lerp(desired_cam_target, lerp_factor);

        // ── Max segs for bar scaling ───────────────────────────────────────────
        let leader_id = ranked
            .first()
            .and_then(|&i| snakes.get(i))
            .map(|s| s.id)
            .unwrap_or(u32::MAX);

        let _max_segs = ranked
            .first()
            .and_then(|&i| snakes.get(i))
            .map(|s| s.segments.len())
            .unwrap_or(1)
            .max(1) as f32;

        // ── RENDER WORLD ───────────────────────────────────────────────────────
        clear_background(BG_COLOR);
        let current_cam = apply_camera(&cam_mode, actual_cam_target, sw, sh);
        set_camera(&current_cam);

        // Culling rect (camera bounds + padding)
        let cam_rect = match cam_mode {
            CamMode::Follow => Rect::new(
                actual_cam_target.x - sw / 2.0 - 200.0,
                actual_cam_target.y - sh / 2.0 - 200.0,
                sw + 400.0,
                sh + 400.0,
            ),
            CamMode::FullMap => Rect::new(0.0, 0.0, ARENA_W, ARENA_H),
        };

        let gs = 40.0;
        let start_x = (cam_rect.x / gs).floor() as i32;
        let end_x = ((cam_rect.x + cam_rect.w) / gs).ceil() as i32;
        let start_y = (cam_rect.y / gs).floor() as i32;
        let end_y = ((cam_rect.y + cam_rect.h) / gs).ceil() as i32;

        let t = get_time() as f32;
        let base_alpha = 0.015;

        let line_thick = if cam_mode == CamMode::FullMap {
            ARENA_W / sw * 1.5 // Scale thickness so it's visible when zoomed out
        } else {
            1.5
        };

        for xi in start_x..=end_x {
            let x = xi as f32 * gs;
            let pulse = ((x / 400.0 + t * 1.5).sin() * 0.5 + 0.5) * 0.035;
            let gc = Color::new(0.6, 0.7, 1.0, base_alpha + pulse);
            draw_line(x, cam_rect.y, x, cam_rect.y + cam_rect.h, line_thick, gc);
        }
        for yi in start_y..=end_y {
            let y = yi as f32 * gs;
            let pulse = ((y / 400.0 - t * 1.5).sin() * 0.5 + 0.5) * 0.035;
            let gc = Color::new(0.6, 0.7, 1.0, base_alpha + pulse);
            draw_line(cam_rect.x, y, cam_rect.x + cam_rect.w, y, line_thick, gc);
        }

        // Obstacles
        for obs in &obstacles {
            if !cam_rect.overlaps(&Rect::new(
                obs.pos.x - obs.radius,
                obs.pos.y - obs.radius,
                obs.radius * 2.0,
                obs.radius * 2.0,
            )) {
                continue;
            }
            match obs.kind {
                ObstacleKind::Stone => {
                    // Base dark stone
                    let base_col = Color::new(0.3, 0.3, 0.32, 1.0);
                    let mid_col = Color::new(0.4, 0.4, 0.42, 1.0);
                    let highlight_col = Color::new(0.5, 0.5, 0.55, 1.0);

                    for i in 0..obs.points.len() {
                        let p1 = obs.pos + obs.points[i];
                        let p2 = obs.pos + obs.points[(i + 1) % obs.points.len()];
                        draw_triangle(obs.pos, p1, p2, base_col);
                    }

                    // Mid layer
                    for i in 0..obs.points.len() {
                        let p1 = obs.pos + obs.points[i] * 0.7;
                        let p2 = obs.pos + obs.points[(i + 1) % obs.points.len()] * 0.7;
                        draw_triangle(obs.pos, p1, p2, mid_col);
                    }

                    // Highlight layer (offset towards top-left to simulate light)
                    let light_offset = vec2(-5.0, -5.0);
                    for i in 0..obs.points.len() {
                        let p1 = obs.pos + light_offset + obs.points[i] * 0.4;
                        let p2 =
                            obs.pos + light_offset + obs.points[(i + 1) % obs.points.len()] * 0.4;
                        draw_triangle(obs.pos + light_offset, p1, p2, highlight_col);
                    }

                    // Edges
                    for i in 0..obs.points.len() {
                        let p1 = obs.pos + obs.points[i];
                        let p2 = obs.pos + obs.points[(i + 1) % obs.points.len()];
                        draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, Color::new(0.2, 0.2, 0.22, 1.0));
                    }
                }
                ObstacleKind::Pond => {
                    // Deep water
                    let deep_col = Color::new(0.1, 0.25, 0.45, 0.8);
                    let shallow_col = Color::new(0.2, 0.45, 0.7, 0.4);

                    // Outer shallow water
                    for i in 0..obs.points.len() {
                        let p1 = obs.pos + obs.points[i];
                        let p2 = obs.pos + obs.points[(i + 1) % obs.points.len()];
                        draw_triangle(obs.pos, p1, p2, shallow_col);
                    }

                    // Inner deep water
                    for i in 0..obs.points.len() {
                        let p1 = obs.pos + obs.points[i] * 0.7;
                        let p2 = obs.pos + obs.points[(i + 1) % obs.points.len()] * 0.7;
                        draw_triangle(obs.pos, p1, p2, deep_col);
                    }

                    // Ripples (animated)
                    let time = get_time() as f32;
                    let ripple_scale = (time * 1.5).sin() * 0.05 + 0.85;
                    for i in 0..obs.points.len() {
                        let p1 = obs.pos + obs.points[i] * ripple_scale;
                        let p2 = obs.pos + obs.points[(i + 1) % obs.points.len()] * ripple_scale;
                        draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, Color::new(1.0, 1.0, 1.0, 0.15));
                    }

                    // Edges (shoreline)
                    for i in 0..obs.points.len() {
                        let p1 = obs.pos + obs.points[i];
                        let p2 = obs.pos + obs.points[(i + 1) % obs.points.len()];
                        draw_line(p1.x, p1.y, p2.x, p2.y, 4.0, Color::new(0.3, 0.6, 0.8, 0.3));
                    }
                }
            }
        }

        // Food — body food fades out as its timer runs down
        for f in &foods {
            if !cam_rect.contains(f.pos) {
                continue;
            }
            // Alpha: 1.0 when fresh, fades to 0 over the last BODY_FOOD_FADE seconds
            let alpha = if let Some((left, _total)) = f.lifetime {
                (left / BODY_FOOD_FADE).clamp(0.0, 1.0)
            } else {
                1.0
            };
            // Scale body-food radius slightly as it dies (shrinks)
            let scale = if f.lifetime.is_some() {
                if let Some((left, total)) = f.lifetime {
                    (left / total).sqrt().max(0.3)
                } else {
                    1.0
                }
            } else {
                1.0
            };
            let r = FOOD_RADIUS * scale;
            let c_outer = Color::new(f.color.r, f.color.g, f.color.b, 0.15 * alpha);
            let c_inner = Color::new(f.color.r, f.color.g, f.color.b, alpha);

            match f.kind {
                FoodKind::Normal => {
                    // Random snacks
                    let hash = f.visual_id;
                    match hash % 5 {
                        0 => {
                            // Candy
                            draw_poly(f.pos.x - r * 1.2, f.pos.y, 3, r * 1.2, 90.0, c_inner);
                            draw_poly(f.pos.x + r * 1.2, f.pos.y, 3, r * 1.2, -90.0, c_inner);
                            draw_circle(f.pos.x, f.pos.y, r * 1.2, c_outer);
                            draw_circle(f.pos.x, f.pos.y, r * 1.0, c_inner);
                        }
                        1 => {
                            // Soda can
                            draw_rectangle(
                                f.pos.x - r,
                                f.pos.y - r * 1.5,
                                r * 2.0,
                                r * 3.0,
                                c_inner,
                            );
                            draw_rectangle(
                                f.pos.x - r * 0.8,
                                f.pos.y - r * 1.8,
                                r * 1.6,
                                r * 0.3,
                                Color::new(0.7, 0.7, 0.7, alpha),
                            );
                        }
                        2 => {
                            // Chip
                            draw_poly(
                                f.pos.x,
                                f.pos.y,
                                3,
                                r * 1.8,
                                45.0,
                                Color::new(0.9, 0.8, 0.2, alpha),
                            );
                            draw_poly(
                                f.pos.x,
                                f.pos.y,
                                3,
                                r * 1.2,
                                25.0,
                                Color::new(1.0, 0.9, 0.3, alpha),
                            );
                        }
                        3 => {
                            // Burger
                            draw_rectangle(
                                f.pos.x - r * 1.2,
                                f.pos.y + r * 0.2,
                                r * 2.4,
                                r * 0.8,
                                Color::new(0.8, 0.6, 0.3, alpha),
                            );
                            draw_rectangle(
                                f.pos.x - r * 1.3,
                                f.pos.y - r * 0.2,
                                r * 2.6,
                                r * 0.4,
                                Color::new(0.4, 0.2, 0.1, alpha),
                            );
                            draw_rectangle(
                                f.pos.x - r * 1.2,
                                f.pos.y - r * 1.0,
                                r * 2.4,
                                r * 0.8,
                                Color::new(0.8, 0.6, 0.3, alpha),
                            );
                        }
                        _ => {
                            // Apple
                            draw_circle(
                                f.pos.x,
                                f.pos.y,
                                r * 1.2,
                                Color::new(0.9, 0.1, 0.1, alpha),
                            );
                            draw_line(
                                f.pos.x,
                                f.pos.y - r * 1.0,
                                f.pos.x + r * 0.5,
                                f.pos.y - r * 1.8,
                                r * 0.3,
                                Color::new(0.3, 0.2, 0.1, alpha),
                            );
                            draw_circle(
                                f.pos.x + r * 0.8,
                                f.pos.y - r * 1.5,
                                r * 0.4,
                                Color::new(0.2, 0.8, 0.2, alpha),
                            );
                        }
                    }
                }
                FoodKind::Speed => {
                    // Energy Drink
                    draw_rectangle(
                        f.pos.x - r,
                        f.pos.y - r * 1.5,
                        r * 2.0,
                        r * 3.0,
                        Color::new(0.1, 0.4, 1.0, alpha),
                    );
                    draw_rectangle(
                        f.pos.x - r * 0.8,
                        f.pos.y - r * 1.8,
                        r * 1.6,
                        r * 0.3,
                        Color::new(0.8, 0.8, 0.8, alpha),
                    );
                    draw_poly(
                        f.pos.x,
                        f.pos.y,
                        3,
                        r * 0.8,
                        90.0,
                        Color::new(1.0, 1.0, 0.0, alpha),
                    );
                }
                FoodKind::Ghost => {
                    // Spooky Ghost
                    draw_circle(
                        f.pos.x,
                        f.pos.y - r * 0.5,
                        r * 1.5,
                        Color::new(0.9, 0.9, 1.0, alpha),
                    );
                    draw_rectangle(
                        f.pos.x - r * 1.5,
                        f.pos.y - r * 0.5,
                        r * 3.0,
                        r * 1.5,
                        Color::new(0.9, 0.9, 1.0, alpha),
                    );
                    draw_circle(
                        f.pos.x - r * 0.5,
                        f.pos.y - r * 0.8,
                        r * 0.3,
                        Color::new(0.1, 0.1, 0.1, alpha),
                    );
                    draw_circle(
                        f.pos.x + r * 0.5,
                        f.pos.y - r * 0.8,
                        r * 0.3,
                        Color::new(0.1, 0.1, 0.1, alpha),
                    );
                }
                FoodKind::Skin(_) => {
                    // Paint Bucket
                    draw_rectangle(
                        f.pos.x - r,
                        f.pos.y - r,
                        r * 2.0,
                        r * 2.0,
                        Color::new(0.5, 0.5, 0.5, alpha),
                    );
                    draw_rectangle(
                        f.pos.x - r * 0.8,
                        f.pos.y - r * 1.2,
                        r * 1.6,
                        r * 0.4,
                        c_inner,
                    );
                    draw_line(
                        f.pos.x - r,
                        f.pos.y - r,
                        f.pos.x,
                        f.pos.y - r * 2.0,
                        r * 0.2,
                        Color::new(0.7, 0.7, 0.7, alpha),
                    );
                    draw_line(
                        f.pos.x,
                        f.pos.y - r * 2.0,
                        f.pos.x + r,
                        f.pos.y - r,
                        r * 0.2,
                        Color::new(0.7, 0.7, 0.7, alpha),
                    );
                }
            }
        }

        for shader in &shaders {
            shader.set_uniform("time", get_time() as f32);
        }

        for s in &snakes {
            if !cam_rect.overlaps(&Rect::new(
                s.segments[0].x - 800.0,
                s.segments[0].y - 800.0,
                1600.0,
                1600.0,
            )) {
                continue;
            }
            s.draw(&shaders, s.id == leader_id);
        }

        // World border
        let bc = Color::new(0.0, 0.0, 0.0, 0.55);
        let bw = 14.0;
        draw_rectangle(0.0, 0.0, ARENA_W, bw, bc);
        draw_rectangle(0.0, ARENA_H - bw, ARENA_W, bw, bc);
        draw_rectangle(0.0, 0.0, bw, ARENA_H, bc);
        draw_rectangle(ARENA_W - bw, 0.0, bw, ARENA_H, bc);

        // ── RENDER UI (screen space) ───────────────────────────────────────────
        set_default_camera();

        // Draw name tag hovering dynamically in front of the followed snake
        if cam_mode == CamMode::Follow
            && let Some(fidx) = follow_idx
        {
            let snake = &snakes[fidx];
            if let Some(&head) = snake.segments.first() {
                let label = snake.name.clone();
                let d = measure_text(&label, Some(&font), 16, 1.0);

                // Dynamically position offset in front of the snake's nose (in world space)
                let off_x = snake.angle.cos() * 40.0;
                let off_y = snake.angle.sin() * 40.0;

                let screen_pos = current_cam.world_to_screen(vec2(head.x + off_x, head.y + off_y));

                draw_text_ex(
                    &label,
                    screen_pos.x - d.width / 2.0,
                    screen_pos.y - 8.0,
                    tp(&font, 16, Color::new(1.0, 1.0, 1.0, 0.9)),
                );
            }
        }

        // ── Panel layout (compute before draw for click detection) ─────────────
        let px = sw - PANEL_W - PANEL_PAD;
        let py = PANEL_PAD;
        let btn_rect = Rect::new(px + 6.0, py + HEADER_H + 4.0, PANEL_W - 12.0, BTN_H);
        let rows_start_y = py + HEADER_H + BTN_H + 16.0;

        let display_ranks: Vec<usize> = ranked.iter().copied().take(MAX_RANKS).collect();
        let mut row_rects: Vec<(Rect, u32)> = vec![];

        if show_ui {
            for (ri, &si) in display_ranks.iter().enumerate() {
                if si >= snakes.len() {
                    continue;
                }
                let ry = rows_start_y + ri as f32 * ROW_H;
                row_rects.push((Rect::new(px, ry, PANEL_W, ROW_H), snakes[si].id));
            }
        }

        if show_ui {
            let panel_h = HEADER_H + BTN_H + 16.0 + row_rects.len() as f32 * ROW_H + 6.0;

            // ── Draw panel ─────────────────────────────────────────────────────────
            draw_glass_rect(px, py, PANEL_W, panel_h, 0.88);

            // Header
            draw_text_ex(
                "LEADERBOARD",
                px + 12.0,
                py + 28.0,
                tp(&font, 22, Color::new(0.75, 0.85, 1.0, 1.0)),
            );

            // Crown emoji-equivalent beside header (use a dot for ranking)
            draw_circle(
                px + PANEL_W - 16.0,
                py + 20.0,
                5.0,
                Color::new(0.98, 0.80, 0.20, 1.0),
            );

            // Toggle button
            let btn_is_map = cam_mode == CamMode::FullMap;
            let btn_bg = if btn_is_map {
                Color::new(0.22, 0.42, 0.82, 0.90)
            } else {
                Color::new(0.12, 0.18, 0.35, 0.85)
            };
            draw_rectangle(btn_rect.x, btn_rect.y, btn_rect.w, btn_rect.h, btn_bg);
            draw_rectangle_lines(
                btn_rect.x,
                btn_rect.y,
                btn_rect.w,
                btn_rect.h,
                1.0,
                Color::new(0.4, 0.65, 1.0, 0.5),
            );
            let btn_lbl = if btn_is_map { "FOLLOW CAM" } else { "FULL MAP" };
            centered_text(btn_lbl, &font, 14, WHITE, btn_rect);

            // Separator
            draw_line(
                px + 8.0,
                rows_start_y - 5.0,
                px + PANEL_W - 8.0,
                rows_start_y - 5.0,
                1.0,
                Color::new(0.35, 0.45, 0.80, 0.3),
            );

            // ── Leaderboard rows ───────────────────────────────────────────────────
            let max_segs = snakes.iter().map(|s| s.segments.len()).max().unwrap_or(1) as f32;

            for (ri, (&si, &(rect, _sid))) in display_ranks.iter().zip(row_rects.iter()).enumerate()
            {
                if si >= snakes.len() {
                    continue;
                }
                let snake = &snakes[si];
                let is_sel = Some(snake.id) == selected_id;
                let mid_y = rect.y + ROW_H / 2.0;

                // Row highlight
                if is_sel {
                    draw_rectangle(
                        rect.x,
                        rect.y,
                        rect.w,
                        rect.h,
                        Color::new(0.25, 0.40, 0.75, 0.30),
                    );
                    draw_rectangle_lines(
                        rect.x,
                        rect.y,
                        rect.w,
                        rect.h,
                        1.0,
                        Color::new(0.45, 0.65, 1.0, 0.40),
                    );
                }

                // Rank number
                let rank_col = match ri {
                    0 => Color::new(0.98, 0.80, 0.20, 1.0), // gold
                    1 => Color::new(0.80, 0.80, 0.82, 1.0), // silver
                    2 => Color::new(0.80, 0.50, 0.20, 1.0), // bronze
                    _ => Color::new(0.55, 0.60, 0.85, 0.75),
                };
                draw_text_ex(
                    format!("#{}", ri + 1),
                    rect.x + 8.0,
                    mid_y + 6.0,
                    tp(&font, 18, rank_col),
                );

                // Text Y-coordinate for the first line
                let text_y = rect.y + 20.0;

                // Color dot
                draw_circle(rect.x + 40.0, text_y - 4.0, 5.0, snake.color);
                draw_circle_lines(
                    rect.x + 40.0,
                    text_y - 4.0,
                    5.0,
                    1.0,
                    Color::new(1.0, 1.0, 1.0, 0.25),
                );

                // Snake name
                let name = snake.name.clone();
                let name_col = if is_sel {
                    WHITE
                } else {
                    Color::new(0.82, 0.88, 1.0, 0.90)
                };
                draw_text_ex(&name, rect.x + 50.0, text_y, tp(&font, 16, name_col));

                // Health (length) number
                let len_str = format!("{}", snake.segments.len());
                draw_text_ex(
                    &len_str,
                    rect.x + 115.0,
                    text_y,
                    tp(&font, 16, Color::new(0.70, 0.75, 0.95, 0.80)),
                );

                let get_name = |id| snakes.iter().find(|s| s.id == id).map(|s| s.name.clone()).unwrap_or_else(|| "?".to_string());
                
                // State label (feed or target id)
                let (st_label, st_col) = match snake.state {
                    SnakeState::Hunting { target_id, .. } => (
                        format!("(HUNT {})", get_name(target_id)),
                        Color::new(1.0, 0.35, 0.35, 0.85),
                    ),
                    SnakeState::Fleeing { threat_id } => (
                        format!("(FLEE {})", get_name(threat_id)),
                        Color::new(0.35, 0.65, 1.0, 0.85),
                    ),
                    SnakeState::Foraging => {
                        ("(FEED)".to_string(), Color::new(0.55, 0.90, 0.55, 0.75))
                    }
                };
                draw_text_ex(&st_label, rect.x + 155.0, text_y, tp(&font, 16, st_col));

                // Health bar
                let bar_x = rect.x + 8.0;
                let bar_y = rect.y + 30.0;
                let bar_w = PANEL_W - 16.0;
                let bar_h = 10.0;
                let fill = (snake.segments.len() as f32 / max_segs) * bar_w;
                draw_rectangle(
                    bar_x,
                    bar_y,
                    bar_w,
                    bar_h,
                    Color::new(0.15, 0.18, 0.30, 0.6),
                );
                draw_rectangle(
                    bar_x,
                    bar_y,
                    fill,
                    bar_h,
                    Color::new(snake.color.r, snake.color.g, snake.color.b, 0.85),
                );
                draw_rectangle_lines(
                    bar_x,
                    bar_y,
                    bar_w,
                    bar_h,
                    1.0,
                    Color::new(0.3, 0.4, 0.7, 0.3),
                );

                // Follow marker
                if is_sel {
                    draw_text_ex(
                        ">",
                        rect.x + PANEL_W - 14.0,
                        text_y,
                        tp(&font, 16, Color::new(0.45, 0.85, 1.0, 1.0)),
                    );
                }

                // Row divider
                if ri + 1 < display_ranks.len() {
                    draw_line(
                        rect.x + 4.0,
                        rect.y + ROW_H,
                        rect.x + PANEL_W - 4.0,
                        rect.y + ROW_H,
                        1.0,
                        Color::new(0.25, 0.32, 0.58, 0.22),
                    );
                }
            }

            // Show if followed snake is outside top MAX_RANKS
            if let Some(fid) = selected_id {
                let rank_pos = ranked
                    .iter()
                    .position(|&i| snakes.get(i).map(|s| s.id) == Some(fid));
                if rank_pos.is_none_or(|r| r >= MAX_RANKS)
                    && let Some(rp) = rank_pos
                {
                    let off_y = rows_start_y + row_rects.len() as f32 * ROW_H + 10.0;
                    draw_text_ex(
                        format!("Following rank #{}", rp + 1),
                        px + 12.0,
                        off_y + 14.0,
                        tp(&font, 22, Color::new(0.55, 0.65, 1.0, 0.75)),
                    );
                }
            }
        } // end show_ui

        // ── HUD bar ────────────────────────────────────────────────────────────
        let hud_w = px - PANEL_PAD;
        draw_rectangle(0.0, 0.0, hud_w, 44.0, Color::new(0.07, 0.09, 0.15, 0.80));
        draw_line(0.0, 44.0, hud_w, 44.0, 1.0, Color::new(0.3, 0.4, 0.7, 0.3));
        draw_rectangle(0.0, 44.0, hud_w, 4.0, Color::new(0.0, 0.0, 0.0, 0.3)); // Shadow

        let ui_btn_w = 120.0;
        let ui_btn_rect = Rect::new(hud_w - ui_btn_w - 12.0, 8.0, ui_btn_w, 28.0);
        draw_rectangle(
            ui_btn_rect.x,
            ui_btn_rect.y,
            ui_btn_rect.w,
            ui_btn_rect.h,
            Color::new(0.12, 0.18, 0.35, 0.85),
        );
        draw_rectangle_lines(
            ui_btn_rect.x,
            ui_btn_rect.y,
            ui_btn_rect.w,
            ui_btn_rect.h,
            1.0,
            Color::new(0.4, 0.65, 1.0, 0.5),
        );
        centered_text(
            if show_ui {
                "HIDE UI (H)"
            } else {
                "SHOW UI (H)"
            },
            &font,
            14,
            WHITE,
            ui_btn_rect,
        );

        let hunting = snakes
            .iter()
            .filter(|s| matches!(s.state, SnakeState::Hunting { .. }))
            .count();
        let fleeing = snakes
            .iter()
            .filter(|s| matches!(s.state, SnakeState::Fleeing { .. }))
            .count();
        draw_text_ex(
            format!(
                "BOTS {}   HUNT {}   FLEE {}   FOOD {}   FPS {}",
                snakes.len(),
                hunting,
                fleeing,
                foods.len(),
                get_fps()
            ),
            16.0,
            28.0,
            tp(&font, 18, Color::new(0.72, 0.82, 1.0, 0.88)),
        );

        // ── Following label (bottom center) ───────────────────────────────────
        if cam_mode == CamMode::Follow {
            if let Some(fidx) = follow_idx {
                let snake = &snakes[fidx];
                let rank_pos = ranked
                    .iter()
                    .position(|&i| i == fidx)
                    .map(|r| r + 1)
                    .unwrap_or(0);

                let get_name = |id| snakes.iter().find(|s| s.id == id).map(|s| s.name.clone()).unwrap_or_else(|| "?".to_string());

                let state_str = match snake.state {
                    SnakeState::Hunting { target_id, .. } => {
                        format!("  |  HUNTING {}", get_name(target_id))
                    }
                    SnakeState::Fleeing { threat_id } => format!("  |  FLEEING {}", get_name(threat_id)),
                    SnakeState::Foraging => "  |  FEEDING".to_string(),
                };

                let label = format!(
                    "FOLLOWING {}  |  RANK #{}  |  LEN {}{}",
                    snake.name,
                    rank_pos,
                    snake.segments.len(),
                    state_str
                );

                let d = measure_text(&label, Some(&font), 18, 1.0);
                let lx = (px - d.width) / 2.0;
                let ly = sh - 20.0;

                let box_x = lx - 16.0;
                let box_y = ly - 20.0;
                let box_w = d.width + 32.0;
                let box_h = 32.0;

                // Shadow
                draw_rectangle(
                    box_x + 2.0,
                    box_y + 2.0,
                    box_w,
                    box_h,
                    Color::new(0.0, 0.0, 0.0, 0.5),
                );
                // Box
                draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.08, 0.1, 0.16, 0.9));
                // Border
                draw_rectangle_lines(
                    box_x,
                    box_y,
                    box_w,
                    box_h,
                    1.0,
                    Color::new(0.3, 0.4, 0.7, 0.5),
                );

                draw_text_ex(
                    &label,
                    lx,
                    ly,
                    tp(&font, 18, Color::new(0.85, 0.92, 1.0, 0.95)),
                );
            }
        } else {
            let label = "FULL MAP VIEW";
            let d = measure_text(label, Some(&font), 18, 1.0);
            let lx = (px - d.width) / 2.0;
            let ly = sh - 20.0;

            let box_x = lx - 16.0;
            let box_y = ly - 20.0;
            let box_w = d.width + 32.0;
            let box_h = 32.0;

            // Shadow
            draw_rectangle(
                box_x + 2.0,
                box_y + 2.0,
                box_w,
                box_h,
                Color::new(0.0, 0.0, 0.0, 0.5),
            );
            // Box
            draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.08, 0.1, 0.16, 0.9));
            // Border
            draw_rectangle_lines(
                box_x,
                box_y,
                box_w,
                box_h,
                1.0,
                Color::new(0.3, 0.4, 0.7, 0.5),
            );

            draw_text_ex(
                label,
                lx,
                ly,
                tp(&font, 18, Color::new(0.85, 0.92, 1.0, 0.95)),
            );
        }

        // ── Notifications ──────────────────────────────────────────────────────
        let mut n_y = 60.0; // Start a bit below the top
        for n in &mut notifications {
            if n.timer > 0.0 {
                n.timer -= dt;
                
                // Fade out effect
                let alpha = (n.timer / 1.0).min(1.0);
                
                // Slide up effect on fade out
                let slide_y = if n.timer < 0.5 { (0.5 - n.timer) * 40.0 } else { 0.0 };
                
                let mut col = n.color;
                col.a = alpha;
                
                let text_dims = measure_text(&n.text, Some(&font), 20, 1.0);
                let box_w = text_dims.width + 32.0;
                let box_h = 36.0;
                let box_x = (sw - box_w) / 2.0;
                let final_y = n_y - slide_y;
                
                draw_rectangle(box_x, final_y, box_w, box_h, Color::new(0.05, 0.05, 0.1, 0.8 * alpha));
                draw_rectangle_lines(box_x, final_y, box_w, box_h, 1.0, col);
                
                draw_text_ex(
                    &n.text,
                    box_x + 16.0,
                    final_y + 24.0,
                    tp(&font, 20, col),
                );
                
                n_y += box_h + 8.0;
            }
        }
        notifications.retain(|n| n.timer > 0.0);

        // ── Input handling ─────────────────────────────────────────────────────
        if is_key_pressed(KeyCode::H) || is_key_pressed(KeyCode::Tab) {
            show_ui = !show_ui;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mp = vec2(mouse_position().0, mouse_position().1);

            if ui_btn_rect.contains(mp) {
                show_ui = !show_ui;
            }

            if show_ui {
                // Toggle button
                if btn_rect.contains(mp) {
                    cam_mode = if cam_mode == CamMode::FullMap {
                        CamMode::Follow
                    } else {
                        CamMode::FullMap
                    };
                }

                // Leaderboard rows → select + follow
                for &(rect, sid) in &row_rects {
                    if rect.contains(mp) {
                        selected_id = Some(sid);
                        cam_mode = CamMode::Follow;
                        break;
                    }
                }
            }
        }

        next_frame().await;
    }
}
