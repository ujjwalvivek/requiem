#![allow(dead_code)]

//! ============================================================
//! util.rs
//! ============================================================
mod util {
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
}

//! ============================================================
//! action.rs
//! ============================================================
mod action {
use engine::{GameAction, InputState, Key};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiemAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

impl GameAction for RequiemAction {
    fn count() -> usize {
        4
    }

    fn index(&self) -> usize {
        *self as usize
    }

    fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::MoveLeft),
            1 => Some(Self::MoveRight),
            2 => Some(Self::MoveUp),
            3 => Some(Self::MoveDown),
            _ => None,
        }
    }

    fn move_negative_x() -> Option<Self> {
        Some(Self::MoveLeft)
    }
    fn move_positive_x() -> Option<Self> {
        Some(Self::MoveRight)
    }
    fn move_negative_y() -> Option<Self> {
        Some(Self::MoveUp)
    }
    fn move_positive_y() -> Option<Self> {
        Some(Self::MoveDown)
    }
}

pub fn setup_bindings(input: &mut InputState<RequiemAction>) {
    let map = input.input_map_mut();

    map.bind_key(Key::A, RequiemAction::MoveLeft);
    map.bind_key(Key::Left, RequiemAction::MoveLeft);
    map.bind_key(Key::D, RequiemAction::MoveRight);
    map.bind_key(Key::Right, RequiemAction::MoveRight);
    map.bind_key(Key::W, RequiemAction::MoveUp);
    map.bind_key(Key::Up, RequiemAction::MoveUp);
    map.bind_key(Key::S, RequiemAction::MoveDown);
    map.bind_key(Key::Down, RequiemAction::MoveDown);

    #[cfg(not(target_arch = "wasm32"))]
    {
        use engine::gilrs::Button;
        map.bind_button(Button::DPadUp, RequiemAction::MoveUp);
        map.bind_button(Button::DPadDown, RequiemAction::MoveDown);
        map.bind_button(Button::DPadLeft, RequiemAction::MoveLeft);
        map.bind_button(Button::DPadRight, RequiemAction::MoveRight);
    }
}
}

//! ============================================================
//! audio.rs
//! ============================================================
mod audio {
//? Procedural WAV generation

use engine::{AudioManager, AudioTrack, StaticSoundData, load_sound_data};

const SAMPLE_RATE: u32 = 44100;

//? Encode raw i16 samples as a WAV byte buffer and leak it to 'static.
fn encode_wav(samples: &[i16]) -> &'static [u8] {
    let data_size = (samples.len() * 2) as u32;
    let file_size = 36 + data_size;
    let mut buf = Vec::with_capacity(44 + data_size as usize);

    //* RIFF header
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");

    //* fmt chunk
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes()); //* subchunk size
    buf.extend_from_slice(&1u16.to_le_bytes()); //* PCM format
    buf.extend_from_slice(&1u16.to_le_bytes()); //* mono
    buf.extend_from_slice(&SAMPLE_RATE.to_le_bytes()); //* sample rate
    buf.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes()); //* byte rate
    buf.extend_from_slice(&2u16.to_le_bytes()); //* block align
    buf.extend_from_slice(&16u16.to_le_bytes()); //* bits per sample

    //* data chunk
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_size.to_le_bytes());
    for &s in samples {
        buf.extend_from_slice(&s.to_le_bytes());
    }

    Box::leak(buf.into_boxed_slice())
}

//? Simple LCG noise source for audio synthesis.
fn noise(seed: &mut u64) -> f32 {
    *seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    ((*seed >> 33) as f32 / u32::MAX as f32) * 2.0 - 1.0
}

fn to_sample(val: f32) -> i16 {
    (val.clamp(-1.0, 1.0) * 32000.0) as i16
}

fn samples_for(duration_secs: f32) -> usize {
    (SAMPLE_RATE as f32 * duration_secs) as usize
}

//? Sound generators

//? Shoot: sharp, percussive "tick", square wave + noise transient
fn gen_shoot() -> Vec<i16> {
    let len = samples_for(0.06);
    let mut out = Vec::with_capacity(len);
    let mut seed = 0xBAD_CAFE_u64;

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = (-t * 60.0).exp(); //* fast decay

        //* Noise transient in first 5ms
        let noise_env = if t < 0.005 { 1.0 - t / 0.005 } else { 0.0 };
        let noise_val = noise(&mut seed) * noise_env * 0.4;

        //* Square wave at 880Hz
        let phase = (880.0 * t * std::f32::consts::TAU).sin();
        let square = if phase > 0.0 { 0.6 } else { -0.6 };

        let val = (square + noise_val) * env * 0.7;
        out.push(to_sample(val));
    }
    out
}

//? Kill: satisfying crunch, descending noise burst with sine
fn gen_kill() -> Vec<i16> {
    let len = samples_for(0.10);
    let mut out = Vec::with_capacity(len);
    let mut seed = 0xDEAD_u64;

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = (-t * 25.0).exp();

        //* Descending sine: 500Hz → 120Hz
        let freq = 500.0 - t * 3800.0;
        let sine = (freq.max(80.0) * t * std::f32::consts::TAU).sin() * 0.5;

        //* Noise crunch
        let n = noise(&mut seed) * 0.5;

        let val = (sine + n) * env * 0.65;
        out.push(to_sample(val));
    }
    out
}

//? Player hit: heavy bass thud
fn gen_hit() -> Vec<i16> {
    let len = samples_for(0.18);
    let mut out = Vec::with_capacity(len);
    let mut seed = 0xBEEF_u64;

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = (-t * 12.0).exp();

        //* Low sine thud at 65Hz
        let sine = (65.0 * t * std::f32::consts::TAU).sin() * 0.7;

        //* Impact noise in first 15ms
        let noise_env = if t < 0.015 {
            1.0
        } else {
            (-((t - 0.015) * 40.0)).exp()
        };
        let n = noise(&mut seed) * noise_env * 0.5;

        let val = (sine + n) * env * 0.8;
        out.push(to_sample(val));
    }
    out
}

//? Level up: triumphant ascending arpeggio, three quick tones
fn gen_levelup() -> Vec<i16> {
    let len = samples_for(0.35);
    let mut out = Vec::with_capacity(len);

    //* C5 → E5 → G5 (major chord arpeggio)
    let notes = [523.25_f32, 659.25, 783.99];
    let note_dur = 0.09;
    let gap = 0.025;

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut val = 0.0_f32;

        for (idx, &freq) in notes.iter().enumerate() {
            let start = idx as f32 * (note_dur + gap);
            let local_t = t - start;
            if local_t >= 0.0 && local_t < note_dur {
                let env = (-(local_t / note_dur) * 4.0).exp();
                //* Sine + slight square for brightness
                let sine = (freq * local_t * std::f32::consts::TAU).sin();
                let harmonic = (freq * 2.0 * local_t * std::f32::consts::TAU).sin() * 0.2;
                val += (sine + harmonic) * env * 0.5;
            }
        }

        out.push(to_sample(val * 0.7));
    }
    out
}

//? XP pickup: tiny satisfying blip
fn gen_xp() -> Vec<i16> {
    let len = samples_for(0.035);
    let mut out = Vec::with_capacity(len);

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = (-t * 80.0).exp();
        let sine = (1600.0 * t * std::f32::consts::TAU).sin();
        let val = sine * env * 0.35;
        out.push(to_sample(val));
    }
    out
}

//? Audio assets
pub struct AudioAssets {
    pub shoot: Option<StaticSoundData>,
    pub kill: Option<StaticSoundData>,
    pub hit: Option<StaticSoundData>,
    pub levelup: Option<StaticSoundData>,
    pub xp: Option<StaticSoundData>,
}

impl AudioAssets {
    pub fn generate() -> Self {
        Self {
            shoot: load_sound_data(encode_wav(&gen_shoot())),
            kill: load_sound_data(encode_wav(&gen_kill())),
            hit: load_sound_data(encode_wav(&gen_hit())),
            levelup: load_sound_data(encode_wav(&gen_levelup())),
            xp: load_sound_data(encode_wav(&gen_xp())),
        }
    }
}

//? SFX event queue (produced in fixed_update, consumed in update)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SfxEvent {
    Shoot,
    Kill,
    Hit,
    LevelUp,
    XpPickup,
}

