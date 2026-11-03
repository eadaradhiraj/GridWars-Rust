use macroquad::prelude::*;
use crate::colors::*;
use crate::grid::Grid;
use crate::particles::ParticleSystem;
use crate::shots::ShotManager;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyKind {
    Pinwheel,        // Kind 2 (pink)
    SeekerDiamond,   // Kind 0 (cyan)
    WandererSquare,  // Kind 1 (green)
    BlackHole,       // Kind 5 (red / gravity well)
    Clone,           // Kind 4 (red grunt)
    Snake,           // Kind 7 (magenta head + tail)
    ShieldedTriangle,// Kind 8 (orange / shield)
    Butterfly,       // Kind 9 (blue weaver)
    SplitterCube,    // Kind 6 (purple)
    Generator,       // Kind 10 (spawner)
}

pub struct SnakeSegment {
    pub x: f32,
    pub y: f32,
}

pub struct Enemy {
    pub kind: EnemyKind,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub rot: f32,
    pub rot_speed: f32,
    pub radius: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub points: i32,
    pub freeze_timer: i32,
    pub timer: f32,
    pub alive: bool,
    pub segments: Vec<SnakeSegment>, // for Snake
    pub cube_stage: usize,           // 0 = big, 1 = med, 2 = small
    pub spawn_rate: i32,             // for Generator
    pub spawn_timer: i32,
    pub spawn_kind: usize,
}

pub struct FloatingText {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub dy: f32,
    pub life: f32,
    pub max_life: f32,
    pub color: Color,
}

pub struct Powerup {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub kind: usize, // 0: ExtraLife, 1: ExtraBomb, 2: TripleShot, 3: SideShot, 4: SuperShot, 5: BouncyShot, 6: Shield
    pub life: f32,
    pub alive: bool,
}

