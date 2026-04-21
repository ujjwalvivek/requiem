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
