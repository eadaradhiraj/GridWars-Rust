use macroquad::prelude::*;
use crate::colors::{COL_BORDER, COL_GRID, COL_GRID_ACCENT};

#[derive(Clone, Copy)]
pub struct GridPoint {
    pub x: f32,
    pub y: f32,
    pub ox: f32,
    pub oy: f32,
    pub dx: f32,
    pub dy: f32,
}

pub struct Grid {
    pub cols: usize,
    pub rows: usize,
    pub spacing_x: f32,
    pub spacing_y: f32,
    pub width: f32,
    pub height: f32,
    pub points: Vec<GridPoint>,
}

impl Grid {
    pub fn new(width: f32, height: f32, spacing: f32) -> Self {
        let cols = (width / spacing).ceil() as usize;
        let rows = (height / spacing).ceil() as usize;
        let spacing_x = width / (cols as f32);
        let spacing_y = height / (rows as f32);

        let mut points = Vec::with_capacity((cols + 1) * (rows + 1));
        for r in 0..=rows {
            for c in 0..=cols {
                let px = (c as f32) * spacing_x;
                let py = (r as f32) * spacing_y;
                points.push(GridPoint {
                    x: px,
                    y: py,
                    ox: px,
                    oy: py,
                    dx: 0.0,
                    dy: 0.0,
                });
            }
        }

        Self {
            cols,
            rows,
            spacing_x,
            spacing_y,
            width,
            height,
            points,
        }
    }

    #[inline]
    fn idx(&self, c: usize, r: usize) -> usize {
        r * (self.cols + 1) + c
    }

    pub fn pull(&mut self, x: f32, y: f32, radius: f32, force: f32) {
        let r2 = radius * radius;
        let c_min = (((x - radius) / self.spacing_x).floor() as isize).max(1) as usize;
        let c_max = (((x + radius) / self.spacing_x).ceil() as usize).min(self.cols.saturating_sub(1));
        let r_min = (((y - radius) / self.spacing_y).floor() as isize).max(1) as usize;
        let r_max = (((y + radius) / self.spacing_y).ceil() as usize).min(self.rows.saturating_sub(1));

        if c_min > c_max || r_min > r_max {
            return;
        }

        for r in r_min..=r_max {
            for c in c_min..=c_max {
                let idx = self.idx(c, r);
                let p = &mut self.points[idx];
                let diff_x = p.x - x;
                let diff_y = p.y - y;
                let d2 = diff_x * diff_x + diff_y * diff_y;
                if d2 < r2 && d2 > 0.001 {
                    let d = d2.sqrt();
                    let factor = (1.0 - d / radius).clamp(0.0, 1.0) * force;
                    p.dx -= (diff_x / d) * factor;
                    p.dy -= (diff_y / d) * factor;
                }
            }
        }
    }

    pub fn push(&mut self, x: f32, y: f32, radius: f32, force: f32) {
        let r2 = radius * radius;
        let c_min = (((x - radius) / self.spacing_x).floor() as isize).max(1) as usize;
        let c_max = (((x + radius) / self.spacing_x).ceil() as usize).min(self.cols.saturating_sub(1));
        let r_min = (((y - radius) / self.spacing_y).floor() as isize).max(1) as usize;
        let r_max = (((y + radius) / self.spacing_y).ceil() as usize).min(self.rows.saturating_sub(1));

        if c_min > c_max || r_min > r_max {
            return;
        }

        for r in r_min..=r_max {
            for c in c_min..=c_max {
                let idx = self.idx(c, r);
                let p = &mut self.points[idx];
                let diff_x = p.x - x;
                let diff_y = p.y - y;
                let d2 = diff_x * diff_x + diff_y * diff_y;
                if d2 < r2 && d2 > 0.001 {
                    let d = d2.sqrt();
                    let factor = (1.0 - d / radius).clamp(0.0, 1.0) * force;
                    p.dx += (diff_x / d) * factor;
                    p.dy += (diff_y / d) * factor;
                }
            }
        }
    }

    pub fn bomb_shockwave(&mut self, x: f32, y: f32) {
        self.push(x, y, 400.0, 24.0);
    }

    pub fn update(&mut self) {
        let cols = self.cols;
        let rows = self.rows;

        // Laplacian neighbor averaging relaxation
        for r in 1..rows {
            for c in 1..cols {
                let idx = self.idx(c, r);
                let left = self.idx(c - 1, r);
                let right = self.idx(c + 1, r);
                let up = self.idx(c, r - 1);
                let down = self.idx(c, r + 1);

                let avg_x = (self.points[left].x + self.points[right].x + self.points[up].x + self.points[down].x) * 0.25;
                let avg_y = (self.points[left].y + self.points[right].y + self.points[up].y + self.points[down].y) * 0.25;

                let p = &mut self.points[idx];
                let diff_ax = avg_x - p.x;
                let diff_ay = avg_y - p.y;
                if diff_ax.abs() > 2.0 {
                    p.dx += diff_ax.signum() * 0.6;
                }
                if diff_ay.abs() > 2.0 {
                    p.dy += diff_ay.signum() * 0.6;
                }

                // Spring back to rest position
                let diff_ox = p.ox - p.x;
                let diff_oy = p.oy - p.y;
                if diff_ox.abs() > 0.5 {
                    p.dx += diff_ox * 0.06;
                } else {
                    p.x = p.ox;
                    p.dx *= 0.5;
                }

                if diff_oy.abs() > 0.5 {
                    p.dy += diff_oy * 0.06;
                } else {
                    p.y = p.oy;
                    p.dy *= 0.5;
                }

                // Damping
                p.dx *= 0.899;
                p.dy *= 0.899;

                // Speed limit
                let speed2 = p.dx * p.dx + p.dy * p.dy;
                if speed2 > 256.0 {
                    let s = speed2.sqrt();
                    p.dx = (p.dx / s) * 16.0;
                    p.dy = (p.dy / s) * 16.0;
                }

                p.x += p.dx;
                p.y += p.dy;
            }
        }
    }

    pub fn draw(&self) {
        let cols = self.cols;
        let rows = self.rows;

        // Horizontal lines
        for r in 0..=rows {
            let is_major = r % 4 == 0;
            let col = if is_major { COL_GRID_ACCENT } else { COL_GRID };
            let width = if is_major { 1.5 } else { 1.0 };
            for c in 0..cols {
                let p1 = &self.points[self.idx(c, r)];
                let p2 = &self.points[self.idx(c + 1, r)];
                draw_line(p1.x, p1.y, p2.x, p2.y, width, col);
            }
        }

        // Vertical lines
        for c in 0..=cols {
            let is_major = c % 4 == 0;
            let col = if is_major { COL_GRID_ACCENT } else { COL_GRID };
            let width = if is_major { 1.5 } else { 1.0 };
            for r in 0..rows {
                let p1 = &self.points[self.idx(c, r)];
                let p2 = &self.points[self.idx(c, r + 1)];
                draw_line(p1.x, p1.y, p2.x, p2.y, width, col);
            }
        }

        // Outer boundary rectangle glowing border
        draw_rectangle_lines(0.0, 0.0, self.width, self.height, 3.0, COL_BORDER);
    }
}
