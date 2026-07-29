use crate::constants::*;
use crate::grid::*;
use macroquad::prelude::*;

// ── Snake state ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum SnakeState {
    Foraging,
    Hunting { target_id: u32, orbit_side: f32 },
    Fleeing { threat_id: u32 },
}

pub enum FoodKind {
    Normal,
    Speed,
    Ghost,       // High points, rare
    Skin(usize), // Gives a skin ID
}

pub struct Food {
    pub pos: Vec2,
    pub color: Color,
    pub kind: FoodKind,
    pub visual_id: usize,
    /// None = permanent food. Some(t) = body food, expires when t reaches 0.
    pub lifetime: Option<(f32, f32)>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ObstacleKind {
    Stone,
    Pond,
}

pub struct Obstacle {
    pub pos: Vec2,
    pub radius: f32,
    pub kind: ObstacleKind,
    pub points: Vec<Vec2>, // Used for rendering the shape
}

// ── Snake ─────────────────────────────────────────────────────────────────────
pub struct Snake {
    pub id: u32,
    pub name: String,
    pub segments: Vec<Vec2>,
    pub path: std::collections::VecDeque<Vec2>,
    pub color: Color,
    pub skin_id: Option<usize>, // If Some, apply shader material
    pub speed_boost_timer: f32,
    pub angle: f32,
    pub base_speed: f32,
    pub dead: bool,
    pub forage_timer: f32,
    pub grow_queue: usize,
    pub state: SnakeState,
    pub glow_timer: f32,
    pub happy_timer: f32,
    pub hunt_cooldown: f32,
    pub skin_timer: f32,
}

impl Snake {
    pub fn radius(&self) -> f32 {
        let length = self.segments.len().max(10) as f32; // Default starting length is 10
        4.0 + length.powf(0.4) * 0.9
    }

    pub fn segment_length(&self) -> f32 {
        self.radius() * 1.25
    }

    pub fn new(x: f32, y: f32, length: usize, color: Color, id: u32) -> Self {
        let angle = macroquad::rand::gen_range(0.0, std::f32::consts::TAU);
        let mut segments = vec![];

        let initial_radius = 4.0 + (length.max(10) as f32).powf(0.4) * 0.9;
        let seg_len = initial_radius * 1.25;

        for i in 0..length {
            segments.push(vec2(
                x - i as f32 * seg_len * angle.cos(),
                y - i as f32 * seg_len * angle.sin(),
            ));
        }

        let mut path = std::collections::VecDeque::new();
        let path_points = length * 15;
        let path_step = seg_len / 15.0;
        for i in 0..path_points {
            path.push_back(vec2(
                x - i as f32 * path_step * angle.cos(),
                y - i as f32 * path_step * angle.sin(),
            ));
        }

        let name = crate::constants::DUMMY_NAMES
            [macroquad::rand::gen_range(0, crate::constants::DUMMY_NAMES.len())]
        .to_string();

        Self {
            id,
            name,
            segments,
            path,
            color,
            skin_id: None,
            speed_boost_timer: 0.0,
            angle,
            base_speed: SNAKE_SPEED * macroquad::rand::gen_range(0.9, 1.1),
            dead: false,
            forage_timer: 0.0,
            grow_queue: 0,
            state: SnakeState::Foraging,
            glow_timer: 0.0,
            happy_timer: 0.0,
            hunt_cooldown: 0.0,
            skin_timer: 0.0,
        }
    }

