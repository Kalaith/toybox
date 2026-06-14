//! 3D shop scene orchestration and immediate-mode HUD for Toybox After Hours.

use crate::data::{GameData, UpgradeDef};
use crate::state::{
    format_elapsed_time, toy_matches_display, GamePhase, GameSession, InteractionPreview,
};
use crate::toys::{toy_color, toy_profile};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, measure_ui_text};

mod environment;
mod fixtures;
mod scene3d;
mod signs;
mod space;
mod title;
mod widgets;
mod wood;

use scene3d::draw_shop_scene;
pub use space::{begin_ui_frame, end_ui_frame, set_ui_camera};
pub(crate) use title::{draw_settings_screen, draw_title_screen};
use widgets::draw_fitted_text;

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    NewGame,
    Continue,
    Settings,
    BackToTitle,
    OpenToolShop,
    CloseToolShop,
    ToggleFullscreen,
    QuitGame,
    Save,
    Load,
    Interact,
    CycleCarry,
    DropActive,
    BuyTool(String),
}

pub struct UiContext<'a> {
    pub data: &'a GameData,
    pub session: &'a GameSession,
    pub mouse_locked: bool,
}

pub fn draw_game_ui(ctx: UiContext<'_>) -> Vec<UiAction> {
    draw_shop_scene(&ctx);
    set_ui_camera();

    let actions = Vec::new();

    draw_minimal_hud(&ctx);
    draw_tool_panel(&ctx);
    draw_context_prompt(&ctx);
    draw_crosshair(&ctx);

    if ctx.session.phase == GamePhase::Finished {
        draw_finish_overlay(&ctx);
    }

    actions
}

pub(crate) fn draw_tool_shop_screen(ctx: UiContext<'_>) -> Vec<UiAction> {
    draw_shop_scene(&ctx);
    set_ui_camera();

    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.02, 0.025, 0.03, 0.58),
    );

    let mut actions = Vec::new();
    let mouse = logical_mouse_position();
    let panel = Rect::new(338.0, 132.0, 604.0, 450.0);
    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.060, 0.068, 0.078, 0.98))
            .with_border(1.0, Color::new(0.55, 0.64, 0.72, 0.70))
            .with_inner_border(3.0, 1.0, Color::new(0.94, 0.76, 0.42, 0.20)),
    );

    draw_ui_text_ex(
        "Shop Tools",
        panel.x + 24.0,
        panel.y + 42.0,
        TextStyle::new(26.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "Tool Credits: {}",
            ctx.session.available_tool_credits(ctx.data)
        ),
        panel.x + 24.0,
        panel.y + 72.0,
        TextStyle::new(17.0, Color::new(0.78, 0.92, 0.90, 1.0)).params(),
    );

    if tool_shop_button(
        Rect::new(panel.right() - 104.0, panel.y + 22.0, 78.0, 32.0),
        "Back",
        true,
        mouse,
    ) {
        actions.push(UiAction::CloseToolShop);
    }

    for (index, upgrade) in ctx.data.upgrades.iter().enumerate() {
        let row = Rect::new(
            panel.x + 24.0,
            panel.y + 104.0 + index as f32 * 104.0,
            panel.w - 48.0,
            86.0,
        );
        draw_tool_row(row, upgrade, &ctx, mouse, &mut actions);
    }

    actions
}

pub fn movement_from_keys() -> Vec2 {
    let mut direction = vec2(0.0, 0.0);

    if is_key_down(KeyCode::W) {
        direction.y += 1.0;
    }
    if is_key_down(KeyCode::D) {
        direction.x += 1.0;
    }
    if is_key_down(KeyCode::S) {
        direction.y -= 1.0;
    }
    if is_key_down(KeyCode::A) {
        direction.x -= 1.0;
    }

    direction
}

pub fn continuous_mouse_delta_pixels() -> Vec2 {
    let local_delta = mouse_delta_position();
    vec2(
        -local_delta.x * screen_width() * 0.5,
        -local_delta.y * screen_height() * 0.5,
    )
}

pub fn look_delta_from_input(mouse_delta: Vec2, mouse_locked: bool, dt: f32) -> Vec2 {
    let mut yaw_delta = 0.0;
    let mut pitch_delta = 0.0;

    if mouse_locked {
        yaw_delta += mouse_delta.x * 0.0032;
        pitch_delta -= mouse_delta.y * 0.0032;
    }

    let keyboard_speed = 1.75 * dt;
    if is_key_down(KeyCode::Left) {
        yaw_delta -= keyboard_speed;
    }
    if is_key_down(KeyCode::Right) {
        yaw_delta += keyboard_speed;
    }
    if is_key_down(KeyCode::Up) {
        pitch_delta += keyboard_speed;
    }
    if is_key_down(KeyCode::Down) {
        pitch_delta -= keyboard_speed;
    }

    vec2(yaw_delta, pitch_delta)
}

