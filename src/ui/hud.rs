//! Gameplay HUD panels inspired by the warm toy-store reference.

use super::hud_icons::{
    brighten_color, category_icon, draw_icon, draw_open_box_icon, draw_stopwatch_icon, IconKind,
};
use crate::state::{
    format_elapsed_time, toy_matches_display, GamePhase, InteractionPreview, ToyState,
};
use crate::toys::{toy_color, toy_profile};
use crate::ui::widgets::draw_fitted_text;
use crate::ui::{UiContext, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, measure_ui_text};

pub(super) fn draw_game_hud(ctx: &UiContext<'_>) {
    draw_status_panel(ctx);
    draw_notice_panel(ctx);
    draw_carried_card(ctx);
    draw_context_prompt(ctx);
    draw_crosshair(ctx);
}

pub(super) fn pointer_blocking_rects() -> [Rect; 2] {
    [status_panel_rect(), carried_card_rect()]
}

fn draw_status_panel(ctx: &UiContext<'_>) {
    let rect = status_panel_rect();
    draw_hud_panel(rect, Color::new(0.025, 0.026, 0.032, 0.86), hud_border());

    let time = format_elapsed_time(ctx.session.player.elapsed_seconds);
    draw_stopwatch_icon(vec2(rect.x + 33.0, rect.y + 38.0), 15.0, dark::TEXT_BRIGHT);
    draw_ui_text_ex(
        &time,
        rect.x + 64.0,
        rect.y + 51.0,
        TextStyle::new(31.0, dark::TEXT_BRIGHT).params(),
    );

    draw_divider(rect.x, rect.y + 69.0, rect.w);

    let placed = ctx.session.total_placed_toys();
    let toy_count = ctx.data.config.toy_count.max(1);
    draw_status_row(
        vec2(rect.x + 18.0, rect.y + 82.0),
        IconKind::Star,
        "Toys Put Away",
        &format!("{placed} / {toy_count}"),
        placed as f32 / toy_count as f32,
        Color::new(1.0, 0.72, 0.16, 1.0),
    );

    let carry_limit = ctx.session.carry_limit(&ctx.data.config).max(1);
    let carried = ctx.session.player.carried_toy_ids.len();
    draw_status_row(
        vec2(rect.x + 18.0, rect.y + 126.0),
        IconKind::Crate,
        "Carry",
        &format!("{carried} / {carry_limit}"),
        carried as f32 / carry_limit as f32,
        Color::new(0.93, 0.48, 0.18, 1.0),
    );
}

fn draw_status_row(
    origin: Vec2,
    icon: IconKind,
    label: &str,
    value: &str,
    progress: f32,
    accent: Color,
) {
    draw_icon(icon, vec2(origin.x + 13.0, origin.y + 13.0), 14.0, accent);
    draw_ui_text_ex(
        label,
        origin.x + 36.0,
        origin.y + 12.0,
        TextStyle::new(13.0, Color::new(0.88, 0.86, 0.80, 1.0)).params(),
    );
    draw_ui_text_ex(
        value,
        origin.x + 36.0,
        origin.y + 32.0,
        TextStyle::new(18.0, dark::TEXT_BRIGHT).params(),
    );
    draw_progress_bar(
        Rect::new(origin.x + 104.0, origin.y + 22.0, 54.0, 6.0),
        progress,
        accent,
    );
}