pub fn dispatch_sfx(events: &[SfxEvent], assets: &AudioAssets, audio: &mut AudioManager) {
    for event in events {
        let data = match event {
            SfxEvent::Shoot => &assets.shoot,
            SfxEvent::Kill => &assets.kill,
            SfxEvent::Hit => &assets.hit,
            SfxEvent::LevelUp => &assets.levelup,
            SfxEvent::XpPickup => &assets.xp,
        };
        if let Some(d) = data {
            audio.play_oneshot(d, AudioTrack::Sfx);
        }
    }
}
}

//! ============================================================
//! player.rs
//! ============================================================
mod player {
use crate::util::*;
use engine::Vec2;

//? Tracks how many times each upgrade has been picked.
#[derive(Debug, Clone, Default)]
pub struct PlayerUpgrades {
    pub spreadshot: u32,
    pub piercing: u32,
    pub rapid_fire: u32,
    pub heavy_caliber: u32,
    pub volley: u32,
    pub void_armor: u32,
    pub chronoshield: u32,
    pub siphon: u32,
    pub swiftness: u32,
    pub magnetism: u32,
    pub acceleration: u32,
    pub death_pulse: u32,
    pub orbital_blade: u32,
    pub detonation: u32,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub speed: f32,
    pub health: i32,
    pub max_health: i32,
    pub invincibility: u16,
    pub attack_timer: f32,
    pub attack_cooldown: f32,
    pub damage: i32,
    pub xp: u32,
    pub xp_to_next: u32,
    pub level: u32,
    pub pickup_radius: f32,
    pub upgrades: PlayerUpgrades,
    pub siphon_kill_counter: u32,
    pub barrier_timer: f32,
    pub barrier_active: bool,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            prev_pos: pos,
            speed: PLAYER_SPEED,
            health: PLAYER_MAX_HP,
            max_health: PLAYER_MAX_HP,
            invincibility: 0,
            attack_timer: 0.0,
            attack_cooldown: PLAYER_ATTACK_COOLDOWN,
            damage: PLAYER_DAMAGE,
            xp: 0,
            xp_to_next: XP_BASE_THRESHOLD,
            level: 1,
            pickup_radius: XP_PICKUP_RADIUS,
            upgrades: PlayerUpgrades::default(),
            siphon_kill_counter: 0,
            barrier_timer: 0.0,
            barrier_active: false,
        }
    }

    //? Effective stats (base + upgrades)

    pub fn effective_speed(&self) -> f32 {
        self.speed * (1.0 + self.upgrades.swiftness as f32 * 0.15)
    }

    pub fn effective_cooldown(&self) -> f32 {
        self.attack_cooldown * (0.80_f32.powi(self.upgrades.rapid_fire as i32))
    }

    pub fn effective_damage(&self) -> i32 {
        ((self.damage as f32) * (1.0 + self.upgrades.heavy_caliber as f32 * 0.50)).ceil() as i32
    }

    pub fn effective_proj_speed(&self) -> f32 {
        PROJECTILE_SPEED * (1.0 + self.upgrades.acceleration as f32 * 0.25)
    }

    pub fn effective_pickup_radius(&self) -> f32 {
        self.pickup_radius * (1.0 + self.upgrades.magnetism as f32 * 0.40)
    }

    pub fn effective_inv_ticks(&self) -> u16 {
        let base = PLAYER_INVINCIBILITY_TICKS as f32;
        (base * (1.0 + self.upgrades.chronoshield as f32 * 0.50)) as u16
    }

    //? Extra projectiles from the Spreadshot upgrade (fan pattern).
    pub fn spread_count(&self) -> u32 {
        self.upgrades.spreadshot * 2 //* +2 per level (1 left, 1 right)
    }

    //? Extra random-target projectiles from Volley.
    pub fn volley_count(&self) -> u32 {
        self.upgrades.volley * 2
    }

    //? Extra piercing projectiles from Piercing upgrade.
    pub fn pierce_count(&self) -> u32 {
        self.upgrades.piercing
    }

    //? Actions

    pub fn take_damage(&mut self, amount: i32) -> bool {
        if self.invincibility > 0 {
            return false;
        }
        //* Barrier absorbs one hit
        if self.barrier_active {
            self.barrier_active = false;
            self.barrier_timer = BARRIER_COOLDOWN;
            return false;
        }
        self.health -= amount;
        self.invincibility = self.effective_inv_ticks();
        true
    }

    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.xp += amount;
        if self.xp >= self.xp_to_next {
            self.xp -= self.xp_to_next;
            self.level += 1;
            self.xp_to_next = XP_BASE_THRESHOLD + self.level * XP_SCALE;
            return true; //* leveled up
        }
        false
    }

    pub fn register_kill(&mut self) -> bool {
        if self.upgrades.siphon > 0 {
            self.siphon_kill_counter += 1;
            let threshold = SIPHON_KILL_THRESHOLD.saturating_sub(self.upgrades.siphon * 3);
            if self.siphon_kill_counter >= threshold {
                self.siphon_kill_counter = 0;
                if self.health < self.max_health {
                    self.health += 1;
                    return true; //* healed
                }
            }
        }
        false
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn is_invincible(&self) -> bool {
        self.invincibility > 0
    }

    pub fn update_barrier(&mut self, dt: f32) {
        if self.upgrades.void_armor > 0 || self.upgrades.chronoshield > 0 {
            //* Barrier only exists if the upgrade is taken
        }
        //* Barrier comes from a dedicated upgrade, check it
        if !self.barrier_active && self.barrier_timer > 0.0 {
            self.barrier_timer -= dt;
            if self.barrier_timer <= 0.0 {
                self.barrier_active = true;
            }
        }
    }
}
}

//! ============================================================
//! enemy.rs
//! ============================================================
mod enemy {
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
}

//! ============================================================
//! projectile.rs
//! ============================================================
mod projectile {
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
}

//! ============================================================
//! xp.rs
//! ============================================================
mod xp {
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
}

//! ============================================================
//! spawner.rs
//! ============================================================
mod spawner {
use crate::enemy::{Enemy, EnemyKind};
use crate::util::*;
use engine::Vec2;

pub struct WaveSpawner {
    pub timer: f32,
    pub wave: u32,
}

impl WaveSpawner {
    pub fn new() -> Self {
        Self {
            timer: 2.5, //* short grace period at start
            wave: 0,
        }
    }

    //? Tick the spawner. Returns a Vec of new enemies when a wave fires.
    pub fn update(
        &mut self,
        dt: f32,
        player_pos: Vec2,
        current_count: usize,
        seed: &mut u64,
    ) -> Option<Vec<Enemy>> {
        self.timer -= dt;
        if self.timer > 0.0 {
            return None;
        }

        self.wave += 1;

        //* Scale wave interval down over time
        let interval = (WAVE_INTERVAL_BASE - self.wave as f32 * 0.15).max(WAVE_INTERVAL_MIN);
        self.timer = interval;

        //* Budget: how many enemies to spawn
        let budget = (3 + self.wave).min(12) as usize;
        let room = ENEMY_CAP.saturating_sub(current_count);
        let count = budget.min(room);
        if count == 0 {
            return None;
        }

        let mut batch = Vec::with_capacity(count);
        for _ in 0..count {
            let kind = pick_enemy_kind(self.wave, seed);
            let pos = random_spawn_pos(player_pos, seed);
            batch.push(Enemy::new(kind, pos));
        }
        Some(batch)
    }
}

//? Pick enemy kind based on wave difficulty.
fn pick_enemy_kind(wave: u32, seed: &mut u64) -> EnemyKind {
    let roll = lcg(seed);
    if wave < 3 {
        EnemyKind::Drone
    } else if wave < 6 {
        if roll < 0.20 {
            EnemyKind::Brute
        } else {
            EnemyKind::Drone
        }
    } else if wave < 10 {
        if roll < 0.15 {
            EnemyKind::Brute
        } else if roll < 0.40 {
            EnemyKind::Phantom
        } else {
            EnemyKind::Drone
        }
    } else {
        if roll < 0.25 {
            EnemyKind::Brute
        } else if roll < 0.55 {
            EnemyKind::Phantom
        } else {
            EnemyKind::Drone
        }
    }
}

//? Spawn at a random position outside the player's view.
fn random_spawn_pos(player_pos: Vec2, seed: &mut u64) -> Vec2 {
    let side = (lcg(seed) * 4.0) as u32;
    let screen_half_w = RES_W as f32 / 2.0 + SPAWN_MARGIN;
    let screen_half_h = RES_H as f32 / 2.0 + SPAWN_MARGIN;

    let (x, y) = match side {
        0 => {
            //* Left
            let x = player_pos.x - screen_half_w;
            let y = lcg_range(
                seed,
                player_pos.y - screen_half_h,
                player_pos.y + screen_half_h,
            );
            (x, y)
        }
        1 => {
            //* Right
            let x = player_pos.x + screen_half_w;
            let y = lcg_range(
                seed,
                player_pos.y - screen_half_h,
                player_pos.y + screen_half_h,
            );
            (x, y)
        }
        2 => {
            //* Top
            let x = lcg_range(
                seed,
                player_pos.x - screen_half_w,
                player_pos.x + screen_half_w,
            );
            let y = player_pos.y - screen_half_h;
            (x, y)
        }
        _ => {
            //* Bottom
            let x = lcg_range(
                seed,
                player_pos.x - screen_half_w,
                player_pos.x + screen_half_w,
            );
            let y = player_pos.y + screen_half_h;
            (x, y)
        }
    };

    Vec2::new(x.clamp(10.0, ARENA_W - 10.0), y.clamp(10.0, ARENA_H - 10.0))
}
}

