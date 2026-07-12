use crate::data::DisplayDef;
use crate::state::toy_matches_display;
use crate::ui::signs::draw_stock_sign;
use crate::ui::wood::{draw_dark_trim, draw_wood_cube};
use crate::ui::UiContext;
use macroquad::prelude::*;

/// Visual family of a display, derived from its id so displays.json can add
/// any number of displays without new Rust dispatch arms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DisplayStyle {
    Wall,
    Bin,
    Pegboard,
    Shelf,
    Table,
    Generic,
}

pub(crate) fn display_style(display: &DisplayDef) -> DisplayStyle {
    let id = display.id.as_str();
    if id.contains("pegboard") {
        DisplayStyle::Pegboard
    } else if id.contains("wall") {
        DisplayStyle::Wall
    } else if id.contains("shelf") {
        DisplayStyle::Shelf
    } else if id.contains("table") {
        DisplayStyle::Table
    } else if id.contains("bin") {
        DisplayStyle::Bin
    } else {
        DisplayStyle::Generic
    }
}

pub(crate) fn draw_displays(ctx: &UiContext<'_>) {
    for display in &ctx.data.displays {
        let accent = accent_color(display, 1.0);
        let is_complete = ctx.session.is_display_complete(&display.id);

        match display_style(display) {
            DisplayStyle::Wall => draw_wall_display(display, accent),
            DisplayStyle::Bin => draw_dragon_bin(display, accent),
            DisplayStyle::Pegboard => draw_robot_pegboard(display, accent),
            DisplayStyle::Shelf => draw_board_shelf(display, accent),
            DisplayStyle::Table => draw_blocks_table(display, accent),
            DisplayStyle::Generic => draw_generic_display(display, accent),
        }
        draw_stock_sign(display, accent);

        if is_complete {
            draw_completion_lights(display, accent);
        }

        if ctx
            .session
            .active_toy()
            .is_some_and(|toy| ctx.session.scanner_enabled() && toy_matches_display(toy, display))
        {
            draw_scanner_guidance(display);
        }
    }
}

pub(crate) fn draw_repair_benches(ctx: &UiContext<'_>) {
    let carrying_part = ctx
        .session
        .active_toy()
        .is_some_and(|toy| toy.is_repair_part());
    let player = ctx.session.player.position.to_vec2();

    for bench in &ctx.data.layout.benches {
        let center = vec3(bench.x, 0.54, bench.y);
        draw_wood_cube(center, vec3(bench.w, 0.24, bench.h), 70);
        draw_cube(
            center + vec3(0.0, 0.15, 0.0),
            vec3(bench.w - 0.30, 0.045, bench.h - 0.28),
            None,
            Color::new(0.18, 0.23, 0.25, 1.0),
        );
        for x in [-0.42, 0.42] {
            for z in [-0.34, 0.34] {
                draw_wood_cube(
                    center + vec3(bench.w * x, -0.42, bench.h * z),
                    vec3(0.18, 0.82, 0.18),
                    71,
                );
            }
        }

        draw_bench_worktop(center, bench);

        if carrying_part {
            let near =
                player.distance_squared(vec2(bench.x, bench.y)) <= bench.radius * bench.radius;
            let color = if near {
                Color::new(0.34, 0.95, 0.60, 0.94)
            } else {
                Color::new(0.42, 0.95, 0.96, 0.72)
            };
            draw_cube_wires(
                center + vec3(0.0, 0.25, 0.0),
                vec3(bench.w + 0.24, 1.05, bench.h + 0.24),
                color,
            );
        }
    }
}

