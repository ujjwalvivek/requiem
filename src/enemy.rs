use crate::util::*;
use engine::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyKind {
    Drone,
    Brute,
    Phantom,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub speed: f32,
    pub health: i32,
    pub max_health: i32,
    pub size: Vec2,
    pub damage: i32,
    pub xp_value: u32,
    pub kind: EnemyKind,
    pub hit_flash: u16,
}

impl Enemy {
    pub fn new(kind: EnemyKind, pos: Vec2) -> Self {
        let (sz, spd, hp, xp) = match kind {
            EnemyKind::Drone => (DRONE_SIZE, DRONE_SPEED, DRONE_HP, DRONE_XP),
            EnemyKind::Brute => (BRUTE_SIZE, BRUTE_SPEED, BRUTE_HP, BRUTE_XP),
            EnemyKind::Phantom => (PHANTOM_SIZE, PHANTOM_SPEED, PHANTOM_HP, PHANTOM_XP),
        };
        Self {
            pos,
            prev_pos: pos,
            speed: spd,
            health: hp,
            max_health: hp,
            size: Vec2::splat(sz),
            damage: 1,
            xp_value: xp,
            kind,
            hit_flash: 0,
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        self.hit_flash = 6; //* 6 ticks = ~100ms white flash
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn color(&self) -> [f32; 4] {
        if self.hit_flash > 0 {
            return COL_HIT_FLASH;
        }
        match self.kind {
            EnemyKind::Drone => COL_DRONE,
            EnemyKind::Brute => COL_BRUTE,
            EnemyKind::Phantom => COL_PHANTOM,
        }
    }
}