    pub fn ray_danger(
        origin: Vec2,
        dir: Vec2,
        grid: &SpatialGrid,
        all_segs: &[Vec<Vec2>],
        all_angles: &[f32],
        obstacles: &[Obstacle],
        my_idx: usize,
        w: f32,
        h: f32,
        my_radius: f32,
    ) -> f32 {
        let step = RAY_LENGTH / RAY_STEPS as f32;
        let mut worst = 0.0f32;
        let wm = 30.0;
        let danger_radius = my_radius * 2.5;
        for s in 1..=RAY_STEPS {
            let p = origin + dir * (s as f32 * step);
            if p.x < 0.0 || p.x > w || p.y < 0.0 || p.y > h {
                return 1.0;
            }
            let wx = (wm - p.x.min(wm)).max(0.0) + (p.x - (w - wm)).max(0.0);
            let wy = (wm - p.y.min(wm)).max(0.0) + (p.y - (h - wm)).max(0.0);
            worst = worst.max(((wx + wy) / wm).min(1.0));
            for (si, ki) in grid.query_radius(p, danger_radius) {
                let (si, ki) = (si as usize, ki as usize);
                if my_idx == si && ki < 4 {
                    continue;
                }
                if let Some(seg) = all_segs.get(si).and_then(|s| s.get(ki)) {
                    let dist = p.distance(*seg);
                    if dist < danger_radius {
                        let base_d = 1.0 - dist / danger_radius;
                        let mul = if ki == 0 && si != my_idx {
                            let fwd = vec2(all_angles[si].cos(), all_angles[si].sin());
                            let tow = (origin - *seg).normalize_or_zero();
                            1.0 + fwd.dot(tow).max(0.0) * 1.5
                        } else {
                            1.0
                        };
                        worst = worst.max((base_d * mul).min(1.0));
                    }
                }
            }
            // Also check obstacles
            for obs in obstacles {
                let dist = p.distance(obs.pos);
                if dist < obs.radius + my_radius * 1.5 {
                    let base_d = 1.0 - dist / (obs.radius + my_radius * 1.5);
                    worst = worst.max((base_d * 2.0).min(1.0));
                }
            }
            if worst >= 1.0 {
                return 1.0;
            }
        }
        worst
    }

    pub fn best_safe_angle(
        head: Vec2,
        desired: Vec2,
        cur: f32,
        grid: &SpatialGrid,
        all_segs: &[Vec<Vec2>],
        all_angles: &[f32],
        obstacles: &[Obstacle],
        my_idx: usize,
        w: f32,
        h: f32,
        my_radius: f32,
    ) -> (f32, bool) {
        let mut best = cur;
        let mut best_s = f32::MAX;
        let mut blocked_rays = 0;

        for i in 0..RAY_COUNT {
            let ra = (i as f32 / RAY_COUNT as f32) * std::f32::consts::TAU;
            let dir = vec2(ra.cos(), ra.sin());
            let danger = Self::ray_danger(
                head, dir, grid, all_segs, all_angles, obstacles, my_idx, w, h, my_radius,
            );

            if danger > 0.4 {
                blocked_rays += 1;
            }

            let align = 1.0 - (dir.dot(desired) + 1.0) / 2.0;
            let mid = head + dir * (RAY_LENGTH * 0.6);
            let far = head + dir * RAY_LENGTH;
            let crowd =
                (grid.crowding(mid, GRID_CELL * 1.5) + grid.crowding(far, GRID_CELL * 1.5)) as f32;
            let score = danger * 5.0 + align + (crowd * 0.04).min(0.5);
            if score < best_s {
                best_s = score;
                best = ra;
            }
        }

        let panic = blocked_rays >= (RAY_COUNT * 5) / 8; // If more than ~60% of escape routes are blocked, panic!
        (best, panic)
    }

