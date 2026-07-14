//! Primitive HUD icons used by the gameplay overlay.

use crate::data::ToyCategory;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub(in crate::ui) enum IconKind {
    Star,
    Crate,
    Plush,
    Dragon,
    Blocks,
    Robot,
    BoardGame,
}

pub(in crate::ui) fn draw_icon(kind: IconKind, center: Vec2, radius: f32, color: Color) {
    match kind {
        IconKind::Star => draw_star_icon(center, radius, color),
        IconKind::Crate => draw_open_box_icon(center, radius, color),
        IconKind::Plush => draw_plush_icon(center, radius, color),
        IconKind::Dragon => draw_dragon_icon(center, radius, color),
        IconKind::Blocks => draw_blocks_icon(center, radius, color),
        IconKind::Robot => draw_robot_icon(center, radius, color),
        IconKind::BoardGame => draw_board_game_icon(center, radius, color),
    }
}

pub(in crate::ui) fn draw_open_box_icon(center: Vec2, radius: f32, color: Color) {
    let w = radius * 1.55;
    let h = radius * 1.05;
    let left = center.x - w * 0.5;
    let top = center.y - h * 0.36;
    draw_rectangle(left, top, w, h, Color::new(color.r, color.g, color.b, 0.86));
    draw_line(
        left,
        top,
        center.x,
        top - radius * 0.35,
        1.6,
        Color::new(1.0, 0.78, 0.42, 0.80),
    );
    draw_line(
        left + w,
        top,
        center.x,
        top - radius * 0.35,
        1.6,
        Color::new(1.0, 0.78, 0.42, 0.80),
    );
    draw_line(
        center.x,
        top - radius * 0.35,
        center.x,
        top + h,
        1.4,
        Color::new(0.16, 0.08, 0.04, 0.72),
    );
    draw_rectangle_lines(left, top, w, h, 1.4, Color::new(0.16, 0.08, 0.04, 0.82));
}

pub(in crate::ui) fn draw_stopwatch_icon(center: Vec2, radius: f32, color: Color) {
    draw_circle_lines(center.x, center.y, radius, 2.4, color);
    draw_rectangle(
        center.x - radius * 0.30,
        center.y - radius * 1.48,
        radius * 0.60,
        radius * 0.34,
        color,
    );
    draw_line(
        center.x,
        center.y,
        center.x,
        center.y - radius * 0.66,
        2.0,
        color,
    );
    draw_line(
        center.x,
        center.y,
        center.x + radius * 0.45,
        center.y + radius * 0.22,
        2.0,
        color,
    );
}

pub(in crate::ui) fn category_icon(category: ToyCategory) -> IconKind {
    match category {
        ToyCategory::Plushies => IconKind::Plush,
        ToyCategory::TinyDragons => IconKind::Dragon,
        ToyCategory::BuildingBlocks => IconKind::Blocks,
        ToyCategory::ActionFigures => IconKind::Robot,
        ToyCategory::BoardGames => IconKind::BoardGame,
    }
}

pub(in crate::ui) fn brighten_color(color: Color, amount: f32) -> Color {
    macroquad_toolkit::colors::lighten(color, amount)
}

fn draw_star_icon(center: Vec2, radius: f32, color: Color) {
    let mut points = [Vec2::ZERO; 10];
    for (index, point) in points.iter_mut().enumerate() {
        let angle = -std::f32::consts::FRAC_PI_2 + index as f32 * std::f32::consts::PI / 5.0;
        let r = if index % 2 == 0 {
            radius
        } else {
            radius * 0.48
        };
        *point = center + vec2(angle.cos() * r, angle.sin() * r);
    }
    for index in 0..points.len() {
        draw_triangle(
            center,
            points[index],
            points[(index + 1) % points.len()],
            color,
        );
    }
}

fn draw_plush_icon(center: Vec2, radius: f32, color: Color) {
    draw_circle(
        center.x - radius * 0.48,
        center.y - radius * 0.42,
        radius * 0.34,
        color,
    );
    draw_circle(
        center.x + radius * 0.48,
        center.y - radius * 0.42,
        radius * 0.34,
        color,
    );
    draw_circle(center.x, center.y, radius * 0.82, color);
    draw_circle(
        center.x - radius * 0.27,
        center.y - radius * 0.10,
        radius * 0.08,
        Color::new(0.04, 0.035, 0.035, 1.0),
    );
    draw_circle(
        center.x + radius * 0.27,
        center.y - radius * 0.10,
        radius * 0.08,
        Color::new(0.04, 0.035, 0.035, 1.0),
    );
    draw_circle(
        center.x,
        center.y + radius * 0.20,
        radius * 0.12,
        Color::new(0.94, 0.82, 0.62, 0.92),
    );
}

