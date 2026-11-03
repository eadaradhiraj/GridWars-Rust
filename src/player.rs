use macroquad::prelude::*;
use crate::colors::{COL_PLAYER, COL_TRAIL};
use crate::grid::Grid;
use crate::particles::ParticleSystem;
use crate::shots::ShotManager;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub rot: f32,
    pub lives: i32,
    pub bombs: i32,
    pub score: i32,
    pub multiplier: i32,
    pub kill_count: i32,
    pub shield_timer: f32,
    pub bomb_cooldown: f32,
    pub shot_cooldown: f32,
    pub triple_shot: f32,
    pub side_shot: f32,
    pub back_shot: f32,
    pub super_shot: f32,
    pub bouncy_shot: f32,
    pub dead: bool,
    pub respawn_timer: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            rot: 0.0,
            lives: 3,
            bombs: 3,
            score: 0,
            multiplier: 1,
            kill_count: 0,
            shield_timer: 180.0, // 3 seconds invulnerability
            bomb_cooldown: 0.0,
            shot_cooldown: 0.0,
            triple_shot: 0.0,
            side_shot: 0.0,
            back_shot: 0.0,
            super_shot: 0.0,
            bouncy_shot: 0.0,
            dead: false,
            respawn_timer: 0.0,
        }
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.dx = 0.0;
        self.dy = 0.0;
        self.rot = 0.0;
        self.lives = 3;
        self.bombs = 3;
        self.score = 0;
        self.multiplier = 1;
        self.kill_count = 0;
        self.shield_timer = 180.0;
        self.bomb_cooldown = 0.0;
        self.shot_cooldown = 0.0;
        self.triple_shot = 0.0;
        self.side_shot = 0.0;
        self.back_shot = 0.0;
        self.super_shot = 0.0;
        self.bouncy_shot = 0.0;
        self.dead = false;
        self.respawn_timer = 0.0;
    }

    pub fn respawn(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.dx = 0.0;
        self.dy = 0.0;
        self.dead = false;
        self.shield_timer = 180.0;
    }

    pub fn add_kills(&mut self, count: i32, pts: i32) -> bool {
        self.score += pts * self.multiplier;
        self.kill_count += count;

        let thresholds = [25, 100, 200, 400, 800, 1600, 2500, 3500, 5000, 5500];
        let mut new_mult = 1;
        for (i, &t) in thresholds.iter().enumerate() {
            if self.kill_count >= t {
                new_mult = (i as i32) + 2;
            }
        }

        let increased = new_mult > self.multiplier;
        self.multiplier = new_mult;
        increased
    }

    pub fn reset_multiplier(&mut self) {
        self.multiplier = 1;
        self.kill_count = 0;
    }

    pub fn handle_input(
        &mut self,
        shots: &mut ShotManager,
        grid: &mut Grid,
        particles: &mut ParticleSystem,
        width: f32,
        height: f32,
    ) -> (bool, bool) {
        // returns (bomb_activated, shot_fired)
        if self.dead {
            return (false, false);
        }

        if self.bomb_cooldown > 0.0 {
            self.bomb_cooldown -= 1.0;
        }
        if self.shot_cooldown > 0.0 {
            self.shot_cooldown -= 1.0;
        }

        // Weapon timers
        if self.triple_shot > 0.0 { self.triple_shot -= 1.0; }
        if self.side_shot > 0.0 { self.side_shot -= 1.0; }
        if self.back_shot > 0.0 { self.back_shot -= 1.0; }
        if self.super_shot > 0.0 { self.super_shot -= 1.0; }
        if self.bouncy_shot > 0.0 { self.bouncy_shot -= 1.0; }
        if self.shield_timer > 0.0 { self.shield_timer -= 1.0; }

        let move_x;
        let move_y;

        let wasd_x = (is_key_down(KeyCode::D) as i32 - is_key_down(KeyCode::A) as i32) as f32;
        let wasd_y = (is_key_down(KeyCode::S) as i32 - is_key_down(KeyCode::W) as i32) as f32;
        let arrow_x = (is_key_down(KeyCode::Right) as i32 - is_key_down(KeyCode::Left) as i32) as f32;
        let arrow_y = (is_key_down(KeyCode::Down) as i32 - is_key_down(KeyCode::Up) as i32) as f32;

        let mut aim_x = 0.0f32;
        let mut aim_y = 0.0f32;
        let mut want_to_fire = false;

        if wasd_x != 0.0 || wasd_y != 0.0 {
            // WASD moves
            move_x = wasd_x;
            move_y = wasd_y;

            // Arrow keys aim & shoot
            if arrow_x != 0.0 || arrow_y != 0.0 {
                aim_x = arrow_x;
                aim_y = arrow_y;
                want_to_fire = true;
            } else if is_key_down(KeyCode::Space) {
                // Space shoots in facing direction
                aim_x = self.rot.cos();
                aim_y = self.rot.sin();
                want_to_fire = true;
            }
        } else {
            // Arrow keys move!
            move_x = arrow_x;
            move_y = arrow_y;

            if is_key_down(KeyCode::Space) {
                aim_x = self.rot.cos();
                aim_y = self.rot.sin();
                want_to_fire = true;
            }
        }

        // Mouse aim / click support
        if is_mouse_button_down(MouseButton::Left) {
            let m_pos = mouse_position();
            let to_mx = m_pos.0 - self.x;
            let to_my = m_pos.1 - self.y;
            let m_dist = (to_mx * to_mx + to_my * to_my).sqrt();
            if m_dist > 1.0 {
                aim_x = to_mx / m_dist;
                aim_y = to_my / m_dist;
                want_to_fire = true;
            }
        }

        // Movement physics
        let move_len = (move_x * move_x + move_y * move_y).sqrt();
        if move_len > 0.001 {
            let norm_x = move_x / move_len;
            let norm_y = move_y / move_len;
            self.dx += norm_x * 1.4;
            self.dy += norm_y * 1.4;

            // Update rotation towards movement if not aiming separately
            if !want_to_fire {
                self.rot = norm_y.atan2(norm_x);
            }
        }

        // Apply friction
        self.dx *= 0.88;
        self.dy *= 0.88;

        self.x += self.dx;
        self.y += self.dy;

        // Keep inside screen
        let bound = 18.0;
        self.x = self.x.clamp(bound, width - bound);
        self.y = self.y.clamp(bound, height - bound);

        // Grid distortion behind player
        grid.pull(self.x, self.y, 45.0, 1.2);

        // Exhaust trail particles
        if move_len > 0.001 {
            let trail_ang = self.rot + std::f32::consts::PI + rand::gen_range(-0.3, 0.3);
            let trail_speed = rand::gen_range(1.5, 4.0);
            particles.spawn(
                self.x - self.rot.cos() * 10.0,
                self.y - self.rot.sin() * 10.0,
                trail_ang.cos() * trail_speed,
                trail_ang.sin() * trail_speed,
                COL_TRAIL,
                15.0,
                2.0,
            );
        }

        // Fire bullets
        let mut shot_fired = false;
        if want_to_fire {
            let aim_len = (aim_x * aim_x + aim_y * aim_y).sqrt();
            if aim_len > 0.001 {
                aim_x /= aim_len;
                aim_y /= aim_len;
                self.rot = aim_y.atan2(aim_x);

                if self.shot_cooldown <= 0.0 {
                    self.shot_cooldown = 4.0; // Fast firing rate
                    shot_fired = true;

                    let speed = 14.0;
                    let is_sup = self.super_shot > 0.0;
                    let is_bnc = self.bouncy_shot > 0.0;

                    // Forward shot
                    shots.spawn(self.x, self.y, aim_x * speed, aim_y * speed, speed, is_sup, is_bnc);

                    // Triple / spread shot
                    if self.triple_shot > 0.0 {
                        let ang1 = self.rot + 0.15;
                        let ang2 = self.rot - 0.15;
                        shots.spawn(self.x, self.y, ang1.cos() * speed, ang1.sin() * speed, speed, is_sup, is_bnc);
                        shots.spawn(self.x, self.y, ang2.cos() * speed, ang2.sin() * speed, speed, is_sup, is_bnc);
                    }

                    // Side shot
                    if self.side_shot > 0.0 {
                        let ang1 = self.rot + std::f32::consts::FRAC_PI_2;
                        let ang2 = self.rot - std::f32::consts::FRAC_PI_2;
                        shots.spawn(self.x, self.y, ang1.cos() * speed, ang1.sin() * speed, speed, is_sup, is_bnc);
                        shots.spawn(self.x, self.y, ang2.cos() * speed, ang2.sin() * speed, speed, is_sup, is_bnc);
                    }

                    // Back shot
                    if self.back_shot > 0.0 {
                        let ang = self.rot + std::f32::consts::PI;
                        shots.spawn(self.x, self.y, ang.cos() * speed, ang.sin() * speed, speed, is_sup, is_bnc);
                    }

                    // Grid recoil push
                    grid.push(self.x, self.y, 35.0, 1.5);
                }
            }
        }

        // Bomb input (B key or Shift or Right Mouse Button)
        let mut bomb_activated = false;
        let bomb_key = is_key_down(KeyCode::B) || is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) || is_mouse_button_down(MouseButton::Right);
        if bomb_key && self.bombs > 0 && self.bomb_cooldown <= 0.0 {
            self.bombs -= 1;
            self.bomb_cooldown = 60.0;
            self.shield_timer = 90.0;
            bomb_activated = true;
            grid.bomb_shockwave(self.x, self.y);
            particles.bomb_ring(self.x, self.y, 180);
        }

        (bomb_activated, shot_fired)
    }

    pub fn draw(&self, player_tex: Option<&Texture2D>) {
        if self.dead {
            return;
        }

        // Draw shield aura if active
        if self.shield_timer > 0.0 {
            let pulse = (self.shield_timer * 0.2).sin() * 3.0;
            draw_circle_lines(self.x, self.y, 22.0 + pulse, 2.0, Color::new(0.2, 0.8, 1.0, 0.8));
        }

        if let Some(tex) = player_tex {
            draw_texture_ex(
                tex,
                self.x - tex.width() * 0.5,
                self.y - tex.height() * 0.5,
                COL_PLAYER,
                DrawTextureParams {
                    rotation: self.rot + std::f32::consts::FRAC_PI_2,
                    ..Default::default()
                },
            );
        } else {
            // Authentic geometric player vector ship
            let cos_r = self.rot.cos();
            let sin_r = self.rot.sin();
            let nose = Vec2::new(self.x + cos_r * 16.0, self.y + sin_r * 16.0);
            let wing_l = Vec2::new(
                self.x - cos_r * 12.0 - sin_r * 10.0,
                self.y - sin_r * 12.0 + cos_r * 10.0,
            );
            let wing_r = Vec2::new(
                self.x - cos_r * 12.0 + sin_r * 10.0,
                self.y - sin_r * 12.0 - cos_r * 10.0,
            );
            let center_in = Vec2::new(self.x - cos_r * 6.0, self.y - sin_r * 6.0);

            draw_line(nose.x, nose.y, wing_l.x, wing_l.y, 2.0, COL_PLAYER);
            draw_line(wing_l.x, wing_l.y, center_in.x, center_in.y, 2.0, COL_PLAYER);
            draw_line(center_in.x, center_in.y, wing_r.x, wing_r.y, 2.0, COL_PLAYER);
            draw_line(wing_r.x, wing_r.y, nose.x, nose.y, 2.0, COL_PLAYER);
        }
    }
}