fn draw_notice_panel(ctx: &UiContext<'_>) {
    let mut rows = Vec::new();
    let credits = ctx.session.available_tool_credits(ctx.data);

    if let Some(upgrade) = ctx.session.next_available_upgrade(ctx.data) {
        let row = if credits >= upgrade.cost {
            NoticeRow {
                key: Some("T"),
                text: format!("Open tools: {}", upgrade.name),
                tone: NoticeTone::Tool,
            }
        } else {
            NoticeRow {
                key: None,
                text: format!("{} needs {} credit(s)", upgrade.name, upgrade.cost),
                tone: NoticeTone::Warning,
            }
        };
        rows.push(row);
    }

    if ctx.session.scanner_enabled() {
        if let Some(active_toy) = ctx.session.active_toy() {
            let text = if active_toy.is_repair_part() {
                "Scanner: Repair Bench".to_owned()
            } else if let Some(display) = ctx
                .data
                .displays
                .iter()
                .find(|display| toy_matches_display(active_toy, display))
            {
                format!("Scanner: {} - {}", display.name, display.theme)
            } else {
                String::new()
            };

            if !text.is_empty() {
                rows.push(NoticeRow {
                    key: None,
                    text,
                    tone: NoticeTone::Scanner,
                });
            }
        }
    }

    if rows.is_empty() {
        return;
    }

    let row_height = 34.0;
    let rect = Rect::new(
        18.0,
        status_panel_rect().bottom() + 10.0,
        342.0,
        14.0 + row_height * rows.len() as f32,
    );
    draw_hud_panel(rect, Color::new(0.020, 0.024, 0.030, 0.75), subtle_border());

    for (index, row) in rows.iter().enumerate() {
        let y = rect.y + 9.0 + index as f32 * row_height;
        let accent = row.tone.accent();
        if let Some(key) = row.key {
            draw_keycap(Rect::new(rect.x + 13.0, y + 4.0, 27.0, 23.0), key, false);
            draw_fitted_text(
                &row.text,
                rect.x + 50.0,
                y + 21.0,
                rect.w - 64.0,
                14.0,
                row.tone.text_color(),
            );
        } else {
            draw_notice_dot(vec2(rect.x + 27.0, y + 16.0), accent);
            draw_fitted_text(
                &row.text,
                rect.x + 50.0,
                y + 21.0,
                rect.w - 64.0,
                14.0,
                row.tone.text_color(),
            );
        }
    }
}

fn draw_carried_card(ctx: &UiContext<'_>) {
    let rect = carried_card_rect();
    draw_hud_panel(rect, Color::new(0.025, 0.024, 0.030, 0.82), subtle_border());

    if ctx.session.player.carried_toy_ids.is_empty() {
        draw_empty_hands_card(rect);
        return;
    }

    let Some(active_toy) = ctx.session.active_toy() else {
        return;
    };

    draw_toy_badge(
        Rect::new(rect.x + 14.0, rect.y + 14.0, 58.0, 58.0),
        active_toy,
    );
    draw_fitted_text(
        &active_toy.name,
        rect.x + 88.0,
        rect.y + 42.0,
        rect.w - 122.0,
        20.0,
        dark::TEXT_BRIGHT,
    );

    draw_keycap(
        Rect::new(rect.right() - 70.0, rect.y + 49.0, 25.0, 22.0),
        "G",
        false,
    );
    draw_ui_text_ex(
        "Drop",
        rect.right() - 38.0,
        rect.y + 65.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );

    draw_carry_pips(ctx, rect, active_toy);
}