//! ============================================================
//! upgrades.rs
//! ============================================================
mod upgrades {
use crate::player::Player;
use crate::util::*;

//? Every upgrade the player can pick during level-up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Upgrade {
    //* Offense
    Spreadshot,
    PiercingRounds,
    RapidFire,
    HeavyCaliber,
    Volley,

    //* Defense
    VoidArmor,
    Chronoshield,
    Siphon,
    Barrier,

    //* Utility
    Swiftness,
    Magnetism,
    Acceleration,

    //* Exotic
    DeathPulse,
    OrbitalBlade,
    Detonation,
}

impl Upgrade {
    pub const ALL: [Upgrade; 15] = [
        Upgrade::Spreadshot,
        Upgrade::PiercingRounds,
        Upgrade::RapidFire,
        Upgrade::HeavyCaliber,
        Upgrade::Volley,
        Upgrade::VoidArmor,
        Upgrade::Chronoshield,
        Upgrade::Siphon,
        Upgrade::Barrier,
        Upgrade::Swiftness,
        Upgrade::Magnetism,
        Upgrade::Acceleration,
        Upgrade::DeathPulse,
        Upgrade::OrbitalBlade,
        Upgrade::Detonation,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Spreadshot => "SPREADSHOT",
            Self::PiercingRounds => "PIERCING ROUNDS",
            Self::RapidFire => "RAPID FIRE",
            Self::HeavyCaliber => "HEAVY CALIBER",
            Self::Volley => "VOLLEY",
            Self::VoidArmor => "VOID ARMOR",
            Self::Chronoshield => "CHRONOSHIELD",
            Self::Siphon => "SIPHON",
            Self::Barrier => "BARRIER",
            Self::Swiftness => "SWIFTNESS",
            Self::Magnetism => "MAGNETISM",
            Self::Acceleration => "ACCELERATION",
            Self::DeathPulse => "DEATH PULSE",
            Self::OrbitalBlade => "ORBITAL BLADE",
            Self::Detonation => "DETONATION",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Spreadshot => "+2 projectiles in a fan pattern",
            Self::PiercingRounds => "Projectiles pierce +1 enemy",
            Self::RapidFire => "Attack cooldown -20%",
            Self::HeavyCaliber => "Projectile damage +50%",
            Self::Volley => "+2 projectiles at random targets",
            Self::VoidArmor => "+1 max HP, heal to full",
            Self::Chronoshield => "Invincibility duration +50%",
            Self::Siphon => "Heal 1 HP every 20 kills",
            Self::Barrier => "Auto-shield absorbs 1 hit / 25s",
            Self::Swiftness => "Movement speed +15%",
            Self::Magnetism => "XP pickup radius +40%",
            Self::Acceleration => "Projectile speed +25%",
            Self::DeathPulse => "Aura deals 1 dmg/1.5s nearby",
            Self::OrbitalBlade => "+2 orbiting blades around you",
            Self::Detonation => "Killed enemies explode (AoE)",
        }
    }

    pub fn category(self) -> &'static str {
        match self {
            Self::Spreadshot
            | Self::PiercingRounds
            | Self::RapidFire
            | Self::HeavyCaliber
            | Self::Volley => "OFFENSE",
            Self::VoidArmor | Self::Chronoshield | Self::Siphon | Self::Barrier => "DEFENSE",
            Self::Swiftness | Self::Magnetism | Self::Acceleration => "UTILITY",
            Self::DeathPulse | Self::OrbitalBlade | Self::Detonation => "EXOTIC",
        }
    }

    pub fn category_color(self) -> engine::egui::Color32 {
        match self.category() {
            "OFFENSE" => engine::egui::Color32::from_rgb(255, 90, 70),
            "DEFENSE" => engine::egui::Color32::from_rgb(80, 180, 255),
            "UTILITY" => engine::egui::Color32::from_rgb(255, 210, 60),
            "EXOTIC" => engine::egui::Color32::from_rgb(200, 120, 255),
            _ => engine::egui::Color32::WHITE,
        }
    }

    //? Current level of this upgrade on the player.
    pub fn current_level(&self, player: &Player) -> u32 {
        match self {
            Self::Spreadshot => player.upgrades.spreadshot,
            Self::PiercingRounds => player.upgrades.piercing,
            Self::RapidFire => player.upgrades.rapid_fire,
            Self::HeavyCaliber => player.upgrades.heavy_caliber,
            Self::Volley => player.upgrades.volley,
            Self::VoidArmor => player.upgrades.void_armor,
            Self::Chronoshield => player.upgrades.chronoshield,
            Self::Siphon => player.upgrades.siphon,
            Self::Barrier => player.upgrades.orbital_blade, //* quirk: barrier is binary
            Self::Swiftness => player.upgrades.swiftness,
            Self::Magnetism => player.upgrades.magnetism,
            Self::Acceleration => player.upgrades.acceleration,
            Self::DeathPulse => player.upgrades.death_pulse,
            Self::OrbitalBlade => player.upgrades.orbital_blade,
            Self::Detonation => player.upgrades.detonation,
        }
    }

    pub fn max_level(self) -> u32 {
        match self {
            Self::Barrier => 1,
            Self::VoidArmor => 5,
            Self::OrbitalBlade => 3,
            _ => 5,
        }
    }

    //? Apply this upgrade to the player. Returns true if applied.
    pub fn apply(self, player: &mut Player) {
        match self {
            Self::Spreadshot => player.upgrades.spreadshot += 1,
            Self::PiercingRounds => player.upgrades.piercing += 1,
            Self::RapidFire => player.upgrades.rapid_fire += 1,
            Self::HeavyCaliber => player.upgrades.heavy_caliber += 1,
            Self::Volley => player.upgrades.volley += 1,
            Self::VoidArmor => {
                player.upgrades.void_armor += 1;
                player.max_health += 1;
                player.health = player.max_health;
            }
            Self::Chronoshield => player.upgrades.chronoshield += 1,
            Self::Siphon => player.upgrades.siphon += 1,
            Self::Barrier => {
                player.barrier_active = true;
                player.barrier_timer = 0.0;
            }
            Self::Swiftness => player.upgrades.swiftness += 1,
            Self::Magnetism => player.upgrades.magnetism += 1,
            Self::Acceleration => player.upgrades.acceleration += 1,
            Self::DeathPulse => player.upgrades.death_pulse += 1,
            Self::OrbitalBlade => player.upgrades.orbital_blade += 1,
            Self::Detonation => player.upgrades.detonation += 1,
        }
    }
}

//? Pick 3 unique random upgrades that haven't hit max level.
pub fn roll_choices(player: &Player, seed: &mut u64) -> Vec<Upgrade> {
    let mut pool: Vec<Upgrade> = Upgrade::ALL
        .iter()
        .copied()
        .filter(|u| u.current_level(player) < u.max_level())
        .collect();

    let mut choices = Vec::with_capacity(3);
    for _ in 0..3 {
        if pool.is_empty() {
            break;
        }
        let idx = (lcg(seed) * pool.len() as f32) as usize % pool.len();
        choices.push(pool.remove(idx));
    }
    choices
}
}

