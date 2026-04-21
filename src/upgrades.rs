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