/// Worktop dressing along the back strip of a repair bench — the front half
/// stays clear for the two part slots. Desk lamp, screwdriver, screw
/// scatter, and a toolbox sell "repairs happen here" from a distance.
fn draw_bench_worktop(center: Vec3, bench: &crate::data::BenchDef) {
    let top = 0.175;
    let back = bench.h * 0.30;
    let metal = Color::new(0.62, 0.65, 0.66, 1.0);

    // Desk lamp: base, stepped arm, warm shade with a glow pool under it.
    let lamp_x = -bench.w * 0.32;
    draw_cube(
        center + vec3(lamp_x, top + 0.02, back),
        vec3(0.14, 0.04, 0.12),
        None,
        Color::new(0.20, 0.23, 0.24, 1.0),
    );
    draw_cube(
        center + vec3(lamp_x, top + 0.14, back),
        vec3(0.03, 0.22, 0.03),
        None,
        metal,
    );
    draw_cube(
        center + vec3(lamp_x + 0.07, top + 0.24, back - 0.05),
        vec3(0.14, 0.03, 0.03),
        None,
        metal,
    );
    draw_cube(
        center + vec3(lamp_x + 0.15, top + 0.21, back - 0.09),
        vec3(0.12, 0.08, 0.12),
        None,
        Color::new(0.24, 0.42, 0.38, 1.0),
    );
    draw_cube(
        center + vec3(lamp_x + 0.15, top + 0.16, back - 0.09),
        vec3(0.06, 0.03, 0.06),
        None,
        Color::new(0.97, 0.88, 0.60, 1.0),
    );
    draw_cube(
        center + vec3(lamp_x + 0.15, top + 0.005, back - 0.09),
        vec3(0.34, 0.006, 0.30),
        None,
        Color::new(0.96, 0.85, 0.55, 0.16),
    );

    // Screwdriver: amber handle, steel shaft, plus a scatter of screws.
    draw_cube(
        center + vec3(0.06, top + 0.025, back + 0.02),
        vec3(0.10, 0.035, 0.04),
        None,
        Color::new(0.86, 0.56, 0.22, 1.0),
    );
    draw_cube(
        center + vec3(0.17, top + 0.02, back + 0.02),
        vec3(0.13, 0.018, 0.018),
        None,
        metal,
    );
    for (offset_x, offset_z) in [
        (0.30_f32, -0.04_f32),
        (0.34, 0.05),
        (0.27, 0.08),
        (0.38, 0.0),
    ] {
        draw_cube(
            center + vec3(offset_x, top + 0.012, back + offset_z),
            vec3(0.022, 0.022, 0.022),
            None,
            metal,
        );
    }

    // Red toolbox anchoring the right end.
    draw_cube(
        center + vec3(bench.w * 0.33, top + 0.07, back - 0.02),
        vec3(0.34, 0.13, 0.16),
        None,
        Color::new(0.72, 0.20, 0.18, 1.0),
    );
    draw_cube(
        center + vec3(bench.w * 0.33, top + 0.145, back - 0.02),
        vec3(0.12, 0.025, 0.05),
        None,
        Color::new(0.30, 0.30, 0.30, 1.0),
    );
}

pub(crate) fn draw_aisle_shelving(ctx: &UiContext<'_>) {
    for (index, shelf) in ctx.data.layout.shelving.iter().enumerate() {
        let center_x = shelf.x + shelf.w * 0.5;
        let center_z = shelf.y + shelf.h * 0.5;
        let seed = 75 + index * 3;

        // Central spine of the gondola.
        draw_wood_cube(
            vec3(center_x, 0.76, center_z),
            vec3(shelf.w, 1.52, 0.24),
            seed,
        );
        // Shelf boards on both faces, left bare: the night-shift store is
        // waiting to be tidied, not already stocked.
        for level in 0..3 {
            let y = 0.34 + level as f32 * 0.44;
            for side in [-1.0_f32, 1.0] {
                draw_wood_cube(
                    vec3(center_x, y, center_z + side * shelf.h * 0.25),
                    vec3(shelf.w * 0.97, 0.06, shelf.h * 0.46),
                    seed + level,
                );
            }
        }
        // End caps.
        for side in [-1.0_f32, 1.0] {
            draw_dark_trim(
                vec3(center_x + side * shelf.w * 0.5, 0.76, center_z),
                vec3(0.10, 1.52, shelf.h),
            );
        }
        draw_wood_cube(
            vec3(center_x, 1.55, center_z),
            vec3(shelf.w, 0.06, shelf.h),
            seed + 1,
        );
    }
}