//! ============================================================
//! render.rs
//! ============================================================
mod render {
use crate::action::RequiemAction;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::projectile::Projectile;
use crate::util::*;
use crate::xp::XpGem;
use engine::{Context, Rect, Vec2};

const W1: Rect = Rect {
    x: 0.0,
    y: 0.0,
    w: 1.0,
    h: 1.0,
};

//? Lerp between previous and current position for smooth sub-tick rendering.
fn lerp_pos(prev: Vec2, current: Vec2, alpha: f32) -> Vec2 {
    Vec2::new(
        prev.x + (current.x - prev.x) * alpha,
        prev.y + (current.y - prev.y) * alpha,
    )
}

//? Draw an additive-blended glow rect.
fn glow(ctx: &mut Context<RequiemAction>, pos: Vec2, size: Vec2, color: [f32; 4]) {
    ctx.draw_sprite_from_sheet_additive(pos, size, color, W1, false, 0);
}

//? Background grid
pub fn render_grid(ctx: &mut Context<RequiemAction>) {
    let cam_x = ctx.camera_offset_x;
    let cam_y = ctx.camera_offset_y;
    let sw = ctx.screen_width;
    let sh = ctx.screen_height;

    let start_x = ((cam_x / GRID_SPACING).floor()) * GRID_SPACING;
    let start_y = ((cam_y / GRID_SPACING).floor()) * GRID_SPACING;

    //* Vertical lines
    let mut x = start_x;
    while x <= cam_x + sw + GRID_SPACING {
        if x >= 0.0 && x <= ARENA_W {
            ctx.draw_rect(
                Vec2::new(x, cam_y.max(0.0)),
                Vec2::new(1.0, sh.min(ARENA_H - cam_y.max(0.0))),
                COL_GRID,
            );
        }
        x += GRID_SPACING;
    }

    //* Horizontal lines
    let mut y = start_y;
    while y <= cam_y + sh + GRID_SPACING {
        if y >= 0.0 && y <= ARENA_H {
            ctx.draw_rect(
                Vec2::new(cam_x.max(0.0), y),
                Vec2::new(sw.min(ARENA_W - cam_x.max(0.0)), 1.0),
                COL_GRID,
            );
        }
        y += GRID_SPACING;
    }
}

//? Arena border
pub fn render_arena_border(ctx: &mut Context<RequiemAction>) {
    let t = 2.0;
    ctx.draw_rect(Vec2::new(0.0, 0.0), Vec2::new(ARENA_W, t), COL_ARENA_BORDER);
    ctx.draw_rect(
        Vec2::new(0.0, ARENA_H - t),
        Vec2::new(ARENA_W, t),
        COL_ARENA_BORDER,
    );
    ctx.draw_rect(Vec2::new(0.0, 0.0), Vec2::new(t, ARENA_H), COL_ARENA_BORDER);
    ctx.draw_rect(
        Vec2::new(ARENA_W - t, 0.0),
        Vec2::new(t, ARENA_H),
        COL_ARENA_BORDER,
    );
}

//? Player
pub fn render_player(ctx: &mut Context<RequiemAction>, player: &Player, alpha: f32) {
    let pos = lerp_pos(player.prev_pos, player.pos, alpha);
    let sz = Vec2::splat(PLAYER_SIZE);
    let tl = draw_pos(pos, sz);

    //* Outer glow
    let glow_pad = 4.0;
    let glow_sz = Vec2::new(PLAYER_SIZE + glow_pad * 2.0, PLAYER_SIZE + glow_pad * 2.0);
    let glow_tl = draw_pos(pos, glow_sz);
    glow(ctx, glow_tl, glow_sz, COL_PLAYER_GLOW);

    //* Player body, blink during invincibility
    let color = if player.is_invincible() {
        let blink = (player.invincibility / 4) % 2 == 0;
        if blink { COL_PLAYER_INV } else { COL_PLAYER }
    } else {
        COL_PLAYER
    };
    ctx.draw_rect(tl, sz, color);

    //* Barrier indicator (small outline)
    if player.barrier_active {
        let b_pad = 7.0;
        let b_sz = Vec2::new(PLAYER_SIZE + b_pad * 2.0, PLAYER_SIZE + b_pad * 2.0);
        let b_tl = draw_pos(pos, b_sz);
        let barrier_col = [0.3, 0.6, 1.0, 0.35];
        ctx.draw_rect(b_tl, Vec2::new(b_sz.x, 1.0), barrier_col);
        ctx.draw_rect(
            Vec2::new(b_tl.x, b_tl.y + b_sz.y - 1.0),
            Vec2::new(b_sz.x, 1.0),
            barrier_col,
        );
        ctx.draw_rect(b_tl, Vec2::new(1.0, b_sz.y), barrier_col);
        ctx.draw_rect(
            Vec2::new(b_tl.x + b_sz.x - 1.0, b_tl.y),
            Vec2::new(1.0, b_sz.y),
            barrier_col,
        );
    }
}

//? Death Pulse aura
pub fn render_death_pulse(ctx: &mut Context<RequiemAction>, player: &Player, alpha: f32) {
    if player.upgrades.death_pulse == 0 {
        return;
    }
    let pos = lerp_pos(player.prev_pos, player.pos, alpha);
    let radius = DEATH_PULSE_RADIUS + player.upgrades.death_pulse as f32 * 10.0;
    let sz = Vec2::splat(radius * 2.0);
    let tl = draw_pos(pos, sz);
    glow(ctx, tl, sz, COL_DEATH_PULSE);
}

//? Orbital blades
pub fn render_orbitals(ctx: &mut Context<RequiemAction>, player: &Player, angle: f32, alpha: f32) {
    let count = player.upgrades.orbital_blade * 2;
    if count == 0 {
        return;
    }
    let pos = lerp_pos(player.prev_pos, player.pos, alpha);
    let step = std::f32::consts::TAU / count as f32;
    for i in 0..count {
        let a = angle + i as f32 * step;
        let ox = pos.x + a.cos() * ORBITAL_RADIUS;
        let oy = pos.y + a.sin() * ORBITAL_RADIUS;
        let sz = Vec2::splat(ORBITAL_SIZE);
        let tl = draw_pos(Vec2::new(ox, oy), sz);
        ctx.draw_rect(tl, sz, COL_ORBITAL);
        glow(
            ctx,
            tl,
            Vec2::splat(ORBITAL_SIZE + 3.0),
            [0.6, 0.6, 1.0, 0.10],
        );
    }
}

//? Enemies
pub fn render_enemies(ctx: &mut Context<RequiemAction>, enemies: &[Enemy], alpha: f32) {
    for enemy in enemies {
        if !enemy.is_alive() {
            continue;
        }
        let pos = lerp_pos(enemy.prev_pos, enemy.pos, alpha);
        let tl = draw_pos(pos, enemy.size);
        ctx.draw_rect(tl, enemy.size, enemy.color());

        //* Health bar for brutes
        if enemy.max_health > 2 && enemy.health < enemy.max_health {
            let bar_w = enemy.size.x;
            let bar_h = 2.0;
            let frac = enemy.health as f32 / enemy.max_health as f32;
            let bar_y = tl.y - 4.0;
            ctx.draw_rect(
                Vec2::new(tl.x, bar_y),
                Vec2::new(bar_w, bar_h),
                [0.3, 0.3, 0.3, 0.6],
            );
            ctx.draw_rect(
                Vec2::new(tl.x, bar_y),
                Vec2::new(bar_w * frac, bar_h),
                [0.9, 0.2, 0.15, 0.9],
            );
        }
    }
}

//? Projectiles
pub fn render_projectiles(
    ctx: &mut Context<RequiemAction>,
    projectiles: &[Projectile],
    alpha: f32,
) {
    let sz = Vec2::splat(PROJECTILE_SIZE);
    let glow_sz = Vec2::splat(PROJECTILE_SIZE + 3.0);
    for proj in projectiles {
        let pos = lerp_pos(proj.prev_pos, proj.pos, alpha);
        let tl = draw_pos(pos, sz);
        ctx.draw_rect(tl, sz, COL_PROJECTILE);
        glow(ctx, draw_pos(pos, glow_sz), glow_sz, COL_PROJ_GLOW);
    }
}

//? XP Gems
pub fn render_xp_gems(ctx: &mut Context<RequiemAction>, gems: &[XpGem], alpha: f32) {
    let sz = Vec2::splat(XP_GEM_SIZE);
    let glow_sz = Vec2::splat(XP_GEM_SIZE + 4.0);
    for gem in gems {
        let pos = lerp_pos(gem.prev_pos, gem.pos, alpha);
        let tl = draw_pos(pos, sz);
        ctx.draw_rect(tl, sz, COL_XP_GEM);
        glow(ctx, draw_pos(pos, glow_sz), glow_sz, COL_XP_GLOW);
    }
}

//? Damage flash overlay
pub fn render_damage_flash(ctx: &mut Context<RequiemAction>, alpha: f32) {
    if alpha <= 0.0 {
        return;
    }
    let cam_x = ctx.camera_offset_x;
    let cam_y = ctx.camera_offset_y;
    let color = [
        COL_DAMAGE_FLASH[0],
        COL_DAMAGE_FLASH[1],
        COL_DAMAGE_FLASH[2],
        alpha * COL_DAMAGE_FLASH[3],
    ];
    ctx.draw_rect(
        Vec2::new(cam_x, cam_y),
        Vec2::new(ctx.screen_width, ctx.screen_height),
        color,
    );
}
}