pub struct EnemyManager {
    pub enemies: Vec<Enemy>,
    pub floating_texts: Vec<FloatingText>,
    pub powerups: Vec<Powerup>,
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: Vec::with_capacity(512),
            floating_texts: Vec::with_capacity(128),
            powerups: Vec::with_capacity(32),
        }
    }

    pub fn clear(&mut self) {
        self.enemies.clear();
        self.floating_texts.clear();
        self.powerups.clear();
    }

    pub fn add_floating_text(&mut self, text: String, x: f32, y: f32, color: Color) {
        if self.floating_texts.len() < 128 {
            self.floating_texts.push(FloatingText {
                text,
                x,
                y,
                dy: -1.0,
                life: 45.0,
                max_life: 45.0,
                color,
            });
        }
    }

    pub fn spawn_powerup(&mut self, x: f32, y: f32, kind: usize) {
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let speed = rand::gen_range(0.5, 1.5);
        self.powerups.push(Powerup {
            x,
            y,
            dx: angle.cos() * speed,
            dy: angle.sin() * speed,
            kind,
            life: 600.0, // 10 seconds at 60 FPS
            alive: true,
        });
    }

    pub fn spawn_enemy(&mut self, kind: EnemyKind, x: f32, y: f32, freeze: i32) {
        let mut segments = Vec::new();
        let (hp, points, radius, rot_speed) = match kind {
            EnemyKind::Pinwheel => (1, 25, 14.0, 0.08),
            EnemyKind::SeekerDiamond => (1, 50, 14.0, 0.04),
            EnemyKind::WandererSquare => (1, 100, 16.0, 0.02),
            EnemyKind::BlackHole => (24, 250, 24.0, 0.05),
            EnemyKind::Clone => (1, 10, 10.0, 0.03),
            EnemyKind::Snake => {
                for _ in 0..16 {
                    segments.push(SnakeSegment { x, y });
                }
                (1, 250, 16.0, 0.05)
            }
            EnemyKind::ShieldedTriangle => (1, 150, 14.0, 0.03),
            EnemyKind::Butterfly => (1, 100, 12.0, 0.05),
            EnemyKind::SplitterCube => (1, 50, 16.0, 0.03),
            EnemyKind::Generator => (8, 200, 22.0, 0.02),
        };

        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let speed = rand::gen_range(1.0, 2.5);

        self.enemies.push(Enemy {
            kind,
            x,
            y,
            dx: angle.cos() * speed,
            dy: angle.sin() * speed,
            rot: angle,
            rot_speed,
            radius,
            hp,
            max_hp: hp,
            points,
            freeze_timer: freeze,
            timer: 0.0,
            alive: true,
            segments,
            cube_stage: 0,
            spawn_rate: 180,
            spawn_timer: 0,
            spawn_kind: 0,
        });
    }

    pub fn update(
        &mut self,
        px: f32,
        py: f32,
        width: f32,
        height: f32,
        grid: &mut Grid,
        shots: &mut ShotManager,
        particles: &mut ParticleSystem,
        audio: &crate::audio::AudioManager,
        speed_mult: f32,
    ) -> (i32, i32) {
        let mut earned_score = 0;
        let mut kills = 0;
        let mut new_spawns = Vec::new();

        // Update floating texts
        self.floating_texts.retain_mut(|t| {
            t.y += t.dy;
            t.life -= 1.0;
            t.life > 0.0
        });

        // Update powerups
        for pu in &mut self.powerups {
            pu.x += pu.dx;
            pu.y += pu.dy;
            if pu.x < 20.0 || pu.x > width - 20.0 {
                pu.dx = -pu.dx;
            }
            if pu.y < 20.0 || pu.y > height - 20.0 {
                pu.dy = -pu.dy;
            }
            pu.life -= 1.0;
            if pu.life <= 0.0 {
                pu.alive = false;
            }
        }
        self.powerups.retain(|pu| pu.alive);

        // Update enemies
        for e in &mut self.enemies {
            if !e.alive {
                continue;
            }

            if e.freeze_timer > 0 {
                e.freeze_timer -= 1;
                continue;
            }

            e.timer += 0.05;
            e.rot += e.rot_speed;

            match e.kind {
                EnemyKind::Pinwheel => {
                    e.x += e.dx * speed_mult;
                    e.y += e.dy * speed_mult;
                    if e.x < e.radius || e.x > width - e.radius {
                        e.dx = -e.dx;
                    }
                    if e.y < e.radius || e.y > height - e.radius {
                        e.dy = -e.dy;
                    }
                }
                EnemyKind::SeekerDiamond => {
                    let to_px = px - e.x;
                    let to_py = py - e.y;
                    let dist = (to_px * to_px + to_py * to_py).sqrt();
                    if dist > 1.0 {
                        let target_dx = (to_px / dist) * 2.8 * speed_mult;
                        let target_dy = (to_py / dist) * 2.8 * speed_mult;
                        e.dx += (target_dx - e.dx) * 0.05;
                        e.dy += (target_dy - e.dy) * 0.05;
                    }
                    e.x += e.dx;
                    e.y += e.dy;
                }
                EnemyKind::WandererSquare => {
                    e.x += e.dx * 1.8 * speed_mult;
                    e.y += e.dy * 1.8 * speed_mult;
                    if e.x < e.radius || e.x > width - e.radius {
                        e.dx = -e.dx;
                    }
                    if e.y < e.radius || e.y > height - e.radius {
                        e.dy = -e.dy;
                    }
                }
                EnemyKind::BlackHole => {
                    // Massive gravitational pull on the elastic grid!
                    grid.pull(e.x, e.y, 160.0, 1.8);

                    // Pull nearby shots towards the black hole
                    for s in &mut shots.shots {
                        let diff_x = e.x - s.x;
                        let diff_y = e.y - s.y;
                        let d2 = diff_x * diff_x + diff_y * diff_y;
                        if d2 < 180.0 * 180.0 && d2 > 10.0 {
                            let d = d2.sqrt();
                            let pull_force = (1.0 - d / 180.0) * 1.2;
                            s.dx += (diff_x / d) * pull_force;
                            s.dy += (diff_y / d) * pull_force;
                            // Absorb shot if too close
                            if d < e.radius * 0.8 {
                                s.alive = false;
                                particles.burst(s.x, s.y, 3, COL_SUN, 2.0, 12.0);
                                e.hp -= 1;
                                if e.hp <= 0 {
                                    e.alive = false;
                                }
                            }
                        }
                    }

                    // Accretion disk particles
                    if rand::gen_range(0, 3) == 0 {
                        let p_ang = rand::gen_range(0.0, std::f32::consts::TAU);
                        let p_dist = rand::gen_range(e.radius * 1.5, e.radius * 2.5);
                        let px_pos = e.x + p_ang.cos() * p_dist;
                        let py_pos = e.y + p_ang.sin() * p_dist;
                        let p_dx = -p_ang.sin() * 2.5 - p_ang.cos() * 1.2;
                        let p_dy = p_ang.cos() * 2.5 - p_ang.sin() * 1.2;
                        particles.spawn(px_pos, py_pos, p_dx, p_dy, COL_SUN, 25.0, 2.0);
                    }
                }
                EnemyKind::Clone => {
                    let to_px = px - e.x;
                    let to_py = py - e.y;
                    let dist = (to_px * to_px + to_py * to_py).sqrt();
                    if dist > 1.0 {
                        e.dx = (to_px / dist) * 1.2 * speed_mult;
                        e.dy = (to_py / dist) * 1.2 * speed_mult;
                    }
                    e.x += e.dx;
                    e.y += e.dy;
                }
                EnemyKind::Snake => {
                    let to_px = px - e.x;
                    let to_py = py - e.y;
                    let dist = (to_px * to_px + to_py * to_py).sqrt();
                    if dist > 1.0 {
                        let target_dx = (to_px / dist) * 2.2 * speed_mult;
                        let target_dy = (to_py / dist) * 2.2 * speed_mult;
                        e.dx += (target_dx - e.dx) * 0.08;
                        e.dy += (target_dy - e.dy) * 0.08;
                    }
                    e.x += e.dx;
                    e.y += e.dy;

                    // Update trailing snake segments
                    let mut prev_x = e.x;
                    let mut prev_y = e.y;
                    for seg in &mut e.segments {
                        let seg_dx = prev_x - seg.x;
                        let seg_dy = prev_y - seg.y;
                        let seg_dist = (seg_dx * seg_dx + seg_dy * seg_dy).sqrt();
                        if seg_dist > 12.0 {
                            seg.x += (seg_dx / seg_dist) * (seg_dist - 12.0);
                            seg.y += (seg_dy / seg_dist) * (seg_dist - 12.0);
                        }
                        prev_x = seg.x;
                        prev_y = seg.y;
                    }
                }
                EnemyKind::ShieldedTriangle => {
                    let to_px = px - e.x;
                    let to_py = py - e.y;
                    let dist = (to_px * to_px + to_py * to_py).sqrt();
                    if dist > 1.0 {
                        let target_dx = (to_px / dist) * 2.0 * speed_mult;
                        let target_dy = (to_py / dist) * 2.0 * speed_mult;
                        e.dx += (target_dx - e.dx) * 0.04;
                        e.dy += (target_dy - e.dy) * 0.04;
                    }
                    e.x += e.dx;
                    e.y += e.dy;
                    // Face moving direction
                    e.rot = e.dy.atan2(e.dx) + std::f32::consts::FRAC_PI_2;
                }
                EnemyKind::Butterfly => {
                    // Weaving evasive motion
                    let weave = e.timer.sin() * 2.5;
                    let to_px = px - e.x;
                    let to_py = py - e.y;
                    let dist = (to_px * to_px + to_py * to_py).sqrt().max(1.0);
                    let norm_x = to_px / dist;
                    let norm_y = to_py / dist;
                    e.dx = (norm_x - norm_y * weave) * 2.2 * speed_mult;
                    e.dy = (norm_y + norm_x * weave) * 2.2 * speed_mult;
                    e.x += e.dx;
                    e.y += e.dy;
                }
                EnemyKind::SplitterCube => {
                    e.x += e.dx * 1.5 * speed_mult;
                    e.y += e.dy * 1.5 * speed_mult;
                    if e.x < e.radius || e.x > width - e.radius {
                        e.dx = -e.dx;
                    }
                    if e.y < e.radius || e.y > height - e.radius {
                        e.dy = -e.dy;
                    }
                }
                EnemyKind::Generator => {
                    e.spawn_timer += 1;
                    if e.spawn_timer >= e.spawn_rate {
                        e.spawn_timer = 0;
                        new_spawns.push((EnemyKind::Pinwheel, e.x, e.y, 20));
                    }
                }
            }

            // Keep within borders
            e.x = e.x.clamp(e.radius, width - e.radius);
            e.y = e.y.clamp(e.radius, height - e.radius);

            // Pull grid slightly around moving enemies
            grid.pull(e.x, e.y, e.radius * 2.0, 0.4);

            // Check collision with player bullets
            for s in &mut shots.shots {
                if !s.alive {
                    continue;
                }
                let diff_x = s.x - e.x;
                let diff_y = s.y - e.y;
                let dist2 = diff_x * diff_x + diff_y * diff_y;
                let hit_r = e.radius + 4.0;

                if dist2 < hit_r * hit_r {
                    // Check ShieldedTriangle front shield
                    if e.kind == EnemyKind::ShieldedTriangle {
                        let shot_rel_x = s.x - e.x;
                        let shot_rel_y = s.y - e.y;
                        let dot = shot_rel_x * e.dx + shot_rel_y * e.dy;
                        if dot > 0.0 {
                            // Hit the front shield - deflected!
                            s.alive = false;
                            particles.burst(s.x, s.y, 6, COL_TRIANGLE, 4.0, 15.0);
                            audio._play_shield();
                            continue;
                        }
                    }

                    if !s.is_super {
                        s.alive = false;
                    }

                    e.hp -= 1;
                    if e.kind == EnemyKind::BlackHole {
                        audio._play_sun_hit();
                    } else {
                        audio._play_pop();
                    }
                    particles.burst(s.x, s.y, 5, COL_PLAYER, 3.0, 12.0);

                    if e.hp <= 0 {
                        e.alive = false;
                        earned_score += e.points;
                        kills += 1;

                        // Explosion particles and grid shockwave
                        let color = match e.kind {
                            EnemyKind::Pinwheel => COL_PIN,
                            EnemyKind::SeekerDiamond => COL_DIAMOND,
                            EnemyKind::WandererSquare => COL_SQUARE,
                            EnemyKind::BlackHole => COL_SUN,
                            EnemyKind::Clone => COL_CLONE,
                            EnemyKind::Snake => COL_SNAKE,
                            EnemyKind::ShieldedTriangle => COL_TRIANGLE,
                            EnemyKind::Butterfly => COL_BUTTER,
                            EnemyKind::SplitterCube => COL_CUBE,
                            EnemyKind::Generator => COL_POWERUP,
                        };
                        particles.burst(e.x, e.y, 25, color, 6.0, 30.0);
                        grid.push(e.x, e.y, 60.0, 8.0);

                        // Cube splitting logic
                        if e.kind == EnemyKind::SplitterCube && e.cube_stage < 2 {
                            let _next_stage = e.cube_stage + 1;
                            new_spawns.push((EnemyKind::SplitterCube, e.x - 8.0, e.y - 8.0, 0));
                            new_spawns.push((EnemyKind::SplitterCube, e.x + 8.0, e.y + 8.0, 0));
                        }

                        // Chance to drop powerup
                        if rand::gen_range(0, 35) == 0 {
                            let p_kind = rand::gen_range(0, 7);
                            self.powerups.push(Powerup {
                                x: e.x,
                                y: e.y,
                                dx: rand::gen_range(-1.0, 1.0),
                                dy: rand::gen_range(-1.0, 1.0),
                                kind: p_kind,
                                life: 600.0,
                                alive: true,
                            });
                        }

                        break;
                    }
                }
            }
        }

        // Spawn queued new enemies (e.g. from generators or splitters)
        for (kind, x, y, freeze) in new_spawns {
            self.spawn_enemy(kind, x, y, freeze);
        }

        self.enemies.retain(|e| e.alive);

        (earned_score, kills)
    }

    pub fn destroy_all(&mut self, particles: &mut ParticleSystem, grid: &mut Grid) -> (i32, i32) {
        let mut score = 0;
        let mut kills = 0;
        for e in &mut self.enemies {
            e.alive = false;
            score += e.points;
            kills += 1;
            particles.burst(e.x, e.y, 20, COL_BOMB, 8.0, 35.0);
        }
        grid.push(grid.width * 0.5, grid.height * 0.5, 500.0, 30.0);
        self.enemies.clear();
        (score, kills)
    }

    pub fn draw(&self, textures: &EnemyTextures) {
        // Draw powerups
        for pu in &self.powerups {
            let label = match pu.kind {
                0 => "1UP",
                1 => "BOMB",
                2 => "3-WAY",
                3 => "SIDE",
                4 => "SUPER",
                5 => "BOUNCE",
                _ => "SHIELD",
            };
            let pulse = (pu.life * 0.1).sin() * 2.0;
            draw_circle_lines(pu.x, pu.y, 14.0 + pulse, 2.0, COL_POWERUP);
            draw_circle(pu.x, pu.y, 8.0, Color::new(1.0, 1.0, 0.2, 0.4));
            crate::vector_font::draw_string_centered(label, pu.x, pu.y - 3.0, 1.2, COL_POWERUP, 1.5);
        }

        // Draw enemies
        for e in &self.enemies {
            match e.kind {
                EnemyKind::Pinwheel => {
                    if let Some(tex) = &textures.pinwheel {
                        draw_texture_ex(
                            tex,
                            e.x - tex.width() * 0.5,
                            e.y - tex.height() * 0.5,
                            COL_PIN,
                            DrawTextureParams {
                                rotation: e.rot,
                                ..Default::default()
                            },
                        );
                    } else {
                        // Vector geometric fallback
                        let r = e.radius;
                        for i in 0..4 {
                            let a = e.rot + (i as f32) * std::f32::consts::FRAC_PI_2;
                            draw_line(e.x, e.y, e.x + a.cos() * r, e.y + a.sin() * r, 2.0, COL_PIN);
                            draw_line(
                                e.x + a.cos() * r,
                                e.y + a.sin() * r,
                                e.x + (a + 0.5).cos() * (r * 0.6),
                                e.y + (a + 0.5).sin() * (r * 0.6),
                                2.0,
                                COL_PIN,
                            );
                        }
                    }
                }
                EnemyKind::SeekerDiamond => {
                    if let Some(tex) = &textures.diamond {
                        draw_texture_ex(
                            tex,
                            e.x - tex.width() * 0.5,
                            e.y - tex.height() * 0.5,
                            COL_DIAMOND,
                            DrawTextureParams {
                                rotation: e.rot,
                                ..Default::default()
                            },
                        );
                    } else {
                        let r = e.radius;
                        let p1 = Vec2::new(e.x, e.y - r);
                        let p2 = Vec2::new(e.x + r * 0.7, e.y);
                        let p3 = Vec2::new(e.x, e.y + r);
                        let p4 = Vec2::new(e.x - r * 0.7, e.y);
                        draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, COL_DIAMOND);
                        draw_line(p2.x, p2.y, p3.x, p3.y, 2.0, COL_DIAMOND);
                        draw_line(p3.x, p3.y, p4.x, p4.y, 2.0, COL_DIAMOND);
                        draw_line(p4.x, p4.y, p1.x, p1.y, 2.0, COL_DIAMOND);
                    }
                }
                EnemyKind::WandererSquare => {
                    if let Some(tex) = &textures.square {
                        draw_texture_ex(
                            tex,
                            e.x - tex.width() * 0.5,
                            e.y - tex.height() * 0.5,
                            COL_SQUARE,
                            DrawTextureParams {
                                rotation: e.rot,
                                ..Default::default()
                            },
                        );
                    } else {
                        let r = e.radius;
                        draw_rectangle_lines(e.x - r * 0.5, e.y - r * 0.5, r, r, 2.0, COL_SQUARE);
                    }
                }
                EnemyKind::BlackHole => {
                    if let Some(tex) = &textures.black_hole {
                        draw_texture_ex(
                            tex,
                            e.x - tex.width() * 0.5,
                            e.y - tex.height() * 0.5,
                            COL_SUN,
                            DrawTextureParams {
                                rotation: e.rot,
                                ..Default::default()
                            },
                        );
                    } else {
                        draw_circle_lines(e.x, e.y, e.radius, 3.0, COL_SUN);
                        draw_circle(e.x, e.y, e.radius * 0.8, Color::new(0.1, 0.0, 0.0, 0.8));
                    }
                }
                EnemyKind::Clone => {
                    if let Some(tex) = &textures.clone_grunt {
                        draw_texture_ex(
                            tex,
                            e.x - tex.width() * 0.5,
                            e.y - tex.height() * 0.5,
                            COL_CLONE,
                            DrawTextureParams {
                                rotation: e.rot,
                                ..Default::default()
                            },
                        );
                    } else {
                        draw_circle_lines(e.x, e.y, e.radius, 2.0, COL_CLONE);
                        draw_circle(e.x, e.y, e.radius * 0.5, COL_CLONE);
                    }
                }
                EnemyKind::Snake => {
                    // Draw tail segments
                    for (i, seg) in e.segments.iter().enumerate() {
                        let seg_r = (14.0 - (i as f32) * 0.6).max(4.0);
                        let alpha = 1.0 - (i as f32) * 0.04;
                        let col = Color::new(COL_TAIL.r, COL_TAIL.g, COL_TAIL.b, alpha);
                        draw_circle_lines(seg.x, seg.y, seg_r, 1.5, col);
                    }
                    // Head
                    if let Some(tex) = &textures.snake_head {
                        draw_texture_ex(
                            tex,
                            e.x - tex.width() * 0.5,
                            e.y - tex.height() * 0.5,
                            COL_SNAKE,
                            DrawTextureParams {
                                rotation: e.dy.atan2(e.dx),
                                ..Default::default()
                            },
                        );
                    } else {
                        draw_circle_lines(e.x, e.y, e.radius, 2.5, COL_SNAKE);
                    }
                }
                EnemyKind::ShieldedTriangle => {
                    if let Some(tex) = &textures.shield_triangle {
                        draw_texture_ex(
                            tex,
                            e.x - tex.width() * 0.5,
                            e.y - tex.height() * 0.5,
                            COL_TRIANGLE,
                            DrawTextureParams {
                                rotation: e.rot,
                                ..Default::default()
                            },
                        );
                    } else {
                        let r = e.radius;
                        let front = Vec2::new(e.x + e.dx.signum() * r, e.y + e.dy.signum() * r);
                        draw_circle_lines(e.x, e.y, r, 2.0, COL_TRIANGLE);
                        // Glowing front shield arc
                        draw_circle_lines(front.x, front.y, 8.0, 3.0, Color::new(0.2, 0.8, 1.0, 1.0));
                    }
                }
                EnemyKind::Butterfly => {
                    if let Some(tex) = &textures.butterfly {
                        draw_texture_ex(
                            tex,
                            e.x - tex.width() * 0.5,
                            e.y - tex.height() * 0.5,
                            COL_BUTTER,
                            DrawTextureParams {
                                rotation: e.rot,
                                ..Default::default()
                            },
                        );
                    } else {
                        draw_circle_lines(e.x, e.y, e.radius, 2.0, COL_BUTTER);
                    }
                }
                EnemyKind::SplitterCube => {
                    let r = e.radius * (1.0 - (e.cube_stage as f32) * 0.25);
                    draw_rectangle_lines(e.x - r * 0.5, e.y - r * 0.5, r, r, 2.0, COL_CUBE);
                }
                EnemyKind::Generator => {
                    draw_circle_lines(e.x, e.y, e.radius, 3.0, COL_POWERUP);
                    let pulse = (e.timer * 4.0).sin() * 4.0;
                    draw_circle_lines(e.x, e.y, e.radius * 0.6 + pulse, 1.5, COL_DIAMOND);
                }
            }
        }

        // Draw floating score text
        for t in &self.floating_texts {
            let alpha = (t.life / t.max_life).clamp(0.0, 1.0);
            let col = Color::new(t.color.r, t.color.g, t.color.b, alpha);
            crate::vector_font::draw_string_centered(&t.text, t.x, t.y, 1.5, col, 1.5);
        }
    }
}

