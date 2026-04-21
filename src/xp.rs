use crate::util::*;
use engine::Vec2;

#[derive(Debug, Clone)]
pub struct XpGem {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub value: u32,
    pub lifetime: f32,
}

impl XpGem {
    pub fn new(pos: Vec2, value: u32) -> Self {
        Self {
            pos,
            prev_pos: pos,
            value,
            lifetime: XP_GEM_LIFETIME,
        }
    }

    //? Returns true while the gem is still alive.
    pub fn update(&mut self, dt: f32, player_pos: Vec2, pickup_radius: f32) -> bool {
        self.lifetime -= dt;
        if self.lifetime <= 0.0 {
            return false;
        }

        let diff = player_pos - self.pos;
        let dist = diff.length();

        if dist < pickup_radius && dist > 0.01 {
            //* Accelerate toward player
            let dir = diff / dist;
            self.pos += dir * XP_ATTRACT_SPEED * dt;
        }
        true
    }

    //? Check if player is touching this gem (for collection).
    pub fn touching_player(&self, player_pos: Vec2) -> bool {
        let diff = player_pos - self.pos;
        diff.length() < PLAYER_SIZE * 0.5 + XP_GEM_SIZE * 0.5
    }
}