fn draw_empty_hands_card(rect: Rect) {
    let icon_rect = Rect::new(rect.x + 18.0, rect.y + 17.0, 50.0, 50.0);
    draw_surface(
        icon_rect,
        &SurfaceStyle::new(Color::new(0.10, 0.11, 0.12, 0.88))
            .with_border(1.0, Color::new(0.46, 0.42, 0.36, 0.56)),
    );
    draw_open_box_icon(
        vec2(
            icon_rect.x + icon_rect.w * 0.5,
            icon_rect.y + icon_rect.h * 0.53,
        ),
        19.0,
        Color::new(0.70, 0.60, 0.46, 0.82),
    );
    draw_ui_text_ex(
        "Hands empty",
        rect.x + 86.0,
        rect.y + 36.0,
        TextStyle::new(20.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "Ready to sort",
        rect.x + 88.0,
        rect.y + 58.0,
        TextStyle::new(13.0, dark::TEXT_DIM).params(),
    );
}

fn draw_carry_pips(ctx: &UiContext<'_>, rect: Rect, active_toy: &ToyState) {
    let mut x = rect.right() - 110.0;
    let y = rect.y + 18.0;

    for (index, toy_id) in ctx.session.player.carried_toy_ids.iter().enumerate() {
        let Some(toy) = ctx
            .session
            .toys
            .iter()
            .find(|candidate| &candidate.id == toy_id)
        else {
            continue;
        };

        let pip = Rect::new(x, y, 18.0, 18.0);
        draw_identity_token(pip, toy);
        if toy.id == active_toy.id || index == ctx.session.player.active_carry_index {
            draw_rectangle_lines(
                pip.x - 3.0,
                pip.y - 3.0,
                pip.w + 6.0,
                pip.h + 6.0,
                1.5,
                Color::new(1.0, 0.73, 0.22, 0.95),
            );
        }
        x += 25.0;
    }
}

fn draw_context_prompt(ctx: &UiContext<'_>) {
    let Some(prompt) = prompt_for_interaction(ctx) else {
        return;
    };

    let rect = prompt_rect();
    let border = prompt.tone.border();
    draw_hud_panel(rect, Color::new(0.026, 0.024, 0.030, 0.84), border);

    let text_x = if let Some(key) = prompt.key {
        draw_keycap(
            Rect::new(rect.x + 22.0, rect.y + 10.0, 34.0, 32.0),
            key,
            true,
        );
        rect.x + 78.0
    } else {
        draw_prompt_status_icon(vec2(rect.x + 39.0, rect.y + 26.0), prompt.tone);
        rect.x + 68.0
    };

    let badge_width = if let Some(toy) = ctx.session.active_toy() {
        if prompt.tone == PromptTone::Action || prompt.tone == PromptTone::Good {
            draw_identity_token(
                Rect::new(rect.right() - 50.0, rect.y + 15.0, 24.0, 24.0),
                toy,
            );
            54.0
        } else {
            20.0
        }
    } else {
        20.0
    };

    draw_fitted_text(
        &prompt.message,
        text_x,
        rect.y + 32.0,
        rect.right() - text_x - badge_width,
        20.0,
        prompt.tone.text_color(),
    );
}

fn prompt_for_interaction(ctx: &UiContext<'_>) -> Option<PromptVisual> {
    let prompt = match ctx.session.interaction_preview(ctx.data) {
        InteractionPreview::PlaceOnShelf => PromptVisual::action("E", "Place on shelf"),
        InteractionPreview::PlaceOnRepairBench => PromptVisual::action("E", "Place on bench"),
        InteractionPreview::RepairReady { toy_name } => {
            PromptVisual::action("E", format!("Repair {toy_name}"))
        }
        InteractionPreview::RepairBenchFull => PromptVisual::warning("Bench full"),
        InteractionPreview::RepairMismatch => PromptVisual::warning("Parts do not match"),
        InteractionPreview::NeedsRepair => PromptVisual::warning("Repair at the bench first"),
        InteractionPreview::PutDown => PromptVisual::action("E", "Place on floor"),
        InteractionPreview::Pickup { toy_name } => {
            PromptVisual::action("E", format!("Pick up {toy_name}"))
        }
        InteractionPreview::InventoryFull => PromptVisual::warning("Carry full"),
        InteractionPreview::ShelfFull => PromptVisual::warning("Shelf full"),
        InteractionPreview::LookAtEmptySlot => PromptVisual::neutral("Aim at an empty shelf spot"),
        InteractionPreview::NothingNearby => {
            if ctx.mouse_locked {
                return None;
            }
            PromptVisual::neutral("Click to look")
        }
        InteractionPreview::Finished => PromptVisual::good("Shop restored"),
    };
    Some(prompt)
}

fn draw_crosshair(ctx: &UiContext<'_>) {
    if ctx.session.phase != GamePhase::Playing {
        return;
    }

    let center = vec2(LOGICAL_WIDTH * 0.5, LOGICAL_HEIGHT * 0.5);
    let color = if ctx.mouse_locked {
        Color::new(1.0, 0.75, 0.24, 0.90)
    } else {
        Color::new(0.80, 0.82, 0.86, 0.34)
    };
    draw_circle_lines(center.x, center.y, 11.0, 1.2, color);
    draw_circle(
        center.x,
        center.y,
        2.0,
        Color::new(color.r, color.g, color.b, color.a * 0.86),
    );
    draw_line(
        center.x - 17.0,
        center.y,
        center.x - 7.0,
        center.y,
        1.2,
        color,
    );
    draw_line(
        center.x + 7.0,
        center.y,
        center.x + 17.0,
        center.y,
        1.2,
        color,
    );
    draw_line(
        center.x,
        center.y - 17.0,
        center.x,
        center.y - 7.0,
        1.2,
        color,
    );
    draw_line(
        center.x,
        center.y + 7.0,
        center.x,
        center.y + 17.0,
        1.2,
        color,
    );
}

fn draw_toy_badge(rect: Rect, toy: &ToyState) {
    let color = toy_color(toy);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.060, 0.055, 0.060, 0.96))
            .with_border(1.0, Color::new(color.r, color.g, color.b, 0.78))
            .with_inner_border(3.0, 1.0, Color::new(1.0, 0.78, 0.32, 0.16)),
    );
    draw_icon(
        category_icon(toy.category),
        vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.50),
        20.0,
        brighten_color(color, 0.16),
    );

    let profile = toy_profile(toy.category, toy.slot_number);
    let text_color = readable_token_text(color);
    draw_surface(
        Rect::new(rect.right() - 22.0, rect.bottom() - 19.0, 19.0, 15.0),
        &SurfaceStyle::new(Color::new(color.r, color.g, color.b, 0.86))
            .with_border(1.0, Color::new(0.02, 0.018, 0.016, 0.82)),
    );
    draw_fitted_text(
        profile.short_code,
        rect.right() - 19.0,
        rect.bottom() - 8.0,
        14.0,
        8.0,
        text_color,
    );
}