//! ============================================================
//! hud.rs
//! ============================================================
mod hud {
use crate::game::RequiemGame;
use engine::egui;

//? Typography
const FONT_XS: f32 = 8.0;
const FONT_SM: f32 = 10.0;
const FONT_MD: f32 = 13.0;
const FONT_LG: f32 = 16.0;
const FONT_XL: f32 = 22.0;
const FONT_TITLE: f32 = 28.0;

//? Palette
const DIM: egui::Color32 = egui::Color32::from_rgba_premultiplied(160, 160, 170, 100);
const BRIGHT: egui::Color32 = egui::Color32::from_rgba_premultiplied(200, 200, 210, 180);
const WHITE: egui::Color32 = egui::Color32::WHITE;
const RED: egui::Color32 = egui::Color32::from_rgb(220, 60, 50);
const RED_BAR_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(255, 50, 40, 30);
const GREEN: egui::Color32 = egui::Color32::from_rgb(60, 190, 100);
const GREEN_BAR_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(60, 190, 100, 25);
const GOLD: egui::Color32 = egui::Color32::from_rgb(255, 200, 50);

const PANEL_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(10, 10, 16, 210);
const PANEL_BORDER: egui::Color32 = egui::Color32::from_rgba_premultiplied(255, 255, 255, 20);
const CARD_BG: egui::Color32 = egui::Color32::from_rgba_premultiplied(12, 12, 18, 240);
const CARD_BORDER: egui::Color32 = egui::Color32::from_rgba_premultiplied(255, 255, 255, 25);
const CARD_HOVER_BORDER: egui::Color32 = egui::Color32::from_rgba_premultiplied(255, 255, 255, 90);
const OVERLAY_DARK: egui::Color32 = egui::Color32::from_rgba_premultiplied(5, 5, 10, 215);
const OVERLAY_DEATH: egui::Color32 = egui::Color32::from_rgba_premultiplied(3, 3, 6, 230);

const CR: f32 = 2.0;

//? Shared frame builder
fn hud_frame() -> egui::Frame {
    egui::Frame {
        fill: PANEL_BG,
        stroke: egui::Stroke::new(1.0, PANEL_BORDER),
        inner_margin: egui::Margin::symmetric(6, 4),
        outer_margin: egui::Margin::ZERO,
        corner_radius: egui::CornerRadius::same(CR as u8),
        shadow: egui::Shadow::NONE,
    }
}

//? Helper: paint a mini bar
fn bar(
    painter: &egui::Painter,
    rect: egui::Rect,
    frac: f32,
    bg: egui::Color32,
    fill: egui::Color32,
) {
    painter.rect_filled(rect, CR, bg);
    if frac > 0.0 {
        let fill_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(rect.width() * frac.clamp(0.0, 1.0), rect.height()),
        );
        painter.rect_filled(fill_rect, CR, fill);
    }
}

//? In-game HUD
pub fn draw_hud(egui_ctx: &egui::Context, game: &RequiemGame) {
    let mono_xs = egui::FontId::monospace(FONT_XS);
    let mono_sm = egui::FontId::monospace(FONT_SM);
    let mono_md = egui::FontId::monospace(FONT_MD);

    //* Top-left: HP + Level/XP
    egui::Area::new(egui::Id::new("hud_tl"))
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(6.0, 6.0))
        .interactable(false)
        .show(egui_ctx, |ui| {
            hud_frame().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;

                    //* HP text row
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        ui.label(egui::RichText::new("HP").font(mono_xs.clone()).color(RED));
                        let txt = format!("{}/{}", game.player.health, game.player.max_health);
                        ui.label(egui::RichText::new(txt).font(mono_sm.clone()).color(WHITE));
                    });

                    //* HP bar
                    let hp_frac = game.player.health as f32 / game.player.max_health.max(1) as f32;
                    let (bar_rect, _) =
                        ui.allocate_exact_size(egui::vec2(80.0, 4.0), egui::Sense::hover());
                    bar(ui.painter(), bar_rect, hp_frac, RED_BAR_BG, RED);

                    ui.add_space(2.0);

                    //* Level + XP text
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        ui.label(
                            egui::RichText::new(format!("LV{}", game.player.level))
                                .font(mono_xs.clone())
                                .color(GREEN),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "{}/{}",
                                game.player.xp, game.player.xp_to_next
                            ))
                            .font(mono_xs.clone())
                            .color(DIM),
                        );
                    });

                    //* XP bar
                    let xp_frac = game.player.xp as f32 / game.player.xp_to_next.max(1) as f32;
                    let (bar_rect, _) =
                        ui.allocate_exact_size(egui::vec2(80.0, 3.0), egui::Sense::hover());
                    bar(ui.painter(), bar_rect, xp_frac, GREEN_BAR_BG, GREEN);
                });
            });
        });

    //* Top-right: Timer / Kills / Wave
    egui::Area::new(egui::Id::new("hud_tr"))
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-6.0, 6.0))
        .interactable(false)
        .show(egui_ctx, |ui| {
            hud_frame().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 1.0;
                    let mins = (game.elapsed / 60.0) as u32;
                    let secs = (game.elapsed % 60.0) as u32;
                    ui.label(
                        egui::RichText::new(format!("{:02}:{:02}", mins, secs))
                            .font(mono_md.clone())
                            .color(WHITE),
                    );
                    ui.label(
                        egui::RichText::new(format!("KILLS {}", game.kills))
                            .font(mono_xs.clone())
                            .color(DIM),
                    );
                    ui.label(
                        egui::RichText::new(format!("WAVE  {}", game.spawner.wave))
                            .font(mono_xs.clone())
                            .color(DIM),
                    );
                });
            });
        });

    //* Bottom-right: FPS
    egui::Area::new(egui::Id::new("hud_fps"))
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-6.0, -4.0))
        .interactable(false)
        .show(egui_ctx, |ui| {
            ui.label(
                egui::RichText::new(format!("{:.0}", game.cached_fps))
                    .font(mono_xs)
                    .color(egui::Color32::from_rgba_premultiplied(120, 120, 130, 45)),
            );
        });
}