pub fn should_lock_mouse_from_screen_position(screen_position: Vec2) -> bool {
    let logical = vec2(
        screen_position.x * LOGICAL_WIDTH / screen_width().max(1.0),
        screen_position.y * LOGICAL_HEIGHT / screen_height().max(1.0),
    );
    let header = Rect::new(18.0, 16.0, 314.0, 48.0);
    let carried = carried_panel_rect();

    !header.contains_point(logical) && !carried.contains_point(logical)
}

fn draw_minimal_hud(ctx: &UiContext<'_>) {
    let rect = Rect::new(18.0, 16.0, 314.0, 48.0);
    let style = SurfaceStyle::new(Color::new(0.045, 0.050, 0.060, 0.86))
        .with_border(1.0, Color::new(0.38, 0.45, 0.54, 0.54));
    draw_surface(rect, &style);

    let placed = ctx.session.total_placed_toys();
    let toy_count = ctx.data.config.toy_count.max(1);
    let carry_limit = ctx.session.carry_limit(&ctx.data.config);
    draw_ui_text_ex(
        &format!(
            "{}   {}/{} put away   Carry {}/{}",
            format_elapsed_time(ctx.session.player.elapsed_seconds),
            placed,
            toy_count,
            ctx.session.player.carried_toy_ids.len(),
            carry_limit
        ),
        rect.x + 14.0,
        rect.y + 30.0,
        TextStyle::new(18.0, dark::TEXT_BRIGHT).params(),
    );

    draw_carried_panel(ctx);
}

fn draw_tool_panel(ctx: &UiContext<'_>) {
    let mut lines = Vec::new();
    let credits = ctx.session.available_tool_credits(ctx.data);

    if let Some(upgrade) = ctx.session.next_available_upgrade(ctx.data) {
        if credits >= upgrade.cost {
            lines.push(format!(
                "Tool credit {credits}   T Open tools: {}",
                upgrade.name
            ));
        } else {
            lines.push(format!(
                "{} needs {} credit(s). You have {}",
                upgrade.name, upgrade.cost, credits
            ));
        }
    }

    if ctx.session.scanner_enabled() {
        if let Some(active_toy) = ctx.session.active_toy() {
            if active_toy.is_repair_part() {
                lines.push("Scanner: Repair Bench".to_owned());
            } else if let Some(display) = ctx
                .data
                .displays
                .iter()
                .find(|display| toy_matches_display(active_toy, display))
            {
                lines.push(format!("Scanner: {} - {}", display.name, display.theme));
            }
        }
    }

    if lines.is_empty() {
        return;
    }

    let height = 30.0 + (lines.len().saturating_sub(1) as f32 * 22.0);
    let rect = Rect::new(18.0, 72.0, 374.0, height);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.035, 0.040, 0.048, 0.76))
            .with_border(1.0, Color::new(0.42, 0.62, 0.72, 0.42)),
    );

    for (index, line) in lines.iter().enumerate() {
        draw_fitted_text(
            line,
            rect.x + 12.0,
            rect.y + 21.0 + index as f32 * 22.0,
            rect.w - 24.0,
            15.0,
            if line.starts_with("Scanner:") {
                Color::new(0.56, 0.92, 0.92, 1.0)
            } else {
                dark::TEXT
            },
        );
    }
}

fn draw_tool_row(
    rect: Rect,
    upgrade: &UpgradeDef,
    ctx: &UiContext<'_>,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.035, 0.040, 0.048, 0.82))
            .with_border(1.0, Color::new(0.38, 0.45, 0.54, 0.34)),
    );

    draw_ui_text_ex(
        &upgrade.name,
        rect.x + 16.0,
        rect.y + 26.0,
        TextStyle::new(19.0, dark::TEXT_BRIGHT).params(),
    );
    draw_fitted_text(
        &upgrade.description,
        rect.x + 16.0,
        rect.y + 54.0,
        rect.w - 156.0,
        14.0,
        dark::TEXT,
    );

    let (status, status_color, can_buy) = tool_status(upgrade, ctx);
    draw_fitted_text(
        &status,
        rect.x + 16.0,
        rect.y + 76.0,
        rect.w - 156.0,
        13.0,
        status_color,
    );

    let button = Rect::new(rect.right() - 104.0, rect.y + 28.0, 82.0, 32.0);
    if tool_shop_button(button, "Buy", can_buy, mouse) {
        actions.push(UiAction::BuyTool(upgrade.id.clone()));
    }
}

