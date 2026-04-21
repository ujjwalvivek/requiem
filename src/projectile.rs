use crate::util::*;
use engine::Vec2;

#[derive(Debug, Clone)]
pub struct Projectile {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub vel: Vec2,
    pub damage: i32,
    pub lifetime: f32,
    pub pierce: u32, //* how many more enemies it can pass through
}

impl Projectile {
    pub fn new(pos: Vec2, vel: Vec2, damage: i32, pierce: u32) -> Self {
        Self {
            pos,
            prev_pos: pos,
            vel,
            damage,
            lifetime: PROJECTILE_LIFETIME,
            pierce,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.lifetime -= dt;
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime <= 0.0
            || self.pos.x < -60.0
            || self.pos.x > ARENA_W + 60.0
            || self.pos.y < -60.0
            || self.pos.y > ARENA_H + 60.0
    }
}
