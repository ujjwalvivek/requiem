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
