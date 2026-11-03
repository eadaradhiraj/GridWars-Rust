use macroquad::prelude::*;
use crate::colors::{COL_SHOT, COL_SHOT_BOUNCY, COL_SHOT_SUPER};
use crate::particles::ParticleSystem;

#[derive(Clone, Copy)]
pub struct Shot {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub speed: f32,
    pub is_super: bool,
    pub is_bouncy: bool,
    pub bounces_left: i32,
    pub alive: bool,
}

pub struct ShotManager {
    pub shots: Vec<Shot>,
}

impl ShotManager {
    pub fn new() -> Self {
        Self {
            shots: Vec::with_capacity(512),
        }
    }

    pub fn spawn(&mut self, x: f32, y: f32, dx: f32, dy: f32, speed: f32, is_super: bool, is_bouncy: bool) {
        if self.shots.len() < 600 {
            self.shots.push(Shot {
                x,
                y,
                dx,
                dy,
                speed,
                is_super,
                is_bouncy,
                bounces_left: if is_bouncy { 3 } else { 0 },
                alive: true,
            });
        }
    }

    pub fn clear(&mut self) {
        self.shots.clear();
    }

    pub fn update(&mut self, width: f32, height: f32, particles: &mut ParticleSystem) {
        for s in &mut self.shots {
            if !s.alive {
                continue;
            }

            // Sub-stepping for high-speed accuracy
            for _ in 0..2 {
                s.x += s.dx * 0.5;
                s.y += s.dy * 0.5;

                // Playfield boundaries
                if s.x < 4.0 {
                    s.x = 4.0;
                    if s.is_bouncy && s.bounces_left > 0 {
                        s.dx = -s.dx;
                        s.bounces_left -= 1;
                    } else {
                        s.alive = false;
                        break;
                    }
                } else if s.x > width - 4.0 {
                    s.x = width - 4.0;
                    if s.is_bouncy && s.bounces_left > 0 {
                        s.dx = -s.dx;
                        s.bounces_left -= 1;
                    } else {
                        s.alive = false;
                        break;
                    }
                }

                if s.y < 4.0 {
                    s.y = 4.0;
                    if s.is_bouncy && s.bounces_left > 0 {
                        s.dy = -s.dy;
                        s.bounces_left -= 1;
                    } else {
                        s.alive = false;
                        break;
                    }
                } else if s.y > height - 4.0 {
                    s.y = height - 4.0;
                    if s.is_bouncy && s.bounces_left > 0 {
                        s.dy = -s.dy;
                        s.bounces_left -= 1;
                    } else {
                        s.alive = false;
                        break;
                    }
                }
            }

            // Boundary impact particles
            if !s.alive {
                let col = if s.is_super {
                    COL_SHOT_SUPER
                } else if s.is_bouncy {
                    COL_SHOT_BOUNCY
                } else {
                    COL_SHOT
                };
                particles.burst(s.x, s.y, 4, col, 2.0, 10.0);
            }
        }

        self.shots.retain(|s| s.alive);
    }

    pub fn draw(&self, shot_tex: Option<&Texture2D>) {
        for s in &self.shots {
            let col = if s.is_super {
                COL_SHOT_SUPER
            } else if s.is_bouncy {
                COL_SHOT_BOUNCY
            } else {
                COL_SHOT
            };

            if let Some(tex) = shot_tex {
                let rot = s.dy.atan2(s.dx) + std::f32::consts::FRAC_PI_2;
                draw_texture_ex(
                    tex,
                    s.x - tex.width() * 0.5,
                    s.y - tex.height() * 0.5,
                    col,
                    DrawTextureParams {
                        rotation: rot,
                        ..Default::default()
                    },
                );
            } else {
                let trail_x = s.x - s.dx * 1.2;
                let trail_y = s.y - s.dy * 1.2;
                draw_line(trail_x, trail_y, s.x, s.y, 2.5, col);
            }
        }
    }
}