/// Checkout counters: wooden base, dark worktop, and register clutter —
/// screen, card reader, basket stack, and impulse candy boxes.
pub(crate) fn draw_checkout_counters(ctx: &UiContext<'_>) {
    let dark = Color::new(0.16, 0.19, 0.20, 1.0);
    let metal = Color::new(0.55, 0.58, 0.60, 1.0);

    for counter in &ctx.data.layout.counters {
        let center_x = counter.x + counter.w * 0.5;
        let center_z = counter.y + counter.h * 0.5;
        let top_y = 0.98;

        draw_wood_cube(
            vec3(center_x, 0.48, center_z),
            vec3(counter.w, 0.96, counter.h),
            82,
        );
        draw_cube(
            vec3(center_x, top_y, center_z),
            vec3(counter.w + 0.12, 0.05, counter.h + 0.12),
            None,
            dark,
        );

        // Register: body, standing screen, button strip.
        let register_x = counter.x + counter.w * 0.24;
        draw_cube(
            vec3(register_x, top_y + 0.13, center_z),
            vec3(0.34, 0.20, 0.30),
            None,
            Color::new(0.24, 0.27, 0.29, 1.0),
        );
        draw_cube(
            vec3(register_x, top_y + 0.34, center_z + 0.08),
            vec3(0.30, 0.22, 0.03),
            None,
            dark,
        );
        draw_cube(
            vec3(register_x, top_y + 0.34, center_z + 0.062),
            vec3(0.24, 0.16, 0.01),
            None,
            Color::new(0.46, 0.86, 0.74, 1.0),
        );
        draw_cube(
            vec3(register_x, top_y + 0.24, center_z - 0.10),
            vec3(0.26, 0.02, 0.10),
            None,
            metal,
        );

        // Card reader on a slim stand.
        let reader_x = counter.x + counter.w * 0.48;
        draw_cube(
            vec3(reader_x, top_y + 0.10, center_z - 0.22),
            vec3(0.03, 0.16, 0.03),
            None,
            metal,
        );
        draw_cube(
            vec3(reader_x, top_y + 0.21, center_z - 0.22),
            vec3(0.11, 0.13, 0.05),
            None,
            dark,
        );

        // Stack of shopping baskets at the far end.
        let basket_x = counter.x + counter.w * 0.80;
        for tier in 0..3 {
            let tint = if tier % 2 == 0 {
                Color::new(0.80, 0.34, 0.28, 1.0)
            } else {
                Color::new(0.86, 0.44, 0.36, 1.0)
            };
            draw_cube(
                vec3(basket_x, top_y + 0.07 + tier as f32 * 0.075, center_z),
                vec3(0.42, 0.06, 0.30),
                None,
                tint,
            );
        }

        // Impulse candy boxes along the front edge.
        for (slot, tint) in [
            Color::new(0.92, 0.72, 0.30, 1.0),
            Color::new(0.52, 0.68, 0.88, 1.0),
            Color::new(0.62, 0.82, 0.52, 1.0),
        ]
        .iter()
        .enumerate()
        {
            draw_cube(
                vec3(
                    counter.x + counter.w * (0.58 + slot as f32 * 0.09),
                    top_y + 0.065,
                    center_z + counter.h * 0.30,
                ),
                vec3(0.10, 0.08, 0.10),
                None,
                *tint,
            );
        }
    }
}

pub(crate) fn placed_height_for_slot(display: &DisplayDef, slot_number: usize) -> f32 {
    let row = (slot_number.saturating_sub(1) / 5) as f32;
    match display_style(display) {
        DisplayStyle::Wall => 0.62 + row * 0.34,
        DisplayStyle::Pegboard => 0.70 + row * 0.31,
        DisplayStyle::Shelf => 0.38 + row * 0.31,
        DisplayStyle::Table => 0.84 + row * 0.11,
        DisplayStyle::Bin => 0.46 + row * 0.07,
        DisplayStyle::Generic => 0.54 + row * 0.10,
    }
}

fn draw_wall_display(display: &DisplayDef, accent: Color) {
    let x = display.x + display.w * 0.5;
    let z = display.y + 0.10;
    draw_wood_cube(vec3(x, 1.15, z), vec3(display.w + 0.16, 2.10, 0.24), 20);
    for shelf in 0..4 {
        let y = 0.38 + shelf as f32 * 0.46;
        draw_wood_cube(
            vec3(x, y, z + 0.23),
            vec3(display.w * 0.94, 0.10, 0.44),
            21 + shelf,
        );
    }
    draw_cube(
        vec3(x, 1.86, z + 0.25),
        vec3(display.w * 0.88, 0.05, 0.34),
        None,
        Color::new(accent.r, accent.g, accent.b, 0.55),
    );
}

fn draw_dragon_bin(display: &DisplayDef, accent: Color) {
    let center = display_center(display, 0.32);
    draw_wood_cube(center, vec3(display.w, 0.62, display.h), 30);
    draw_cube_wires(center, vec3(display.w, 0.66, display.h), accent);
    for side in [-1.0, 1.0] {
        draw_wood_cube(
            center + vec3(0.0, 0.42, side * display.h * 0.42),
            vec3(display.w, 0.18, 0.16),
            31,
        );
    }
    for x in [-0.42, 0.42] {
        draw_dark_trim(
            center + vec3(display.w * x, 0.42, 0.0),
            vec3(0.12, 0.20, display.h),
        );
    }
}