pub struct EnemyTextures {
    pub pinwheel: Option<Texture2D>,
    pub diamond: Option<Texture2D>,
    pub square: Option<Texture2D>,
    pub black_hole: Option<Texture2D>,
    pub clone_grunt: Option<Texture2D>,
    pub snake_head: Option<Texture2D>,
    pub shield_triangle: Option<Texture2D>,
    pub butterfly: Option<Texture2D>,
}

async fn load_tex_safe(path: &str) -> Option<Texture2D> {
    load_texture(path).await.ok()
}

impl EnemyTextures {
    pub async fn load() -> Self {
        let pinwheel = load_tex_safe("gfx/high/pinkpinwheel.png").await;
        let diamond = load_tex_safe("gfx/high/bluediamond.png").await;
        let square = load_tex_safe("gfx/high/greensquare.png").await;
        let black_hole = load_tex_safe("gfx/high/redcircle.png").await;
        let clone_grunt = load_tex_safe("gfx/high/redclone.png").await;
        let snake_head = load_tex_safe("gfx/high/snakehead.png").await;
        let shield_triangle = load_tex_safe("gfx/high/orangetriangle.png").await;
        let butterfly = load_tex_safe("gfx/high/indigotriangle.png").await;

        Self {
            pinwheel,
            diamond,
            square,
            black_hole,
            clone_grunt,
            snake_head,
            shield_triangle,
            butterfly,
        }
    }
}
