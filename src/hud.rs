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