//? Level-Up
pub fn draw_level_up(egui_ctx: &egui::Context, game: &mut RequiemGame) -> Option<usize> {
    let mut picked: Option<usize> = None;

    //* Dark overlay (non-interactive background)
    let screen = egui_ctx.content_rect();
    egui::Area::new(egui::Id::new("lu_bg"))
        .fixed_pos(screen.min)
        .interactable(false)
        .order(egui::Order::Background)
        .show(egui_ctx, |ui| {
            ui.painter().rect_filled(screen, 0.0, OVERLAY_DARK);
        });

    //* Content
    egui::Area::new(egui::Id::new("lu_content"))
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 25.0))
        .order(egui::Order::Foreground)
        .show(egui_ctx, |ui| {
            let mono_xs = egui::FontId::monospace(FONT_XS);
            let mono_sm = egui::FontId::monospace(FONT_SM);
            let mono_md = egui::FontId::monospace(FONT_MD);
            let mono_xl = egui::FontId::monospace(FONT_XL);

            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 2.0;

                ui.label(
                    egui::RichText::new("LEVEL UP")
                        .font(mono_xl.clone())
                        .color(WHITE),
                );
                ui.label(
                    egui::RichText::new(format!(" Level {},", game.player.level))
                        .font(mono_sm.clone())
                        .color(DIM),
                );

                ui.add_space(14.0);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;

                    for (i, upgrade) in game.upgrade_choices.iter().enumerate() {
                        let current_lv = upgrade.current_level(&game.player);
                        let cat_col = upgrade.category_color();

                        let frame = egui::Frame {
                            fill: CARD_BG,
                            stroke: egui::Stroke::new(1.0, CARD_BORDER),
                            inner_margin: egui::Margin::symmetric(8, 6),
                            outer_margin: egui::Margin::ZERO,
                            corner_radius: egui::CornerRadius::same(CR as u8),
                            shadow: egui::Shadow::NONE,
                        };

                        let resp = frame.show(ui, |ui| {
                            ui.set_min_size(egui::vec2(115.0, 110.0));
                            ui.set_max_width(120.0);
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 2.0;

                                //* Color accent line
                                let (accent, _) = ui.allocate_exact_size(
                                    egui::vec2(100.0, 2.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect_filled(accent, 1.0, cat_col);

                                ui.add_space(2.0);
                                ui.label(
                                    egui::RichText::new(upgrade.category())
                                        .font(mono_xs.clone())
                                        .color(cat_col),
                                );
                                ui.label(
                                    egui::RichText::new(upgrade.name())
                                        .font(mono_md.clone())
                                        .color(WHITE),
                                );
                                ui.add_space(2.0);
                                ui.label(
                                    egui::RichText::new(upgrade.description())
                                        .font(mono_xs.clone())
                                        .color(BRIGHT),
                                );
                                ui.add_space(3.0);

                                let lv_text = if current_lv > 0 {
                                    format!("LV {} → {}", current_lv, current_lv + 1)
                                } else {
                                    "NEW".into()
                                };
                                ui.label(
                                    egui::RichText::new(lv_text)
                                        .font(mono_xs.clone())
                                        .color(cat_col),
                                );
                            });
                        });

                        //* Make the card clickable
                        let card_rect = resp.response.rect;
                        let click = ui.interact(
                            card_rect,
                            egui::Id::new(format!("lu_card_{i}")),
                            egui::Sense::click(),
                        );

                        if click.clicked() {
                            picked = Some(i);
                        }

                        if click.hovered() {
                            ui.painter().rect_stroke(
                                card_rect,
                                CR,
                                egui::Stroke::new(1.5, CARD_HOVER_BORDER),
                                egui::StrokeKind::Outside,
                            );
                        }
                    }
                });

                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("click to select")
                        .font(mono_xs)
                        .color(DIM),
                );
            });
        });

    picked
}

//? Game Over
pub fn draw_game_over(egui_ctx: &egui::Context, game: &RequiemGame) {
    //* Dark overlay
    let screen = egui_ctx.content_rect();
    egui::Area::new(egui::Id::new("go_bg"))
        .fixed_pos(screen.min)
        .interactable(false)
        .order(egui::Order::Background)
        .show(egui_ctx, |ui| {
            ui.painter().rect_filled(screen, 0.0, OVERLAY_DEATH);
        });

    egui::Area::new(egui::Id::new("go_content"))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .order(egui::Order::Foreground)
        .interactable(false)
        .show(egui_ctx, |ui| {
            let mono_xs = egui::FontId::monospace(FONT_XS);
            let mono_sm = egui::FontId::monospace(FONT_SM);
            let mono_md = egui::FontId::monospace(FONT_MD);
            let mono_lg = egui::FontId::monospace(FONT_LG);
            let mono_title = egui::FontId::monospace(FONT_TITLE);

            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 2.0;

                //* Title
                ui.label(egui::RichText::new("REQUIEM").font(mono_title).color(RED));
                ui.add_space(1.0);
                ui.label(
                    egui::RichText::new("you have fallen")
                        .font(mono_sm.clone())
                        .color(DIM),
                );

                ui.add_space(14.0);

                //* Stats box
                let stats_frame = egui::Frame {
                    fill: PANEL_BG,
                    stroke: egui::Stroke::new(1.0, PANEL_BORDER),
                    inner_margin: egui::Margin::symmetric(14, 8),
                    outer_margin: egui::Margin::ZERO,
                    corner_radius: egui::CornerRadius::same(CR as u8),
                    shadow: egui::Shadow::NONE,
                };

                stats_frame.show(ui, |ui| {
                    ui.set_min_width(120.0);
                    ui.set_max_width(150.0);
                    ui.spacing_mut().item_spacing.y = 3.0;

                    let mins = (game.elapsed / 60.0) as u32;
                    let secs = (game.elapsed % 60.0) as u32;

                    let row = |ui: &mut egui::Ui, label: &str, val: String| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            ui.label(egui::RichText::new(label).font(mono_xs.clone()).color(DIM));
                            ui.label(egui::RichText::new(val).font(mono_md.clone()).color(WHITE));
                        });
                    };

                    row(ui, "TIME ", format!("{:02}:{:02}", mins, secs));
                    row(ui, "KILLS", format!("{}", game.kills));
                    row(ui, "LEVEL", format!("{}", game.player.level));
                    row(ui, "WAVE ", format!("{}", game.spawner.wave));
                });

                ui.add_space(16.0);

                ui.label(egui::RichText::new("[ SPACE ]").font(mono_lg).color(GOLD));
                ui.add_space(1.0);
                ui.label(
                    egui::RichText::new("to rise again")
                        .font(mono_xs)
                        .color(DIM),
                );
            });
        });
}
}

//! ============================================================
//! game.rs
//! ============================================================
mod game {
use crate::action::RequiemAction;
use crate::audio::{AudioAssets, SfxEvent, dispatch_sfx};
use crate::enemy::Enemy;
use crate::hud;
use crate::player::Player;
use crate::projectile::Projectile;
use crate::render;
use crate::spawner::WaveSpawner;
use crate::upgrades::{self, Upgrade};
use crate::util::*;
use crate::xp::XpGem;
use engine::{AABB, Context, FixedTime, GameApp, Key, Vec2, move_towards};

//? Game Phase
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamePhase {
    Playing,
    LevelUp,
    GameOver,
}

//? RequiemGame
pub struct RequiemGame {
    pub phase: GamePhase,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub xp_gems: Vec<XpGem>,
    pub spawner: WaveSpawner,
    pub kills: u32,
    pub elapsed: f32,
    pub damage_flash: f32,
    pub last_move_dir: Vec2,
    pub camera_x: f32,
    pub camera_y: f32,
    pub prev_camera_x: f32,
    pub prev_camera_y: f32,
    pub seed: u64,
    pub cached_fps: f32,
    pub upgrade_choices: Vec<Upgrade>,
    pub death_pulse_timer: f32,
    pub orbital_angle: f32,
    pub sfx_queue: Vec<SfxEvent>,
    pub audio: AudioAssets,
}

impl RequiemGame {
    fn restart(&mut self) {
        let start = Vec2::new(ARENA_W / 2.0, ARENA_H / 2.0);
        self.phase = GamePhase::Playing;
        self.player = Player::new(start);
        self.enemies.clear();
        self.projectiles.clear();
        self.xp_gems.clear();
        self.spawner = WaveSpawner::new();
        self.kills = 0;
        self.elapsed = 0.0;
        self.damage_flash = 0.0;
        self.last_move_dir = Vec2::new(1.0, 0.0);
        self.camera_x = start.x - RES_W as f32 / 2.0;
        self.camera_y = start.y - RES_H as f32 / 2.0;
        self.prev_camera_x = self.camera_x;
        self.prev_camera_y = self.camera_y;
        self.upgrade_choices.clear();
        self.death_pulse_timer = 0.0;
        self.orbital_angle = 0.0;
        self.sfx_queue.clear();
    }