fn tool_status(upgrade: &UpgradeDef, ctx: &UiContext<'_>) -> (String, Color, bool) {
    if ctx.session.has_upgrade(&upgrade.id) {
        return ("Owned".to_owned(), Color::new(0.60, 0.92, 0.66, 1.0), false);
    }

    let completed = ctx.session.completed_display_count();
    if completed < upgrade.unlock_completed_displays {
        return (
            format!(
                "Locked: {}/{} displays restored",
                completed, upgrade.unlock_completed_displays
            ),
            dark::TEXT_DIM,
            false,
        );
    }

    let credits = ctx.session.available_tool_credits(ctx.data);
    if credits < upgrade.cost {
        return (
            format!("Need {} credit(s). You have {}", upgrade.cost, credits),
            Color::new(0.95, 0.72, 0.36, 1.0),
            false,
        );
    }

    (
        format!("Available: {} credit(s)", upgrade.cost),
        Color::new(0.56, 0.92, 0.92, 1.0),
        true,
    )
}

fn tool_shop_button(rect: Rect, label: &str, enabled: bool, mouse: Vec2) -> bool {
    let hovered = enabled && rect.contains_point(mouse);
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let activated = hovered && is_mouse_button_released(MouseButton::Left);
    let face = if !enabled {
        Color::new(0.075, 0.080, 0.088, 0.84)
    } else if pressed {
        Color::new(0.075, 0.130, 0.150, 0.96)
    } else if hovered {
        Color::new(0.120, 0.190, 0.215, 0.98)
    } else {
        Color::new(0.090, 0.130, 0.150, 0.94)
    };
    draw_surface(
        rect,
        &SurfaceStyle::new(face).with_border(
            1.0,
            if enabled {
                Color::new(0.58, 0.78, 0.82, 0.72)
            } else {
                Color::new(0.26, 0.30, 0.34, 0.62)
            },
        ),
    );
    draw_text_centered_in_box(
        label,
        rect.x + 8.0,
        rect.y,
        rect.w - 16.0,
        rect.h,
        14.0,
        if enabled { dark::TEXT } else { dark::TEXT_DIM },
    );
    activated
}

fn logical_mouse_position() -> Vec2 {
    let (screen_x, screen_y) = mouse_position();
    vec2(
        screen_x * LOGICAL_WIDTH / screen_width().max(1.0),
        screen_y * LOGICAL_HEIGHT / screen_height().max(1.0),
    )
}

fn draw_carried_panel(ctx: &UiContext<'_>) {
    let rect = carried_panel_rect();
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.035, 0.040, 0.048, 0.78))
            .with_border(1.0, Color::new(0.38, 0.45, 0.54, 0.36)),
    );

    if ctx.session.player.carried_toy_ids.is_empty() {
        draw_text_centered_in_box(
            "Hands empty",
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            16.0,
            dark::TEXT_DIM,
        );
        return;
    }

    let Some(active_toy) = ctx.session.active_toy() else {
        return;
    };

    draw_identity_token(
        Rect::new(rect.x + 12.0, rect.y + 13.0, 20.0, 20.0),
        active_toy,
    );
    draw_ui_text_ex(
        "Holding   G drop",
        rect.x + 42.0,
        rect.y + 17.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );

    draw_fitted_text(
        &active_toy.name,
        rect.x + 42.0,
        rect.y + 36.0,
        272.0,
        15.0,
        dark::TEXT,
    );

    let mut x = rect.x + 328.0;
    for (index, toy_id) in ctx.session.player.carried_toy_ids.iter().enumerate() {
        let Some(toy) = ctx
            .session
            .toys
            .iter()
            .find(|candidate| &candidate.id == toy_id)
        else {
            continue;
        };
        let pip = Rect::new(x, rect.y + 15.0, 16.0, 16.0);
        draw_identity_token(pip, toy);
        if index == ctx.session.player.active_carry_index {
            draw_rectangle_lines(
                pip.x - 3.0,
                pip.y - 3.0,
                pip.w + 6.0,
                pip.h + 6.0,
                1.5,
                Color::new(0.96, 0.76, 0.38, 0.92),
            );
        }
        x += 24.0;
    }
}

