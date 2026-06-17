use crate::quad::{Quad, V_BOT_LEFT, V_BOT_RIGHT, V_TOP_LEFT, V_TOP_RIGHT};
use config::CursorTrailConfig;
use mux::renderable::StableCursorPosition;
use std::time::Instant;
use wezterm_term::StableRowIndex;

const SETTLED_THRESHOLD: f32 = 0.05;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Pos {
    x: f32,
    y: f32,
}

impl From<StableCursorPosition> for Pos {
    fn from(p: StableCursorPosition) -> Self {
        Self {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

#[derive(Debug, Default)]
struct TrailQuad([Pos; 4]);

impl std::ops::Index<usize> for TrailQuad {
    type Output = Pos;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl std::ops::IndexMut<usize> for TrailQuad {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl TrailQuad {
    fn at(p: Pos) -> Self {
        Self([
            Pos { x: p.x, y: p.y },
            Pos {
                x: p.x + 1.0,
                y: p.y,
            },
            Pos {
                x: p.x + 1.0,
                y: p.y + 1.0,
            },
            Pos {
                x: p.x,
                y: p.y + 1.0,
            },
        ])
    }

    fn interp(&mut self, target: &TrailTarget, delta_time: f32, decay_fast: f32, decay_slow: f32) {
        let target_x = [target.left, target.right, target.right, target.left];
        let target_y = [target.top, target.top, target.bottom, target.bottom];
        let center_x = (target.left + target.right) * 0.5;
        let center_y = (target.top + target.bottom) * 0.5;
        let half_diag = ((target.right - target.left).powi(2)
            + (target.bottom - target.top).powi(2))
        .sqrt()
            * 0.5;

        let mut dx = [0.0_f32; 4];
        let mut dy = [0.0_f32; 4];
        let mut dot = [0.0_f32; 4];

        for i in 0..4 {
            dx[i] = target_x[i] - self.0[i].x;
            dy[i] = target_y[i] - self.0[i].y;

            let distance = (dx[i].powi(2) + dy[i].powi(2)).sqrt();
            if distance <= f32::EPSILON {
                continue;
            }

            let corner_to_center_x = target_x[i] - center_x;
            let corner_to_center_y = target_y[i] - center_y;
            dot[i] = (dx[i] * corner_to_center_x + dy[i] * corner_to_center_y)
                / (half_diag * distance);
        }

        let min_dot = dot.iter().copied().fold(f32::INFINITY, f32::min);
        let max_dot = dot.iter().copied().fold(f32::NEG_INFINITY, f32::max);

        for i in 0..4 {
            if dx[i] == 0.0 && dy[i] == 0.0 {
                continue;
            }

            let decay = if (max_dot - min_dot).abs() < f32::EPSILON {
                decay_fast
            } else {
                decay_slow + (decay_fast - decay_slow) * (dot[i] - min_dot) / (max_dot - min_dot)
            };

            let step = 1.0 - 2.0_f32.powf(-10.0 * delta_time / decay.max(0.001));
            self.0[i].x += dx[i] * step;
            self.0[i].y += dy[i] * step;
        }
    }
}

#[derive(Debug, Default)]
struct TrailTarget {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl TrailTarget {
    fn at(p: Pos) -> Self {
        Self {
            top: p.y,
            bottom: p.y + 1.0,
            left: p.x,
            right: p.x + 1.0,
        }
    }

    fn distance_to(&self, p: Pos) -> f32 {
        (p.x - self.left).abs() + (p.y - self.top).abs()
    }
}

pub struct TickContext {
    cursor_pos: Pos,
    now: Instant,
    distance_threshold: f32,
    decay_fast: f32,
    decay_slow: f32,
    dwell_threshold: u64,
}

impl TickContext {
    pub fn from_cursor(cursor_pos: StableCursorPosition, config: &CursorTrailConfig) -> Self {
        let duration = config.duration as f32;
        Self {
            cursor_pos: cursor_pos.into(),
            now: Instant::now(),
            distance_threshold: config.distance_threshold as f32,
            decay_fast: duration / 1000.0,
            decay_slow: (duration * config.spread) / 1000.0,
            dwell_threshold: config.dwell_threshold,
        }
    }
}

#[derive(Debug)]
pub struct CursorTrail {
    quad: TrailQuad,
    target: TrailTarget,
    last_cursor_pos: Pos,
    movement: Pos,
    cursor_last_moved: Instant,
    updated_at: Instant,
    initialized: bool,
}

impl CursorTrail {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            quad: TrailQuad::default(),
            target: TrailTarget::default(),
            last_cursor_pos: Pos::default(),
            movement: Pos::default(),
            cursor_last_moved: now,
            updated_at: now,
            initialized: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn tick(&mut self, ctx: TickContext) -> bool {
        let delta_time = ctx.now.duration_since(self.updated_at).as_secs_f32();
        self.updated_at = ctx.now;

        if !self.initialized {
            self.target = TrailTarget::at(ctx.cursor_pos);
            self.quad = TrailQuad::at(ctx.cursor_pos);
            self.last_cursor_pos = ctx.cursor_pos;
            self.movement = Pos::default();
            self.cursor_last_moved = ctx.now;
            self.initialized = true;
            return false;
        }

        if self.last_cursor_pos != ctx.cursor_pos {
            self.movement = Pos {
                x: ctx.cursor_pos.x - self.last_cursor_pos.x,
                y: ctx.cursor_pos.y - self.last_cursor_pos.y,
            };
            self.cursor_last_moved = ctx.now;
            self.last_cursor_pos = ctx.cursor_pos;

            if self.target.distance_to(ctx.cursor_pos) < ctx.distance_threshold {
                self.target = TrailTarget::at(ctx.cursor_pos);
                self.quad = TrailQuad::at(ctx.cursor_pos);
                return false;
            }

            if ctx.dwell_threshold == 0 {
                self.target = TrailTarget::at(ctx.cursor_pos);
            }
        }

        let mut waiting_for_dwell = false;
        if ctx.dwell_threshold != 0 {
            let dwell_time = ctx.now.duration_since(self.cursor_last_moved).as_millis() as u64;
            if dwell_time >= ctx.dwell_threshold {
                self.target = TrailTarget::at(ctx.cursor_pos);
            } else if self.target.distance_to(ctx.cursor_pos) > 0.0 {
                waiting_for_dwell = true;
            }
        }

        self.quad
            .interp(&self.target, delta_time, ctx.decay_fast, ctx.decay_slow);

        waiting_for_dwell || !self.settled(SETTLED_THRESHOLD)
    }

    fn settled(&self, threshold: f32) -> bool {
        for i in 0..4 {
            let target_x = if i == 1 || i == 2 {
                self.target.right
            } else {
                self.target.left
            };
            let target_y = if i >= 2 {
                self.target.bottom
            } else {
                self.target.top
            };
            let dx = target_x - self.quad[i].x;
            let dy = target_y - self.quad[i].y;

            if dx.abs() > threshold || dy.abs() > threshold {
                return false;
            }
        }

        true
    }

    pub fn apply_to_quad(
        &self,
        quad: &mut Quad,
        cell_width: f32,
        cell_height: f32,
        pane_left: usize,
        pane_top: StableRowIndex,
        px_x: f32,
        px_y: f32,
    ) {
        let mut left = self.quad[0].x.min(self.quad[3].x);
        let mut right = self.quad[1].x.max(self.quad[2].x);
        let mut top = self.quad[0].y.min(self.quad[1].y);
        let mut bottom = self.quad[2].y.max(self.quad[3].y);

        // The real cursor is anchored at the target's left/top edge; keep the
        // trail connected to that anchor when moving in either direction.
        if self.movement.x.abs() >= self.movement.y.abs() {
            if self.movement.x >= 0.0 {
                right = right.min(self.target.left);
            } else {
                left = left.min(self.target.left);
            }
        } else if self.movement.y >= 0.0 {
            bottom = bottom.min(self.target.top);
        } else {
            top = top.min(self.target.top);
        }

        if right < left {
            right = left;
        }
        if bottom < top {
            bottom = top;
        }

        quad.vert[V_TOP_LEFT].position = [
            px_x + (left - pane_left as f32) * cell_width,
            px_y + (top - pane_top as f32) * cell_height,
        ];
        quad.vert[V_TOP_RIGHT].position = [
            px_x + (right - pane_left as f32) * cell_width,
            px_y + (top - pane_top as f32) * cell_height,
        ];
        quad.vert[V_BOT_RIGHT].position = [
            px_x + (right - pane_left as f32) * cell_width,
            px_y + (bottom - pane_top as f32) * cell_height,
        ];
        quad.vert[V_BOT_LEFT].position = [
            px_x + (left - pane_left as f32) * cell_width,
            px_y + (bottom - pane_top as f32) * cell_height,
        ];
    }
}
