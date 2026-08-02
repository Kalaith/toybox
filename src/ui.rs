//! 3D shop scene orchestration and immediate-mode HUD for Toybox After Hours.

use crate::data::{GameData, UpgradeDef};
use crate::state::GameSession;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

mod ambience;
mod benches;
mod debug_overlay;
mod environment;
mod fixtures;
mod hud;
mod hud_chrome;
mod hud_icons;
mod minimap;
mod scanner;
mod scene3d;
mod score;
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
use widgets::{draw_fitted_text, draw_wrapped_text, WrapStyle};

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

thread_local! {
    static ANIMATION_SECONDS: std::cell::Cell<f32> = const { std::cell::Cell::new(0.0) };
}

/// Advance the view layer's animation clock by one frame.
///
/// Ambient animation — lamp breathing, dust motes, completion sparkles, the
/// scanner beacon pulse — used to read `get_time()`, macroquad's wall clock.
/// The capture harness simulates a fixed number of frames at a fixed timestep
/// *precisely* so a screenshot is reproducible, and the wall clock defeated
/// that: the same scene captured twice came out with different bytes, so no
/// drift check could ever distinguish a stale reference image from a fresh one.
/// Driving it from the same `dt` the simulation gets makes captures repeatable
/// and costs normal play nothing, since there `dt` is the frame time anyway.
pub fn advance_animation_clock(dt: f32) {
    ANIMATION_SECONDS.with(|clock| clock.set(clock.get() + dt));
}

/// Seconds of animation elapsed. View-only: gameplay state never reads this.
pub(crate) fn animation_seconds() -> f32 {
    ANIMATION_SECONDS.with(|clock| clock.get())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    NewGame,
    /// A shift with the deadline switched off.
    NewRelaxedGame,
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

    // The score screen is the whole message once a run ends; leaving the HUD
    // and minimap under it just crowds the panel with numbers it already shows.
    if ctx.session.phase.is_over() {
        score::draw_score_screen(&ctx);
    } else {
        draw_game_hud(&ctx);
        minimap::draw_minimap(&ctx);
    }
    overlay.draw(&ctx, &stats);

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
    // Tall enough for two description lines per row: at one line, four of the
    // five tools ended mid-word and the player was buying blind.
    let panel = Rect::new(330.0, 40.0, 620.0, 648.0);
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
            92.0,
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
    let last_line = draw_wrapped_text(
        &upgrade.description,
        rect.x + 16.0,
        rect.y + 48.0,
        rect.w - 156.0,
        WrapStyle {
            size: 14.0,
            line_height: 18.0,
            max_lines: 2,
            color: dark::TEXT,
        },
    );

    let (status, status_color, can_buy) = tool_status(upgrade, ctx);
    draw_fitted_text(
        &status,
        rect.x + 16.0,
        last_line + 20.0,
        rect.w - 156.0,
        13.0,
        status_color,
    );

    let button = Rect::new(rect.right() - 104.0, rect.y + 30.0, 82.0, 32.0);
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
        // "Available: 1 credit(s)" reads as the player's balance, not the
        // price — actively wrong next to a Tool Credits counter showing 9.
        format!("Costs {} credit(s)", upgrade.cost),
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