    //? Auto-fire projectiles
    fn fire_projectiles(&mut self) {
        let origin = self.player.pos;
        let base_damage = self.player.effective_damage();
        let speed = self.player.effective_proj_speed();
        let pierce = self.player.pierce_count();

        let dir = nearest_enemy_dir(&self.enemies, origin).unwrap_or(self.last_move_dir);

        //* Main shot
        self.spawn_projectile(origin, dir, speed, base_damage, pierce);

        //* Spreadshot: fan pattern
        let spread_count = self.player.spread_count();
        if spread_count > 0 {
            let half = spread_count / 2;
            let angle_step = 0.18; //* radians between spread lines
            for i in 1..=half {
                let a = i as f32 * angle_step;
                self.spawn_projectile(origin, rotate_vec(dir, a), speed, base_damage, pierce);
                self.spawn_projectile(origin, rotate_vec(dir, -a), speed, base_damage, pierce);
            }
        }

        //* Volley: random targets
        let volley_count = self.player.volley_count();
        for _ in 0..volley_count {
            if let Some(d) = random_enemy_dir(&self.enemies, origin, &mut self.seed) {
                self.spawn_projectile(origin, d, speed, base_damage, pierce);
            }
        }
    }

    fn spawn_projectile(&mut self, origin: Vec2, dir: Vec2, speed: f32, damage: i32, pierce: u32) {
        if self.projectiles.len() >= PROJECTILE_CAP {
            return;
        }
        let vel = dir * speed;
        self.projectiles
            .push(Projectile::new(origin, vel, damage, pierce));
    }

    //? Detonation: on-death explosion
    fn detonate_at(&mut self, pos: Vec2, ctx: &mut Context<RequiemAction>) {
        if self.player.upgrades.detonation == 0 {
            return;
        }
        let radius = DETONATION_RADIUS + self.player.upgrades.detonation as f32 * 8.0;
        let dmg = DETONATION_DAMAGE * self.player.upgrades.detonation as i32;
        for enemy in &mut self.enemies {
            if !enemy.is_alive() {
                continue;
            }
            let dist = (enemy.pos - pos).length();
            if dist < radius {
                enemy.take_damage(dmg);
            }
        }
        ctx.trigger_shake(2.0, 0.08);
    }
}

//? GameApp
impl GameApp for RequiemGame {
    type Action = RequiemAction;

    fn window_title() -> &'static str {
        "REQUIEM"
    }

    fn internal_resolution() -> (u32, u32) {
        (RES_W, RES_H)
    }

    //? Init
    fn init(ctx: &mut Context<RequiemAction>) -> Self {
        crate::action::setup_bindings(&mut ctx.input);

        let start = Vec2::new(ARENA_W / 2.0, ARENA_H / 2.0);
        let cam_x = start.x - ctx.screen_width / 2.0;
        let cam_y = start.y - ctx.screen_height / 2.0;

        Self {
            phase: GamePhase::Playing,
            player: Player::new(start),
            enemies: Vec::new(),
            projectiles: Vec::new(),
            xp_gems: Vec::new(),
            spawner: WaveSpawner::new(),
            kills: 0,
            elapsed: 0.0,
            damage_flash: 0.0,
            last_move_dir: Vec2::new(1.0, 0.0),
            camera_x: cam_x,
            camera_y: cam_y,
            prev_camera_x: cam_x,
            prev_camera_y: cam_y,
            seed: 0xDEAD_BEEF_CAFE_1337,
            cached_fps: 0.0,
            upgrade_choices: Vec::new(),
            death_pulse_timer: 0.0,
            orbital_angle: 0.0,
            sfx_queue: Vec::new(),
            audio: AudioAssets::generate(),
        }
    }

    //? Fixed Update (60Hz)
    fn fixed_update(&mut self, ctx: &mut Context<RequiemAction>, fixed_time: &FixedTime) {
        if self.phase != GamePhase::Playing {
            //* Still decay visual effects while paused
            self.damage_flash = (self.damage_flash - fixed_time.fixed_dt).max(0.0);
            return;
        }

        let dt = fixed_time.fixed_dt;
        self.elapsed += dt;

        //* Snapshot previous positions for render interpolation
        self.player.prev_pos = self.player.pos;
        for enemy in &mut self.enemies {
            enemy.prev_pos = enemy.pos;
        }
        for proj in &mut self.projectiles {
            proj.prev_pos = proj.pos;
        }
        for gem in &mut self.xp_gems {
            gem.prev_pos = gem.pos;
        }

        //* Player movement
        let mx = ctx.input.get_move_x();
        let my = ctx.input.get_move_y();
        let move_input = Vec2::new(mx, my);
        if move_input.length() > 0.01 {
            let dir = move_input.normalize();
            self.last_move_dir = dir;
            let speed = self.player.effective_speed();
            self.player.pos += dir * speed * dt;
        }

        //* Clamp to arena
        let half = PLAYER_SIZE / 2.0;
        self.player.pos.x = self.player.pos.x.clamp(half, ARENA_W - half);
        self.player.pos.y = self.player.pos.y.clamp(half, ARENA_H - half);

        //* Timers
        self.player.invincibility = self.player.invincibility.saturating_sub(1);
        self.damage_flash = (self.damage_flash - dt).max(0.0);
        self.player.update_barrier(dt);
        self.orbital_angle += ORBITAL_SPEED * dt;

        //* Auto-attack
        self.player.attack_timer -= dt;
        if self.player.attack_timer <= 0.0 {
            self.player.attack_timer += self.player.effective_cooldown();
            self.fire_projectiles();
            self.sfx_queue.push(SfxEvent::Shoot);
        }

        //* Update projectiles
        for proj in &mut self.projectiles {
            proj.update(dt);
        }
        self.projectiles.retain(|p| !p.is_expired());

        //* Spawn enemies
        if let Some(new_enemies) =
            self.spawner
                .update(dt, self.player.pos, self.enemies.len(), &mut self.seed)
        {
            self.enemies.extend(new_enemies);
        }

        //* Enemy AI
        for enemy in &mut self.enemies {
            if !enemy.is_alive() {
                continue;
            }
            enemy.pos.x = move_towards(enemy.pos.x, self.player.pos.x, enemy.speed * dt);
            enemy.pos.y = move_towards(enemy.pos.y, self.player.pos.y, enemy.speed * dt);
            enemy.hit_flash = enemy.hit_flash.saturating_sub(1);
        }

        //* Projectile → Enemy collision
        let mut kill_positions: Vec<(Vec2, u32)> = Vec::new();
        for proj in &mut self.projectiles {
            if proj.is_expired() {
                continue;
            }
            let proj_aabb = AABB::new(proj.pos, Vec2::splat(PROJECTILE_SIZE));
            for enemy in &mut self.enemies {
                if !enemy.is_alive() {
                    continue;
                }
                let enemy_aabb = AABB::new(enemy.pos, enemy.size);
                if proj_aabb.check_collision(&enemy_aabb) {
                    enemy.take_damage(proj.damage);
                    if !enemy.is_alive() {
                        kill_positions.push((enemy.pos, enemy.xp_value));
                    }
                    if proj.pierce > 0 {
                        proj.pierce -= 1;
                    } else {
                        proj.lifetime = -1.0; //* expire
                    }
                    break; //* one hit per projectile per tick
                }
            }
        }

        //* Orbital blade damage
        let orbital_count = self.player.upgrades.orbital_blade * 2;
        if orbital_count > 0 {
            let step = std::f32::consts::TAU / orbital_count as f32;
            for i in 0..orbital_count {
                let a = self.orbital_angle + i as f32 * step;
                let orb_pos = Vec2::new(
                    self.player.pos.x + a.cos() * ORBITAL_RADIUS,
                    self.player.pos.y + a.sin() * ORBITAL_RADIUS,
                );
                let orb_aabb = AABB::new(orb_pos, Vec2::splat(ORBITAL_SIZE));
                for enemy in &mut self.enemies {
                    if !enemy.is_alive() {
                        continue;
                    }
                    let enemy_aabb = AABB::new(enemy.pos, enemy.size);
                    if orb_aabb.check_collision(&enemy_aabb) {
                        enemy.take_damage(ORBITAL_DAMAGE);
                        if !enemy.is_alive() {
                            kill_positions.push((enemy.pos, enemy.xp_value));
                        }
                    }
                }
            }
        }

        //* Death pulse aura
        if self.player.upgrades.death_pulse > 0 {
            self.death_pulse_timer -= dt;
            if self.death_pulse_timer <= 0.0 {
                self.death_pulse_timer = DEATH_PULSE_INTERVAL;
                let radius = DEATH_PULSE_RADIUS + self.player.upgrades.death_pulse as f32 * 10.0;
                for enemy in &mut self.enemies {
                    if !enemy.is_alive() {
                        continue;
                    }
                    if (enemy.pos - self.player.pos).length() < radius {
                        enemy.take_damage(DEATH_PULSE_DAMAGE);
                        if !enemy.is_alive() {
                            kill_positions.push((enemy.pos, enemy.xp_value));
                        }
                    }
                }
            }
        }

        //* Process kills
        let multi_kill = kill_positions.len() >= 3;
        for (pos, xp_val) in &kill_positions {
            self.kills += 1;
            ctx.trigger_shake(SHAKE_KILL.0, SHAKE_KILL.1);
            self.player.register_kill();
            self.sfx_queue.push(SfxEvent::Kill);

            //* Spawn XP gem
            if self.xp_gems.len() < XP_CAP {
                self.xp_gems.push(XpGem::new(*pos, *xp_val));
            }
        }

        //* Detonation chain
        let det_positions: Vec<Vec2> = kill_positions.iter().map(|(p, _)| *p).collect();
        for pos in det_positions {
            self.detonate_at(pos, ctx);
        }

        //* Multi-kill bonus shake
        if multi_kill {
            ctx.trigger_shake(6.0, 0.2);
        }

        //* Remove dead enemies
        self.enemies.retain(|e| e.is_alive());

        //* Enemy → Player collision
        if !self.player.is_invincible() && !self.player.is_dead() {
            for enemy in &self.enemies {
                if !enemy.is_alive() {
                    continue;
                }
                let player_aabb = AABB::new(self.player.pos, Vec2::splat(PLAYER_SIZE));
                let enemy_aabb = AABB::new(enemy.pos, enemy.size);
                if player_aabb.check_collision(&enemy_aabb) {
                    if self.player.take_damage(enemy.damage) {
                        ctx.trigger_freeze(FREEZE_HIT);
                        ctx.trigger_shake(SHAKE_HIT.0, SHAKE_HIT.1);
                        self.damage_flash = DAMAGE_FLASH_DURATION;
                        self.sfx_queue.push(SfxEvent::Hit);
                    }
                    break;
                }
            }
        }

        //* XP gem collection
        let pickup_r = self.player.effective_pickup_radius();
        let player_pos = self.player.pos;
        let mut leveled_up = false;
        self.xp_gems.retain_mut(|gem| {
            if !gem.update(dt, player_pos, pickup_r) {
                return false; //* expired
            }
            if gem.touching_player(player_pos) {
                if self.player.add_xp(gem.value) {
                    leveled_up = true;
                }
                self.sfx_queue.push(SfxEvent::XpPickup);
                return false; //* collected
            }
            true
        });

        //* Level-up transition
        if leveled_up {
            self.phase = GamePhase::LevelUp;
            self.upgrade_choices = upgrades::roll_choices(&self.player, &mut self.seed);
            ctx.trigger_freeze(FREEZE_LEVELUP);
            ctx.trigger_shake(SHAKE_LEVELUP.0, SHAKE_LEVELUP.1);
            self.sfx_queue.push(SfxEvent::LevelUp);
        }

        //* Death check
        if self.player.is_dead() {
            self.phase = GamePhase::GameOver;
            ctx.trigger_freeze(8);
            ctx.trigger_shake(8.0, 0.35);
        }

        //* Camera follow
        self.prev_camera_x = self.camera_x;
        self.prev_camera_y = self.camera_y;
        let target_x = self.player.pos.x - ctx.screen_width / 2.0;
        let target_y = self.player.pos.y - ctx.screen_height / 2.0;
        self.camera_x += (target_x - self.camera_x) * 0.12;
        self.camera_y += (target_y - self.camera_y) * 0.12;
        self.camera_x = self
            .camera_x
            .clamp(0.0, (ARENA_W - ctx.screen_width).max(0.0));
        self.camera_y = self
            .camera_y
            .clamp(0.0, (ARENA_H - ctx.screen_height).max(0.0));
    }