fn draw_identity_token(rect: Rect, toy: &ToyState) {
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

fn draw_hud_panel(rect: Rect, fill: Color, border: Color) {
    draw_rectangle(
        rect.x + 3.0,
        rect.y + 4.0,
        rect.w,
        rect.h,
        Color::new(0.0, 0.0, 0.0, 0.30),
    );
    draw_surface(
        rect,
        &SurfaceStyle::new(fill)
            .with_border(1.0, border)
            .with_inner_border(3.0, 1.0, Color::new(1.0, 0.78, 0.40, 0.12)),
    );
    draw_line(
        rect.x + 6.0,
        rect.y + 4.0,
        rect.right() - 6.0,
        rect.y + 4.0,
        1.0,
        Color::new(1.0, 0.88, 0.62, 0.10),
    );
}

fn draw_keycap(rect: Rect, label: &str, large: bool) {
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.92, 0.92, 0.90, 0.98))
            .with_border(1.0, Color::new(0.22, 0.22, 0.24, 0.74))
            .with_top_highlight(2.0, Color::new(1.0, 1.0, 1.0, 0.55)),
    );
    draw_text_centered_in_box(
        label,
        rect.x,
        rect.y - if large { 1.0 } else { 0.0 },
        rect.w,
        rect.h,
        if large { 19.0 } else { 13.0 },
        Color::new(0.10, 0.10, 0.12, 1.0),
    );
}

fn draw_progress_bar(rect: Rect, progress: f32, accent: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.14, 0.12, 0.10, 0.86),
    );
    let filled = rect.w * progress.clamp(0.0, 1.0);
    draw_rectangle(
        rect.x,
        rect.y,
        filled,
        rect.h,
        Color::new(accent.r, accent.g, accent.b, 0.90),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(0.90, 0.76, 0.42, 0.28),
    );
}

fn draw_divider(x: f32, y: f32, width: f32) {
    draw_line(
        x + 1.0,
        y,
        x + width - 1.0,
        y,
        1.0,
        Color::new(0.64, 0.58, 0.50, 0.22),
    );
}

fn draw_notice_dot(center: Vec2, color: Color) {
    draw_circle(
        center.x,
        center.y,
        7.0,
        Color::new(color.r, color.g, color.b, 0.22),
    );
    draw_circle(center.x, center.y, 3.5, color);
}

fn draw_prompt_status_icon(center: Vec2, tone: PromptTone) {
    let color = tone.border();
    match tone {
        PromptTone::Warning => {
            draw_triangle(
                vec2(center.x, center.y - 11.0),
                vec2(center.x - 11.0, center.y + 10.0),
                vec2(center.x + 11.0, center.y + 10.0),
                Color::new(color.r, color.g, color.b, 0.88),
            );
            draw_line(
                center.x,
                center.y - 4.0,
                center.x,
                center.y + 3.0,
                2.0,
                Color::new(0.08, 0.06, 0.04, 1.0),
            );
            draw_circle(
                center.x,
                center.y + 7.0,
                1.6,
                Color::new(0.08, 0.06, 0.04, 1.0),
            );
        }
        PromptTone::Good => {
            draw_circle(
                center.x,
                center.y,
                11.0,
                Color::new(color.r, color.g, color.b, 0.28),
            );
            draw_line(
                center.x - 6.0,
                center.y,
                center.x - 1.0,
                center.y + 5.0,
                2.4,
                color,
            );
            draw_line(
                center.x - 1.0,
                center.y + 5.0,
                center.x + 7.0,
                center.y - 6.0,
                2.4,
                color,
            );
        }
        PromptTone::Action | PromptTone::Neutral => {
            draw_circle_lines(center.x, center.y, 10.0, 1.5, color);
            draw_circle(center.x, center.y, 3.0, color);
        }
    }
}