fn draw_context_prompt(ctx: &UiContext<'_>) {
    let text = match ctx.session.interaction_preview(ctx.data) {
        InteractionPreview::PlaceMatch => "E Place held toy".to_owned(),
        InteractionPreview::PlaceMismatch => "Wrong display".to_owned(),
        InteractionPreview::RepairReady { toy_name } => format!("E Repair {toy_name}"),
        InteractionPreview::RepairNeedsParts { toy_name } => {
            format!("Find matching part for {toy_name}")
        }
        InteractionPreview::NeedsRepair => "Repair at the bench first".to_owned(),
        InteractionPreview::PutDown => "E Place on floor".to_owned(),
        InteractionPreview::Pickup { toy_name } => format!("E Pick up {toy_name}"),
        InteractionPreview::InventoryFull => "Carry full".to_owned(),
        InteractionPreview::ShelfFull => "Shelf full".to_owned(),
        InteractionPreview::LookAtEmptySlot => "Aim at an empty shelf spot".to_owned(),
        InteractionPreview::NothingNearby => {
            if ctx.mouse_locked {
                String::new()
            } else {
                "Click to look".to_owned()
            }
        }
        InteractionPreview::Finished => "Shop restored".to_owned(),
    };

    if text.is_empty() {
        return;
    }

    let rect = Rect::new(440.0, LOGICAL_HEIGHT - 78.0, 400.0, 40.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.035, 0.040, 0.048, 0.70))
            .with_border(1.0, Color::new(0.38, 0.45, 0.54, 0.28)),
    );
    draw_text_centered_in_box(
        &text,
        rect.x + 12.0,
        rect.y,
        rect.w - 24.0,
        rect.h,
        15.0,
        dark::TEXT,
    );
}

fn draw_identity_token(rect: Rect, toy: &crate::state::ToyState) {
    let color = toy_color(toy);
    let profile = toy_profile(toy.category, toy.slot_number);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(0.02, 0.02, 0.02, 0.8),
    );
    let text_size = (rect.h * 0.54).clamp(8.0, 11.0);
    let text_color = readable_token_text(color);
    let measured = measure_ui_text(profile.short_code, None, text_size as u16, 1.0);
    draw_ui_text_ex(
        profile.short_code,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + (rect.h + measured.height) * 0.5 - 1.0,
        TextStyle::new(text_size, text_color).params(),
    );
}

fn readable_token_text(color: Color) -> Color {
    let luminance = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;
    if luminance > 0.54 {
        Color::new(0.035, 0.040, 0.048, 1.0)
    } else {
        Color::new(0.96, 0.96, 0.92, 1.0)
    }
}

fn carried_panel_rect() -> Rect {
    Rect::new(18.0, LOGICAL_HEIGHT - 64.0, 428.0, 46.0)
}

fn draw_crosshair(ctx: &UiContext<'_>) {
    if ctx.session.phase != GamePhase::Playing {
        return;
    }

    let center = vec2(LOGICAL_WIDTH * 0.5, LOGICAL_HEIGHT * 0.5);
    let color = if ctx.mouse_locked {
        Color::new(0.96, 0.76, 0.38, 0.86)
    } else {
        Color::new(0.78, 0.82, 0.88, 0.38)
    };
    draw_line(
        center.x - 8.0,
        center.y,
        center.x - 2.0,
        center.y,
        1.4,
        color,
    );
    draw_line(
        center.x + 2.0,
        center.y,
        center.x + 8.0,
        center.y,
        1.4,
        color,
    );
    draw_line(
        center.x,
        center.y - 8.0,
        center.x,
        center.y - 2.0,
        1.4,
        color,
    );
    draw_line(
        center.x,
        center.y + 2.0,
        center.x,
        center.y + 8.0,
        1.4,
        color,
    );
}

fn draw_finish_overlay(ctx: &UiContext<'_>) {
    let rect = Rect::new(340.0, 190.0, 600.0, 190.0);
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.02, 0.025, 0.03, 0.56),
    );
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.08, 0.09, 0.105, 0.98))
            .with_border(1.0, Color::new(0.96, 0.74, 0.38, 0.85)),
    );
    draw_text_centered_in_box(
        "Store Restored",
        rect.x,
        rect.y + 24.0,
        rect.w,
        36.0,
        28.0,
        dark::TEXT_BRIGHT,
    );
    let completion =
        ctx.session.total_placed_toys() as f32 / ctx.data.config.toy_count.max(1) as f32 * 100.0;
    let completed_displays = ctx.session.completed_display_count();
    draw_text_centered_in_box(
        &format!(
            "Time {}   Wrong attempts {}   Completion {:.0}%",
            format_elapsed_time(ctx.session.player.elapsed_seconds),
            ctx.session.player.mistakes,
            completion
        ),
        rect.x,
        rect.y + 78.0,
        rect.w,
        28.0,
        18.0,
        dark::TEXT,
    );
    draw_text_centered_in_box(
        &format!(
            "Toys placed {}/{}   Full displays {}/{}",
            ctx.session.total_placed_toys(),
            ctx.data.config.toy_count,
            completed_displays,
            ctx.data.displays.len()
        ),
        rect.x,
        rect.y + 118.0,
        rect.w,
        28.0,
        16.0,
        dark::TEXT,
    );
    draw_text_centered_in_box(
        "The snow globe shop is ready for sunrise.",
        rect.x,
        rect.y + 150.0,
        rect.w,
        28.0,
        16.0,
        dark::TEXT_DIM,
    );
}