fn draw_dragon_icon(center: Vec2, radius: f32, color: Color) {
    draw_circle(
        center.x - radius * 0.20,
        center.y + radius * 0.06,
        radius * 0.72,
        color,
    );
    draw_circle(
        center.x + radius * 0.60,
        center.y - radius * 0.12,
        radius * 0.38,
        color,
    );
    for index in 0..3 {
        let x = center.x - radius * 0.50 + index as f32 * radius * 0.36;
        draw_triangle(
            vec2(x, center.y - radius * 0.56),
            vec2(x + radius * 0.16, center.y - radius * 0.92),
            vec2(x + radius * 0.32, center.y - radius * 0.56),
            Color::new(1.0, 0.74, 0.22, 0.94),
        );
    }
    draw_circle(
        center.x + radius * 0.70,
        center.y - radius * 0.18,
        radius * 0.06,
        Color::new(0.03, 0.025, 0.02, 1.0),
    );
}

fn draw_blocks_icon(center: Vec2, radius: f32, color: Color) {
    let block = radius * 0.60;
    for (index, offset) in [vec2(-0.54, 0.14), vec2(0.10, 0.14), vec2(-0.22, -0.48)]
        .iter()
        .enumerate()
    {
        let shade = brighten_color(color, index as f32 * 0.06);
        let x = center.x + offset.x * radius;
        let y = center.y + offset.y * radius;
        draw_rectangle(x, y, block, block, shade);
        draw_rectangle_lines(x, y, block, block, 1.2, Color::new(0.04, 0.035, 0.03, 0.78));
    }
}

fn draw_robot_icon(center: Vec2, radius: f32, color: Color) {
    let head = Rect::new(
        center.x - radius * 0.64,
        center.y - radius * 0.48,
        radius * 1.28,
        radius * 0.92,
    );
    draw_rectangle(head.x, head.y, head.w, head.h, color);
    draw_rectangle_lines(
        head.x,
        head.y,
        head.w,
        head.h,
        1.5,
        Color::new(0.04, 0.05, 0.06, 0.82),
    );
    draw_line(
        center.x,
        head.y,
        center.x,
        head.y - radius * 0.42,
        1.4,
        color,
    );
    draw_circle(
        center.x,
        head.y - radius * 0.48,
        radius * 0.12,
        Color::new(1.0, 0.76, 0.24, 0.96),
    );
    draw_circle(
        center.x - radius * 0.28,
        center.y - radius * 0.12,
        radius * 0.08,
        Color::new(0.03, 0.04, 0.05, 1.0),
    );
    draw_circle(
        center.x + radius * 0.28,
        center.y - radius * 0.12,
        radius * 0.08,
        Color::new(0.03, 0.04, 0.05, 1.0),
    );
    draw_line(
        center.x - radius * 0.32,
        center.y + radius * 0.20,
        center.x + radius * 0.32,
        center.y + radius * 0.20,
        1.4,
        Color::new(0.03, 0.04, 0.05, 1.0),
    );
}

fn draw_board_game_icon(center: Vec2, radius: f32, color: Color) {
    let rect = Rect::new(
        center.x - radius * 0.72,
        center.y - radius * 0.52,
        radius * 1.44,
        radius * 1.05,
    );
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        Color::new(0.05, 0.035, 0.03, 0.82),
    );
    draw_line(
        rect.x + radius * 0.24,
        rect.y,
        rect.x + radius * 0.24,
        rect.bottom(),
        1.0,
        Color::new(1.0, 0.92, 0.72, 0.48),
    );
    draw_line(
        rect.x + radius * 0.72,
        rect.y,
        rect.x + radius * 0.72,
        rect.bottom(),
        1.0,
        Color::new(1.0, 0.92, 0.72, 0.48),
    );
    draw_line(
        rect.x,
        rect.y + radius * 0.34,
        rect.right(),
        rect.y + radius * 0.34,
        1.0,
        Color::new(1.0, 0.92, 0.72, 0.48),
    );
}