    pub fn update(
        &mut self,
        dt: f32,
        foods: &mut Vec<Food>,
        fg: &FoodGrid,
        w: f32,
        h: f32,
        grid: &SpatialGrid,
        all_segs: &[Vec<Vec2>],
        all_angles: &[f32],
        obstacles: &[Obstacle],
        all_ids: &[u32],
        my_idx: usize,
        is_leader: bool,
        rank: usize,
    ) -> bool {
        if self.dead || self.segments.is_empty() {
            return false;
        }

        self.glow_timer = (self.glow_timer - dt).max(0.0);
        self.happy_timer = (self.happy_timer - dt).max(0.0);
        self.hunt_cooldown = (self.hunt_cooldown - dt).max(0.0);
        self.speed_boost_timer = (self.speed_boost_timer - dt).max(0.0);

        if self.skin_timer > 0.0 {
            self.skin_timer -= dt;
            if self.skin_timer <= 0.0 {
                self.skin_id = None;
            }
        }

        if matches!(self.state, SnakeState::Foraging) {
            self.forage_timer += dt;
        } else {
            self.forage_timer = 0.0;
        }

        let head = self.segments[0];
        let my_len = self.segments.len();

        struct EI {
            idx: usize,
            dist: f32,
        }

        let current_target_idx = match self.state {
            SnakeState::Hunting { target_id, .. } => all_ids.iter().position(|&id| id == target_id),
            _ => None,
        };

        let mut smaller: Option<EI> = None;
        let mut larger: Option<EI> = None;
        let mut locked_prey: Option<EI> = None;

        for (j, segs) in all_segs.iter().enumerate() {
            if j == my_idx || segs.is_empty() {
                continue;
            }
            let d = head.distance(segs[0]);
            let el = segs.len();
            if el > my_len + SIZE_THREAT_MARGIN
                && d < THREAT_RANGE
                && larger.as_ref().is_none_or(|e: &EI| d < e.dist)
            {
                larger = Some(EI { idx: j, dist: d });
            }
            if el < my_len {
                let is_current = Some(j) == current_target_idx;

                // If it's a new target, we hunt it if it is smaller than us by the threat margin.
                let valid_prey = if is_current {
                    el < my_len
                } else {
                    el + SIZE_THREAT_MARGIN <= my_len && self.hunt_cooldown <= 0.0
                };

                if valid_prey {
                    if is_current {
                        // Lock onto this prey regardless of distance, so it doesn't give up until the opponent is dead
                        locked_prey = Some(EI { idx: j, dist: d });
                    } else if d < HUNT_RANGE && smaller.as_ref().is_none_or(|e: &EI| d < e.dist) {
                        smaller = Some(EI { idx: j, dist: d });
                    }
                }
            }
        }

        let chosen_prey = locked_prey.or(smaller);

        self.state = if let Some(ref t) = larger {
            SnakeState::Fleeing {
                threat_id: all_ids[t.idx],
            }
        } else if let Some(ref p) = chosen_prey {
            if my_len >= MIN_HUNT_LENGTH {
                let orbit_side = match self.state {
                    SnakeState::Hunting {
                        target_id,
                        orbit_side,
                    } if target_id == all_ids[p.idx] => orbit_side,
                    _ => {
                        let eh = all_segs[p.idx][0];
                        let ed = if all_segs[p.idx].len() > 1 {
                            (all_segs[p.idx][0] - all_segs[p.idx][1]).normalize_or_zero()
                        } else {
                            vec2(1.0, 0.0)
                        };
                        let tu = (head - eh).normalize_or_zero();
                        if ed.x * tu.y - ed.y * tu.x >= 0.0 {
                            1.0
                        } else {
                            -1.0
                        }
                    }
                };
                SnakeState::Hunting {
                    target_id: all_ids[p.idx],
                    orbit_side,
                }
            } else {
                SnakeState::Foraging
            }
        } else if rank <= 20 && self.forage_timer > 20.0 {
            // Force hunting if top 20 and idle for 20s
            let closest_smaller = all_segs
                .iter()
                .enumerate()
                .filter(|&(j, segs)| j != my_idx && !segs.is_empty() && segs.len() < my_len)
                .min_by(|&(_, a), &(_, b)| {
                    a[0].distance(head)
                        .partial_cmp(&b[0].distance(head))
                        .unwrap()
                });
            if let Some((j, _)) = closest_smaller {
                SnakeState::Hunting {
                    target_id: all_ids[j],
                    orbit_side: 1.0,
                }
            } else {
                SnakeState::Foraging
            }
        } else {
            SnakeState::Foraging
        };

        let desired: Vec2 = match self.state {
            SnakeState::Foraging => {
                let mut best_idx = None;
                let mut best_score = f32::MAX;

                // Only evaluate foods within an 800-pixel radius (squared to avoid sqrt).
                // This avoids calling `count_nearby` (and doing full loops) on thousands of far away foods!
                let max_dist_sq = 800.0 * 800.0;

                for (i, f) in foods.iter().enumerate() {
                    if is_leader && f.lifetime.is_none() {
                        continue;
                    }
                    let dist_sq = head.distance_squared(f.pos);
                    if dist_sq > max_dist_sq {
                        continue;
                    }

                    let true_dist = dist_sq.sqrt();
                    let cluster =
                        fg.count_nearby(f.pos, crate::constants::CLUSTER_BONUS_RADIUS) as f32;
                    let score = true_dist - cluster * crate::constants::CLUSTER_BONUS_WEIGHT;

                    if score < best_score {
                        best_score = score;
                        best_idx = Some(i);
                    }
                }

                if let Some(idx) = best_idx {
                    (foods[idx].pos - head).normalize_or_zero()
                } else {
                    vec2(self.angle.cos(), self.angle.sin())
                }
            }
            SnakeState::Hunting {
                target_id,
                orbit_side,
            } => {
                if let Some(target_idx) = all_ids.iter().position(|&id| id == target_id) {
                    let es = &all_segs[target_idx];
                    let eh = es[0];
                    let ev = if es.len() > 1 {
                        (es[0] - es[1]).normalize_or_zero()
                    } else {
                        vec2(1.0, 0.0)
                    };

                    // Dynamically calculate a safe circling radius so the big snake doesn't crash into the prey
                    let prey_radius = 4.0 + (es.len() as f32).powf(0.4) * 0.9;
                    let safe_r = self.radius() + prey_radius + 20.0;

                    // To completely surround the prey, we calculate the vector from the prey to us.
                    let te = head - eh;
                    let d = te.length().max(1.0);
                    let current_angle = te.y.atan2(te.x);

                    // We progressively shrink the orbit radius to squeeze the prey, down to the safe radius.
                    let target_r = (d - 25.0).clamp(safe_r, 400.0);

                    // Advance along the circle circumference (slower steps prevent cutting through the center)
                    let next_angle = current_angle + orbit_side * 1.1;

                    // Add a strong predictive lead based on prey's current velocity to cut them off
                    let lp = eh + ev * 120.0;

                    let ot = lp + vec2(next_angle.cos(), next_angle.sin()) * target_r;
                    (ot - head).normalize_or_zero()
                } else {
                    vec2(self.angle.cos(), self.angle.sin()) // Should never happen
                }
            }
            SnakeState::Fleeing { threat_id } => {
                if let Some(threat_idx) = all_ids.iter().position(|&id| id == threat_id) {
                    let ts = &all_segs[threat_idx];
                    let th = ts[0];
                    let tv = if ts.len() > 1 {
                        (ts[0] - ts[1]).normalize_or_zero() * SNAKE_SPEED
                    } else {
                        Vec2::ZERO
                    };
                    (head - (th + tv * PREDICT_SECS)).normalize_or_zero()
                } else {
                    vec2(self.angle.cos(), self.angle.sin()) // Should never happen
                }
            }
        };

        let (best_angle, panic) = Self::best_safe_angle(
            head,
            desired,
            self.angle,
            grid,
            all_segs,
            all_angles,
            obstacles,
            my_idx,
            w,
            h,
            self.radius(),
        );

        if panic {
            self.speed_boost_timer = self.speed_boost_timer.max(0.4); // Panic sprint!
        }

        let mut diff = best_angle - self.angle;
        while diff > std::f32::consts::PI {
            diff -= std::f32::consts::TAU;
        }
        while diff < -std::f32::consts::PI {
            diff += std::f32::consts::TAU;
        }

        let fwd = vec2(self.angle.cos(), self.angle.sin());
        let fd = Self::ray_danger(
            head,
            fwd,
            grid,
            all_segs,
            all_angles,
            obstacles,
            my_idx,
            w,
            h,
            self.radius(),
        );
        let mut speed = match self.state {
            SnakeState::Fleeing { .. } => self.base_speed * FLEE_SPEED_BOOST,
            SnakeState::Hunting { .. } => self.base_speed * crate::constants::HUNT_SPEED_BOOST,
            _ => self.base_speed,
        };
        if self.speed_boost_timer > 0.0 {
            speed *= 1.6; // Speed boost!
        }

        let urgency = match self.state {
            SnakeState::Fleeing { .. } => 2.2 + fd * 4.0,
            SnakeState::Hunting { .. } => 1.6 + fd * 3.0,
            SnakeState::Foraging => 1.0 + fd * 3.5,
        };

        let turn_rate = TURN_SPEED * urgency;
        let turn_step = turn_rate * dt;
        if diff.abs() < turn_step {
            self.angle = best_angle;
        } else {
            self.angle += turn_step * diff.signum();
        }
        let nh = head + vec2(self.angle.cos(), self.angle.sin()) * speed * dt;
        if nh.x < 0.0 || nh.x > w || nh.y < 0.0 || nh.y > h {
            self.dead = true;
            return false;
        }
        self.segments[0] = nh;

        self.path.push_front(nh);

        // Compute path distances and dynamically truncate the path so it never exceeds the snake's actual physical length.
        // This ensures new segments stack neatly at the tail instead of reaching into deep history.
        let mut path_dist = vec![0.0; self.path.len()];
        let slen = self.segment_length();
        let max_dist = self.segments.len() as f32 * slen;

        for i in 1..self.path.len() {
            path_dist[i] = path_dist[i - 1] + self.path[i - 1].distance(self.path[i]);
            if path_dist[i] > max_dist + slen {
                self.path.truncate(i + 1);
                path_dist.truncate(i + 1);
                break;
            }
        }
        let mut last_j = 1;
        for i in 1..self.segments.len() {
            let target_dist = i as f32 * slen;
            let mut found = false;
            for j in last_j..self.path.len() {
                if path_dist[j] >= target_dist {
                    let overshoot = path_dist[j] - target_dist;
                    let seg_len = path_dist[j] - path_dist[j - 1];
                    let t = if seg_len > 0.0 {
                        overshoot / seg_len
                    } else {
                        0.0
                    };
                    self.segments[i] = self.path[j].lerp(self.path[j - 1], t);
                    found = true;
                    last_j = j;
                    break;
                }
            }
            if !found && !self.path.is_empty() {
                self.segments[i] = *self.path.back().unwrap();
            }

            // Body obstacle physics: Push segments out of obstacles
            for obs in obstacles {
                let hit_r = obs.radius + self.radius();
                let seg = self.segments[i];
                match obs.kind {
                    ObstacleKind::Pond => {
                        if seg.distance(obs.pos) < hit_r {
                            let dir = (seg - obs.pos).normalize_or_zero();
                            self.segments[i] = obs.pos + dir * hit_r;
                        }
                    }
                    ObstacleKind::Stone => {
                        // For stones, we can just push away from the center for simplicity,
                        // but with a slightly larger radius to clear the jagged edges.
                        if seg.distance(obs.pos) < hit_r {
                            let dir = (seg - obs.pos).normalize_or_zero();
                            self.segments[i] = obs.pos + dir * hit_r;
                        }
                    }
                }
            }
        }

        let mut ate_food = false;
        let eat_r = self.radius() + FOOD_RADIUS;
        foods.retain(|f| {
            if self.segments[0].distance(f.pos) < eat_r {
                if is_leader && f.lifetime.is_none() {
                    // Leader can only eat dead bodies (which have a lifetime)
                    return true;
                }
                ate_food = true;
                match f.kind {
                    FoodKind::Normal => self.grow_queue += 1,
                    FoodKind::Speed => {
                        self.grow_queue += 2;
                        self.speed_boost_timer = 5.0; // We need to add this field to Snake
                    }
                    FoodKind::Ghost => {
                        self.grow_queue += 10;
                    }
                    FoodKind::Skin(id) => {
                        self.grow_queue += 5;
                        self.skin_id = Some(id);
                        self.skin_timer = 15.0;
                    }
                }
                false
            } else {
                true
            }
        });

        if self.grow_queue > 0 {
            // Spawn perfectly stacked on the tail. The dynamic path truncation
            // ensures it remains here until the head pulls forward, mimicking Slither.io precisely.
            let last = *self.segments.last().unwrap();
            self.segments.push(last);
            self.grow_queue -= 1;
        }
        ate_food
    }

