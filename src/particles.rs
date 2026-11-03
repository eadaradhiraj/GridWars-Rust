use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub life: f32,
    pub max_life: f32,
    pub color: Color,
    pub size: f32,
}

pub struct Star {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub size: f32,
    pub brightness: f32,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub stars: Vec<Star>,
}

impl ParticleSystem {
    pub fn new(num_stars: usize, width: f32, height: f32) -> Self {
        let mut stars = Vec::with_capacity(num_stars);
        for _ in 0..num_stars {
            stars.push(Star {
                x: rand::gen_range(0.0, width),
                y: rand::gen_range(0.0, height),
                speed: rand::gen_range(0.1, 0.6),
                size: rand::gen_range(1.0, 2.5),
                brightness: rand::gen_range(0.2, 0.9),
            });
        }
        Self {
            particles: Vec::with_capacity(4096),
            stars,
        }
    }

    pub fn spawn(&mut self, x: f32, y: f32, dx: f32, dy: f32, color: Color, life: f32, size: f32) {
        if self.particles.len() < 8192 {
            self.particles.push(Particle {
                x,
                y,
                dx,
                dy,
                life,
                max_life: life,
                color,
                size,
            });
        }
    }

    pub fn burst(&mut self, x: f32, y: f32, count: usize, color: Color, max_speed: f32, life: f32) {
        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(0.5, max_speed);
            let dx = angle.cos() * speed;
            let dy = angle.sin() * speed;
            self.spawn(x, y, dx, dy, color, rand::gen_range(life * 0.5, life), rand::gen_range(1.0, 3.0));
        }
    }

    pub fn bomb_ring(&mut self, x: f32, y: f32, count: usize) {
        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let speed = rand::gen_range(6.0, 12.0);
            let dx = angle.cos() * speed;
            let dy = angle.sin() * speed;
            let col = Color::new(
                rand::gen_range(0.8, 1.0),
                rand::gen_range(0.8, 1.0),
                rand::gen_range(0.8, 1.0),
                1.0,
            );
            self.spawn(x, y, dx, dy, col, rand::gen_range(40.0, 70.0), 2.5);
        }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        // Update particles
        self.particles.retain_mut(|p| {
            p.x += p.dx;
            p.y += p.dy;
            p.dx *= 0.96;
            p.dy *= 0.96;
            p.life -= 1.0;
            p.life > 0.0
        });

        // Update starfield twinkling
        for s in &mut self.stars {
            s.x -= s.speed * 0.5;
            if s.x < 0.0 {
                s.x += width;
                s.y = rand::gen_range(0.0, height);
            }
        }
    }

    pub fn draw_stars(&self) {
        for s in &self.stars {
            let c = Color::new(1.0, 1.0, 1.0, s.brightness * 0.7);
            draw_circle(s.x, s.y, s.size, c);
        }
    }

    pub fn draw_particles(&self) {
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let col = Color::new(p.color.r, p.color.g, p.color.b, p.color.a * alpha);
            draw_line(
                p.x,
                p.y,
                p.x - p.dx * 1.5,
                p.y - p.dy * 1.5,
                p.size,
                col,
            );
        }
    }
}