fn readable_token_text(color: Color) -> Color {
    let luminance = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;
    if luminance > 0.54 {
        Color::new(0.035, 0.040, 0.048, 1.0)
    } else {
        Color::new(0.96, 0.96, 0.92, 1.0)
    }
}

fn status_panel_rect() -> Rect {
    Rect::new(18.0, 16.0, 190.0, 166.0)
}

fn carried_card_rect() -> Rect {
    Rect::new(24.0, LOGICAL_HEIGHT - 110.0, 300.0, 86.0)
}

fn prompt_rect() -> Rect {
    Rect::new(
        (LOGICAL_WIDTH - 420.0) * 0.5,
        LOGICAL_HEIGHT - 76.0,
        420.0,
        52.0,
    )
}

fn hud_border() -> Color {
    Color::new(0.68, 0.64, 0.58, 0.56)
}

fn subtle_border() -> Color {
    Color::new(0.54, 0.50, 0.46, 0.42)
}

#[derive(Debug)]
struct NoticeRow {
    key: Option<&'static str>,
    text: String,
    tone: NoticeTone,
}

#[derive(Debug, Clone, Copy)]
enum NoticeTone {
    Tool,
    Warning,
    Scanner,
}

impl NoticeTone {
    fn accent(self) -> Color {
        match self {
            NoticeTone::Tool => Color::new(0.98, 0.72, 0.26, 1.0),
            NoticeTone::Warning => Color::new(0.98, 0.54, 0.26, 1.0),
            NoticeTone::Scanner => Color::new(0.46, 0.94, 0.96, 1.0),
        }
    }

    fn text_color(self) -> Color {
        match self {
            NoticeTone::Tool => dark::TEXT,
            NoticeTone::Warning => Color::new(0.98, 0.72, 0.42, 1.0),
            NoticeTone::Scanner => Color::new(0.58, 0.94, 0.96, 1.0),
        }
    }
}

#[derive(Debug)]
struct PromptVisual {
    key: Option<&'static str>,
    message: String,
    tone: PromptTone,
}

impl PromptVisual {
    fn action(key: &'static str, message: impl Into<String>) -> Self {
        Self {
            key: Some(key),
            message: message.into(),
            tone: PromptTone::Action,
        }
    }

    fn warning(message: impl Into<String>) -> Self {
        Self {
            key: None,
            message: message.into(),
            tone: PromptTone::Warning,
        }
    }

    fn neutral(message: impl Into<String>) -> Self {
        Self {
            key: None,
            message: message.into(),
            tone: PromptTone::Neutral,
        }
    }

    fn good(message: impl Into<String>) -> Self {
        Self {
            key: None,
            message: message.into(),
            tone: PromptTone::Good,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PromptTone {
    Action,
    Good,
    Warning,
    Neutral,
}

impl PromptTone {
    fn border(self) -> Color {
        match self {
            PromptTone::Action => Color::new(0.92, 0.72, 0.34, 0.72),
            PromptTone::Good => Color::new(0.42, 0.92, 0.56, 0.72),
            PromptTone::Warning => Color::new(0.98, 0.62, 0.26, 0.76),
            PromptTone::Neutral => Color::new(0.58, 0.62, 0.68, 0.48),
        }
    }

    fn text_color(self) -> Color {
        match self {
            PromptTone::Action | PromptTone::Good => dark::TEXT_BRIGHT,
            PromptTone::Warning => Color::new(1.0, 0.76, 0.42, 1.0),
            PromptTone::Neutral => dark::TEXT,
        }
    }
}
