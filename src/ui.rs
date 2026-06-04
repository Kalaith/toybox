//! 3D shop scene orchestration and immediate-mode HUD for Toybox After Hours.

use crate::data::GameData;
use crate::state::{format_elapsed_time, GamePhase, GameSession, InteractionPreview};
use crate::toys::{toy_color, toy_profile};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;

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
    ToggleFullscreen,
    QuitGame,
    Save,
    Load,
    Interact,
    CycleCarry,
    DropActive,
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
    draw_context_prompt(&ctx);
    draw_crosshair(&ctx);

    if ctx.session.phase == GamePhase::Finished {
        draw_finish_overlay(&ctx);
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
    draw_text_ex(
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
    draw_text_ex(
        "Holding",
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
        InteractionPreview::PlaceMatch | InteractionPreview::PlaceMismatch => {
            "E Place held toy".to_owned()
        }
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
    let measured = measure_text(profile.short_code, None, text_size as u16, 1.0);
    draw_text_ex(
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