fn draw_robot_pegboard(display: &DisplayDef, accent: Color) {
    let x = display.x + display.w * 0.5;
    let z = display.y + display.h - 0.1;
    draw_wood_cube(vec3(x, 1.18, z), vec3(display.w, 2.0, 0.24), 40);
    draw_cube(
        vec3(x, 1.18, z - 0.14),
        vec3(display.w * 0.88, 1.82, 0.045),
        None,
        Color::new(0.13, 0.17, 0.18, 1.0),
    );
    for row in 0..4 {
        for column in 0..5 {
            let peg_x = display.x + 0.38 + column as f32 * 0.78;
            let peg_y = 0.52 + row as f32 * 0.36;
            draw_cube(
                vec3(peg_x, peg_y, z - 0.24),
                vec3(0.11, 0.07, 0.30),
                None,
                if (row + column) % 2 == 0 {
                    accent
                } else {
                    Color::new(0.58, 0.66, 0.70, 1.0)
                },
            );
        }
    }
}

fn draw_board_shelf(display: &DisplayDef, accent: Color) {
    let x = display.x + display.w * 0.5;
    let z = display.y + display.h * 0.5;
    draw_wood_cube(
        vec3(x, 0.90, z),
        vec3(display.w, 1.70, display.h * 0.38),
        50,
    );
    for shelf in 0..4 {
        draw_wood_cube(
            vec3(x, 0.24 + shelf as f32 * 0.42, z - 0.43),
            vec3(display.w * 0.92, 0.10, 0.36),
            51 + shelf,
        );
    }
    for side in [-0.46, 0.46] {
        draw_dark_trim(
            vec3(x + display.w * side, 0.90, z - 0.43),
            vec3(0.10, 1.62, 0.34),
        );
    }
    draw_cube(
        vec3(x, 1.58, z - 0.65),
        vec3(display.w * 0.82, 0.06, 0.08),
        None,
        accent,
    );
}

fn draw_blocks_table(display: &DisplayDef, accent: Color) {
    let center = display_center(display, 0.66);
    draw_wood_cube(center, vec3(display.w, 0.28, display.h), 60);
    draw_cube(
        center + vec3(0.0, 0.17, 0.0),
        vec3(display.w * 0.88, 0.045, display.h * 0.86),
        None,
        Color::new(accent.r, accent.g, accent.b, 0.30),
    );
    for (index, x) in [-0.38, 0.38].iter().enumerate() {
        for (z_index, z) in [-0.34, 0.34].iter().enumerate() {
            draw_wood_cube(
                center + vec3(display.w * x, -0.42, display.h * z),
                vec3(0.20, 0.82, 0.20),
                61 + index + z_index,
            );
        }
    }
}

fn draw_generic_display(display: &DisplayDef, accent: Color) {
    draw_cube(
        display_center(display, 0.38),
        vec3(display.w, 0.76, display.h),
        None,
        accent,
    );
}

fn draw_completion_lights(display: &DisplayDef, accent: Color) {
    let center = display_center(display, 1.65);
    draw_sphere(
        center,
        0.22,
        None,
        Color::new(accent.r, accent.g, accent.b, 0.78),
    );
    for index in 0..6 {
        let angle = index as f32 / 6.0 * std::f32::consts::TAU;
        let offset = vec3(
            angle.cos() * 0.55,
            0.10 + index as f32 * 0.035,
            angle.sin() * 0.55,
        );
        draw_sphere(center + offset, 0.07, None, accent);
    }
}

fn draw_scanner_guidance(display: &DisplayDef) {
    let color = Color::new(0.42, 0.95, 0.96, 0.92);
    let center = display_center(display, 0.88);
    draw_cube_wires(
        center,
        vec3(display.w + 0.34, 1.36, display.h + 0.34),
        color,
    );
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

fn display_center(display: &DisplayDef, height: f32) -> Vec3 {
    vec3(
        display.x + display.w * 0.5,
        height,
        display.y + display.h * 0.5,
    )
}

fn accent_color(display: &DisplayDef, alpha: f32) -> Color {
    Color::new(
        display.accent[0],
        display.accent[1],
        display.accent[2],
        display.accent[3] * alpha,
    )
}
