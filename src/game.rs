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
