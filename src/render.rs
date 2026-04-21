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
