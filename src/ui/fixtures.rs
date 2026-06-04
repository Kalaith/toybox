use crate::data::DisplayDef;
use crate::ui::signs::draw_stock_sign;
use crate::ui::wood::{draw_dark_trim, draw_wood_cube};
use crate::ui::UiContext;
use macroquad::prelude::*;

pub(crate) fn draw_displays(ctx: &UiContext<'_>) {
    for display in &ctx.data.displays {
        let accent = accent_color(display, 1.0);
        let is_complete = ctx.session.is_display_complete(&display.id);

        match display.id.as_str() {
            "plushie_wall" => draw_wall_display(display, accent),
            "dragon_bin" => draw_dragon_bin(display, accent),
            "robot_pegboard" => draw_robot_pegboard(display, accent),
            "board_game_shelf" => draw_board_shelf(display, accent),
            "blocks_table" => draw_blocks_table(display, accent),
            _ => draw_generic_display(display, accent),
        }
        draw_stock_sign(display, accent);

        if is_complete {
            draw_completion_lights(display, accent);
        }
    }
}

pub(crate) fn placed_height_for_slot(display: &DisplayDef, slot_number: usize) -> f32 {
    let row = (slot_number.saturating_sub(1) / 5) as f32;
    match display.id.as_str() {
        "plushie_wall" => 0.62 + row * 0.34,
        "robot_pegboard" => 0.70 + row * 0.31,
        "board_game_shelf" => 0.38 + row * 0.31,
        "blocks_table" => 0.84 + row * 0.11,
        "dragon_bin" => 0.46 + row * 0.07,
        _ => 0.54 + row * 0.10,
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
