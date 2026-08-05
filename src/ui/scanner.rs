//! Everything the Toy Scanner draws into the 3D scene: the halo over the
//! recommended display for a carried toy, the far half of a repair part, and
//! the nearest loose toy bought through the late-shift spotlight service.

use crate::data::DisplayDef;
use crate::ui::fixtures::display_center;
use crate::ui::UiContext;
use macroquad::prelude::*;

const SCANNER_CYAN: Color = Color::new(0.42, 0.95, 0.96, 0.92);

pub(crate) fn draw_scanner_guidance(display: &DisplayDef, recommended: bool) {
    let color = if recommended {
        SCANNER_CYAN
    } else {
        Color::new(0.38, 0.72, 0.76, 0.34)
    };
    let center = display_center(display, 0.88);
    draw_cube_wires(
        center,
        vec3(
            display.w + if recommended { 0.34 } else { 0.18 },
            if recommended { 1.36 } else { 1.12 },
            display.h + if recommended { 0.34 } else { 0.18 },
        ),
        color,
    );
    if !recommended {
        return;
    }
    draw_sphere(center + vec3(0.0, 0.82, 0.0), 0.12, None, color);
    draw_line_3d(
        center + vec3(-display.w * 0.44, 1.58, -display.h * 0.44),
        center + vec3(display.w * 0.44, 1.58, display.h * 0.44),
        color,
    );
    draw_line_3d(
        center + vec3(-display.w * 0.44, 1.58, display.h * 0.44),
        center + vec3(display.w * 0.44, 1.58, -display.h * 0.44),
        color,
    );
}

pub(crate) fn draw_stockroom_spotlight(ctx: &UiContext<'_>) {
    if !ctx.session.stockroom_spotlight_active() {
        return;
    }
    let Some(toy) = ctx.session.stockroom_spotlight_target() else {
        return;
    };
    let ground = vec3(toy.position.x, 0.0, toy.position.y);
    let pulse = (crate::ui::animation_seconds() * 3.2).sin() * 0.5 + 0.5;
    let gold = Color::new(1.0, 0.76, 0.22, 0.92);
    draw_cube(
        ground + vec3(0.0, 1.9, 0.0),
        vec3(0.08, 3.8, 0.08),
        None,
        Color::new(gold.r, gold.g, gold.b, 0.24 + pulse * 0.34),
    );
    draw_cube_wires(ground + vec3(0.0, 0.04, 0.0), vec3(0.78, 0.08, 0.78), gold);
    draw_sphere(
        ground + vec3(0.0, 3.9 + pulse * 0.16, 0.0),
        0.16,
        None,
        gold,
    );
}

/// A tall beacon over the other half of the carried part, drawn through the
/// shelving so the errand is visible from wherever the player is standing.
/// Amber once the counterpart is already waiting on a bench, cyan while it is
/// still loose on the floor.
pub(crate) fn draw_counterpart_beacon(ctx: &UiContext<'_>) {
    if !ctx.session.scanner_enabled(ctx.data) {
        return;
    }
    let Some(counterpart) = ctx.session.carried_counterpart() else {
        return;
    };

    let color = if counterpart.on_bench {
        Color::new(0.98, 0.72, 0.26, 0.90)
    } else {
        SCANNER_CYAN
    };
    let ground = vec3(counterpart.position.x, 0.0, counterpart.position.y);
    let pulse = (crate::ui::animation_seconds() * 2.4).sin() * 0.5 + 0.5;

    // Column, plus a ring at floor level so the exact spot reads once close.
    draw_cube(
        ground + vec3(0.0, 1.6, 0.0),
        vec3(0.06, 3.2, 0.06),
        None,
        Color::new(color.r, color.g, color.b, 0.30 + 0.40 * pulse),
    );
    draw_cube_wires(ground + vec3(0.0, 0.03, 0.0), vec3(0.62, 0.06, 0.62), color);
    draw_sphere(
        ground + vec3(0.0, 3.3 + pulse * 0.10, 0.0),
        0.13,
        None,
        color,
    );
}
