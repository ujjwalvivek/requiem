use engine::Vec2;

//? Resolution & Arena
pub const RES_W: u32 = 640;
pub const RES_H: u32 = 360;
pub const ARENA_W: f32 = 3000.0;
pub const ARENA_H: f32 = 3000.0;

//? Player
pub const PLAYER_SIZE: f32 = 12.0;
pub const PLAYER_SPEED: f32 = 110.0;
pub const PLAYER_MAX_HP: i32 = 5;
pub const PLAYER_ATTACK_COOLDOWN: f32 = 0.75;
pub const PLAYER_DAMAGE: i32 = 1;
pub const PLAYER_INVINCIBILITY_TICKS: u16 = 90; //* 1.5s at 60Hz
pub const XP_BASE_THRESHOLD: u32 = 8;
pub const XP_SCALE: u32 = 6;
pub const XP_PICKUP_RADIUS: f32 = 45.0;

//? Projectile
pub const PROJECTILE_SPEED: f32 = 280.0;
pub const PROJECTILE_SIZE: f32 = 4.0;
pub const PROJECTILE_LIFETIME: f32 = 1.8;
pub const PROJECTILE_CAP: usize = 120;

//? Enemy
pub const ENEMY_CAP: usize = 60;

//? Enemy kind stats: (size, speed, hp, xp_value, damage)
pub const DRONE_SIZE: f32 = 8.0;
pub const DRONE_SPEED: f32 = 65.0;
pub const DRONE_HP: i32 = 1;
pub const DRONE_XP: u32 = 1;

pub const BRUTE_SIZE: f32 = 18.0;
pub const BRUTE_SPEED: f32 = 30.0;
pub const BRUTE_HP: i32 = 6;
pub const BRUTE_XP: u32 = 5;

pub const PHANTOM_SIZE: f32 = 10.0;
pub const PHANTOM_SPEED: f32 = 85.0;
pub const PHANTOM_HP: i32 = 2;
pub const PHANTOM_XP: u32 = 2;

//? XP Gem
pub const XP_GEM_SIZE: f32 = 4.0;
pub const XP_GEM_LIFETIME: f32 = 25.0;
pub const XP_ATTRACT_SPEED: f32 = 220.0;
pub const XP_CAP: usize = 250;

//? Spawner
pub const WAVE_INTERVAL_BASE: f32 = 5.0;
pub const WAVE_INTERVAL_MIN: f32 = 1.8;
pub const SPAWN_MARGIN: f32 = 80.0;

//? Rendering
pub const GRID_SPACING: f32 = 60.0;

//? Colors, stark monochrome with minimal accents
pub const COL_BG: [f32; 3] = [0.028, 0.028, 0.035];
pub const COL_GRID: [f32; 4] = [0.07, 0.07, 0.09, 0.25];
pub const COL_ARENA_BORDER: [f32; 4] = [0.15, 0.15, 0.18, 0.6];
pub const COL_PLAYER: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
pub const COL_PLAYER_GLOW: [f32; 4] = [1.0, 1.0, 1.0, 0.10];
pub const COL_PLAYER_INV: [f32; 4] = [0.95, 0.95, 0.95, 0.45];
pub const COL_DRONE: [f32; 4] = [0.45, 0.45, 0.45, 1.0];
pub const COL_BRUTE: [f32; 4] = [0.82, 0.82, 0.82, 1.0];
pub const COL_PHANTOM: [f32; 4] = [0.30, 0.30, 0.50, 0.55];
pub const COL_PROJECTILE: [f32; 4] = [1.0, 1.0, 1.0, 0.92];
pub const COL_PROJ_GLOW: [f32; 4] = [1.0, 1.0, 1.0, 0.15];
pub const COL_XP_GEM: [f32; 4] = [0.25, 0.95, 0.55, 0.85];
pub const COL_XP_GLOW: [f32; 4] = [0.2, 0.8, 0.4, 0.18];
pub const COL_DAMAGE_FLASH: [f32; 4] = [1.0, 0.12, 0.08, 0.25];
pub const COL_HIT_FLASH: [f32; 4] = [1.0, 0.25, 0.18, 0.85];
pub const COL_DEATH_PULSE: [f32; 4] = [0.7, 0.2, 0.2, 0.12];
pub const COL_ORBITAL: [f32; 4] = [0.8, 0.8, 1.0, 0.9];

//? Game Feel
pub const SHAKE_KILL: (f32, f32) = (3.0, 0.12);
pub const SHAKE_HIT: (f32, f32) = (5.0, 0.20);
pub const SHAKE_LEVELUP: (f32, f32) = (2.0, 0.15);
pub const FREEZE_HIT: u16 = 5;
pub const FREEZE_LEVELUP: u16 = 3;

pub const DAMAGE_FLASH_DURATION: f32 = 0.35;

//? Death pulse (upgrade)
pub const DEATH_PULSE_RADIUS: f32 = 50.0;
pub const DEATH_PULSE_INTERVAL: f32 = 1.5;
pub const DEATH_PULSE_DAMAGE: i32 = 1;

//? Orbital blade (upgrade)
pub const ORBITAL_RADIUS: f32 = 28.0;
pub const ORBITAL_SPEED: f32 = 3.5; //* radians/sec
pub const ORBITAL_SIZE: f32 = 5.0;
pub const ORBITAL_DAMAGE: i32 = 1;

//? Detonation (upgrade)
pub const DETONATION_RADIUS: f32 = 35.0;
pub const DETONATION_DAMAGE: i32 = 2;

//? Siphon
pub const SIPHON_KILL_THRESHOLD: u32 = 20;

//? Barrier
pub const BARRIER_COOLDOWN: f32 = 25.0;

//? LCG pseudo-random (no `rand` crate dependency)
pub fn lcg(seed: &mut u64) -> f32 {
    *seed = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    ((*seed >> 33) as f32) / (u32::MAX as f32)
}

//? Random f32 in [lo, hi).
pub fn lcg_range(seed: &mut u64, lo: f32, hi: f32) -> f32 {
    lo + lcg(seed) * (hi - lo)
}

//? Random i32 in [lo, hi].
pub fn lcg_range_i(seed: &mut u64, lo: i32, hi: i32) -> i32 {
    lo + (lcg(seed) * (hi - lo + 1) as f32) as i32
}

//? Convert entity center → draw top-left.
pub fn draw_pos(center: Vec2, size: Vec2) -> Vec2 {
    Vec2::new(center.x - size.x * 0.5, center.y - size.y * 0.5)
}