    //? Variable Update
    fn update(&mut self, ctx: &mut Context<RequiemAction>) {
        self.cached_fps = ctx.fps;

        //* Dispatch queued SFX
        let events: Vec<SfxEvent> = self.sfx_queue.drain(..).collect();
        dispatch_sfx(&events, &self.audio, &mut ctx.audio);

        if self.phase != GamePhase::Playing {
            ctx.camera_offset_x = self.camera_x;
            ctx.camera_offset_y = self.camera_y;
            //* Collapse gap so resuming is glitch-free
            self.prev_camera_x = self.camera_x;
            self.prev_camera_y = self.camera_y;
        } else {
            //* Smooth interpolated camera during gameplay
            let alpha = ctx.interpolation_alpha;
            let cam_x = self.prev_camera_x + (self.camera_x - self.prev_camera_x) * alpha;
            let cam_y = self.prev_camera_y + (self.camera_y - self.prev_camera_y) * alpha;
            ctx.camera_offset_x = cam_x;
            ctx.camera_offset_y = cam_y;
        }

        //* Game-over restart
        if self.phase == GamePhase::GameOver && ctx.input.is_key_just_pressed(Key::Space) {
            self.restart();
        }
    }

    //? Render
    fn render(&mut self, ctx: &mut Context<RequiemAction>) {
        let alpha = if self.phase == GamePhase::Playing {
            ctx.interpolation_alpha
        } else {
            1.0
        };
        render::render_grid(ctx);
        render::render_arena_border(ctx);
        render::render_xp_gems(ctx, &self.xp_gems, alpha);
        render::render_enemies(ctx, &self.enemies, alpha);
        render::render_projectiles(ctx, &self.projectiles, alpha);
        render::render_death_pulse(ctx, &self.player, alpha);
        render::render_orbitals(ctx, &self.player, self.orbital_angle, alpha);
        render::render_player(ctx, &self.player, alpha);
        render::render_damage_flash(ctx, self.damage_flash / DAMAGE_FLASH_DURATION);
    }

    //? UI
    fn ui(
        &mut self,
        egui_ctx: &engine::egui::Context,
        _ctx: &mut Context<RequiemAction>,
        scene_params: &mut engine::SceneParams,
    ) {
        scene_params.background_color = COL_BG;
        scene_params.fog_enabled = true;
        scene_params.fog_density = 1.2;
        scene_params.fog_opacity = 0.04;
        scene_params.fog_color = [0.06, 0.06, 0.08];
        scene_params.fog_anim_speed = 0.3;

        match self.phase {
            GamePhase::Playing => {
                hud::draw_hud(egui_ctx, self);
            }
            GamePhase::LevelUp => {
                hud::draw_hud(egui_ctx, self);
                if let Some(idx) = hud::draw_level_up(egui_ctx, self) {
                    if idx < self.upgrade_choices.len() {
                        let upgrade = self.upgrade_choices[idx];
                        upgrade.apply(&mut self.player);
                        self.phase = GamePhase::Playing;
                    }
                }
            }
            GamePhase::GameOver => {
                hud::draw_game_over(egui_ctx, self);
            }
        }
    }
}

//? Helpers
//? Direction from origin to nearest living enemy. Returns None if no enemies.
fn nearest_enemy_dir(enemies: &[Enemy], origin: Vec2) -> Option<Vec2> {
    let mut best_dist = f32::MAX;
    let mut best_dir = None;
    for e in enemies {
        if !e.is_alive() {
            continue;
        }
        let diff = e.pos - origin;
        let dist = diff.length();
        if dist < best_dist && dist > 0.01 {
            best_dist = dist;
            best_dir = Some(diff / dist);
        }
    }
    best_dir
}

//? Direction from origin to a random living enemy.
fn random_enemy_dir(enemies: &[Enemy], origin: Vec2, seed: &mut u64) -> Option<Vec2> {
    let alive: Vec<&Enemy> = enemies.iter().filter(|e| e.is_alive()).collect();
    if alive.is_empty() {
        return None;
    }
    let idx = (lcg(seed) * alive.len() as f32) as usize % alive.len();
    let diff = alive[idx].pos - origin;
    let len = diff.length();
    if len > 0.01 { Some(diff / len) } else { None }
}

//? Rotate a 2D direction vector by `angle` radians.
fn rotate_vec(v: Vec2, angle: f32) -> Vec2 {
    let (s, c) = angle.sin_cos();
    Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c)
}
}

fn main() {
    engine::run::<game::RequiemGame>();
}

#[cfg(target_arch = "wasm32")]
mod wasm_entry {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(start)]
    pub fn wasm_main() {
        engine::run_wasm::<super::game::RequiemGame>();
    }
}

