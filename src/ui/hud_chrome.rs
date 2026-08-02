//! Reusable HUD chrome: panels, keycaps, badges, and the tone vocabulary
//! the gameplay panels in [`super::hud`] compose out of.

use super::hud_icons::{brighten_color, category_icon, draw_icon};
use crate::state::ToyState;
use crate::toys::{toy_color, toy_profile};
use crate::ui::widgets::draw_fitted_text;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, measure_ui_text};

pub(in crate::ui) fn draw_toy_badge(rect: Rect, toy: &ToyState) {
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

pub(in crate::ui) fn draw_identity_token(rect: Rect, toy: &ToyState) {
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

pub(in crate::ui) fn draw_hud_panel(rect: Rect, fill: Color, border: Color) {
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

pub(in crate::ui) fn draw_keycap(rect: Rect, label: &str, large: bool) {
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

pub(in crate::ui) fn draw_progress_bar(rect: Rect, progress: f32, accent: Color) {
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

pub(in crate::ui) fn draw_divider(x: f32, y: f32, width: f32) {
    draw_line(
        x + 1.0,
        y,
        x + width - 1.0,
        y,
        1.0,
        Color::new(0.64, 0.58, 0.50, 0.22),
    );
}

pub(in crate::ui) fn draw_notice_dot(center: Vec2, color: Color) {
    draw_circle(
        center.x,
        center.y,
        7.0,
        Color::new(color.r, color.g, color.b, 0.22),
    );
    draw_circle(center.x, center.y, 3.5, color);
}

pub(in crate::ui) fn draw_prompt_status_icon(center: Vec2, tone: PromptTone) {
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

pub(in crate::ui) fn readable_token_text(color: Color) -> Color {
    let luminance = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;
    if luminance > 0.54 {
        Color::new(0.035, 0.040, 0.048, 1.0)
    } else {
        Color::new(0.96, 0.96, 0.92, 1.0)
    }
}

#[derive(Debug)]
pub(in crate::ui) struct NoticeRow {
    pub(in crate::ui) key: Option<&'static str>,
    pub(in crate::ui) text: String,
    pub(in crate::ui) tone: NoticeTone,
}

#[derive(Debug, Clone, Copy)]
pub(in crate::ui) enum NoticeTone {
    Tool,
    Warning,
    Scanner,
}

impl NoticeTone {
    pub(in crate::ui) fn accent(self) -> Color {
        match self {
            NoticeTone::Tool => Color::new(0.98, 0.72, 0.26, 1.0),
            NoticeTone::Warning => Color::new(0.98, 0.54, 0.26, 1.0),
            NoticeTone::Scanner => Color::new(0.46, 0.94, 0.96, 1.0),
        }
    }

    pub(in crate::ui) fn text_color(self) -> Color {
        match self {
            NoticeTone::Tool => dark::TEXT,
            NoticeTone::Warning => Color::new(0.98, 0.72, 0.42, 1.0),
            NoticeTone::Scanner => Color::new(0.58, 0.94, 0.96, 1.0),
        }
    }
}

#[derive(Debug)]
pub(in crate::ui) struct PromptVisual {
    pub(in crate::ui) key: Option<&'static str>,
    pub(in crate::ui) message: String,
    pub(in crate::ui) tone: PromptTone,
}

impl PromptVisual {
    pub(in crate::ui) fn action(key: &'static str, message: impl Into<String>) -> Self {
        Self {
            key: Some(key),
            message: message.into(),
            tone: PromptTone::Action,
        }
    }

    pub(in crate::ui) fn warning(message: impl Into<String>) -> Self {
        Self {
            key: None,
            message: message.into(),
            tone: PromptTone::Warning,
        }
    }

    pub(in crate::ui) fn neutral(message: impl Into<String>) -> Self {
        Self {
            key: None,
            message: message.into(),
            tone: PromptTone::Neutral,
        }
    }

    pub(in crate::ui) fn good(message: impl Into<String>) -> Self {
        Self {
            key: None,
            message: message.into(),
            tone: PromptTone::Good,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::ui) enum PromptTone {
    Action,
    Good,
    Warning,
    Neutral,
}

impl PromptTone {
    pub(in crate::ui) fn border(self) -> Color {
        match self {
            PromptTone::Action => Color::new(0.92, 0.72, 0.34, 0.72),
            PromptTone::Good => Color::new(0.42, 0.92, 0.56, 0.72),
            PromptTone::Warning => Color::new(0.98, 0.62, 0.26, 0.76),
            PromptTone::Neutral => Color::new(0.58, 0.62, 0.68, 0.48),
        }
    }

    pub(in crate::ui) fn text_color(self) -> Color {
        match self {
            PromptTone::Action | PromptTone::Good => dark::TEXT_BRIGHT,
            PromptTone::Warning => Color::new(1.0, 0.76, 0.42, 1.0),
            PromptTone::Neutral => dark::TEXT,
        }
    }
}
