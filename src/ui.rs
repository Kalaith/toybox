//! 3D shop scene orchestration and immediate-mode HUD for Toybox After Hours.

use crate::data::{GameData, UpgradeDef};
use crate::state::{format_elapsed_time, GamePhase, GameSession};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

mod debug_overlay;
mod environment;
mod fixtures;
mod hud;
mod hud_icons;
mod minimap;
mod scene3d;
mod signs;
mod space;
mod title;
mod widgets;
mod wood;

pub use debug_overlay::DebugOverlay;
use hud::{draw_game_hud, pointer_blocking_rects};
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
    CloseSettings,
    BackToTitle,
    OpenToolShop,
    CloseToolShop,
    ToggleFullscreen,
    FovIncrease,
    FovDecrease,
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
    pub fov_degrees: f32,
}

pub fn draw_game_ui(ctx: UiContext<'_>, overlay: &DebugOverlay) -> Vec<UiAction> {
    let stats = draw_shop_scene(&ctx);
    set_ui_camera();

    let actions = Vec::new();

    draw_game_hud(&ctx);
    minimap::draw_minimap(&ctx);
    overlay.draw(&ctx, &stats);

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
    !pointer_blocking_rects()
        .iter()
        .any(|rect| rect.contains_point(logical))
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