    pub fn draw(&self, shaders: &[Material], is_leader: bool) {
        if self.dead || self.segments.is_empty() {
            return;
        }
        let n = self.segments.len();
        let glow_alpha = if self.glow_timer > 0.0 {
            (self.glow_timer / 5.0).min(1.0) * ((get_time() as f32 * 6.0).sin() * 0.2 + 0.3)
        } else {
            0.0
        };

        if let Some(id) = self.skin_id {
            if let Some(mat) = shaders.get(id) {
                mat.set_uniform("base_color", (self.color.r, self.color.g, self.color.b));
                gl_use_material(mat);
            }
        }

        for i in (0..n).rev() {
            let seg = self.segments[i];
            let t = i as f32 / n as f32;
            let r = self.radius() * (1.0 - t * 0.38).max(0.45);

            // Glow effect
            if glow_alpha > 0.0 {
                draw_circle(
                    seg.x,
                    seg.y,
                    r * 1.8,
                    Color::new(1.0, 0.85, 0.2, glow_alpha),
                );
            }

            // Leader red glow
            if is_leader {
                let time_pulse = (get_time() as f32 * 4.0).sin() * 0.15 + 0.25;
                draw_circle(
                    seg.x,
                    seg.y,
                    r * 1.6,
                    Color::new(1.0, 0.25, 0.25, time_pulse),
                );
            }
            let col = self.color;
            draw_circle(seg.x, seg.y, r + 1.5, Color::new(0.0, 0.0, 0.0, 0.30));
            draw_circle(seg.x, seg.y, r, col);
        }

        if self.skin_id.is_some() {
            gl_use_default_material();
        }
        let head = self.segments[0];

        // Leader horns
        if is_leader {
            let left_horn_base = head
                + vec2(self.angle.cos(), self.angle.sin()).rotate(vec2(0.0, -1.0))
                    * self.radius()
                    * 0.5;
            let right_horn_base = head
                + vec2(self.angle.cos(), self.angle.sin()).rotate(vec2(0.0, 1.0))
                    * self.radius()
                    * 0.5;
            let horn_dir = vec2(self.angle.cos(), self.angle.sin()).rotate(vec2(-0.5, 0.0));

            let p1_l = left_horn_base + horn_dir * 10.0;
            let p2_l = left_horn_base - horn_dir * 4.0;
            let tip_l = left_horn_base
                + vec2(self.angle.cos(), self.angle.sin()) * 12.0
                + vec2(self.angle.cos(), self.angle.sin()).rotate(vec2(0.0, -1.0)) * 4.0;

            let p1_r = right_horn_base + horn_dir * 10.0;
            let p2_r = right_horn_base - horn_dir * 4.0;
            let tip_r = right_horn_base
                + vec2(self.angle.cos(), self.angle.sin()) * 12.0
                + vec2(self.angle.cos(), self.angle.sin()).rotate(vec2(0.0, 1.0)) * 4.0;

            let horn_col = Color::new(0.9, 0.1, 0.1, 1.0);
            draw_triangle(p1_l, p2_l, tip_l, horn_col);
            draw_triangle(p1_r, p2_r, tip_r, horn_col);
        }

        // Happy effect
        if self.happy_timer > 0.0 {
            let p = (self.happy_timer * 8.0).sin().abs();
            let hx = head.x;
            let hy = head.y - 25.0 - p * 15.0; // Bouncing up and down above head
            let alpha = (self.happy_timer).min(1.0); // Fade out at the end
            let face_col = Color::new(1.0, 0.85, 0.2, alpha);
            let eye_col = Color::new(0.1, 0.1, 0.1, alpha);

            draw_circle(hx, hy, 10.0, face_col); // Yellow face
            draw_circle(hx - 3.5, hy - 2.0, 1.8, eye_col); // Left eye
            draw_circle(hx + 3.5, hy - 2.0, 1.8, eye_col); // Right eye

            // Big smile
            draw_circle(hx, hy + 2.5, 4.5, eye_col);
            draw_circle(hx, hy + 1.5, 4.5, face_col);
        }

        match self.state {
            SnakeState::Hunting { .. } => {
                let p = (get_time() as f32 * 5.0).sin() * 0.5 + 0.5;
                draw_circle(
                    head.x,
                    head.y,
                    self.radius() * 2.2 + p * 3.0,
                    Color::new(1.0, 0.15, 0.15, 0.22 * p),
                );
            }
            SnakeState::Fleeing { .. } => {
                let p = (get_time() as f32 * 8.0).sin() * 0.5 + 0.5;
                draw_circle(
                    head.x,
                    head.y,
                    self.radius() * 2.2 + p * 3.0,
                    Color::new(0.2, 0.5, 1.0, 0.30 * p),
                );
            }
            _ => {}
        }
        let ed = self.radius() * 0.6;
        let e1 = vec2((self.angle + 0.75).cos(), (self.angle + 0.75).sin()) * ed;
        let e2 = vec2((self.angle - 0.75).cos(), (self.angle - 0.75).sin()) * ed;
        draw_circle(head.x + e1.x, head.y + e1.y, self.radius() * 0.38, WHITE);
        draw_circle(head.x + e2.x, head.y + e2.y, self.radius() * 0.38, WHITE);
        let pu = vec2(self.angle.cos(), self.angle.sin()) * (self.radius() * 0.15);
        draw_circle(
            head.x + e1.x + pu.x,
            head.y + e1.y + pu.y,
            self.radius() * 0.20,
            BLACK,
        );
        draw_circle(
            head.x + e2.x + pu.x,
            head.y + e2.y + pu.y,
            self.radius() * 0.20,
            BLACK,
        );
    }
}
