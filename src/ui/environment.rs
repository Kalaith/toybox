use crate::data::GameConfig;
use crate::ui::wood::{draw_dark_trim, draw_wood_cube, wood_tone};
use macroquad::prelude::*;

pub(crate) fn draw_shop_environment(config: &GameConfig) {
    draw_sky_glow(config);
    draw_floor(config);
    draw_walls(config);
    draw_ceiling(config);
    draw_front_window(config);
}

fn draw_sky_glow(config: &GameConfig) {
    let width = config.room_width;
    draw_cube(
        vec3(width - 3.1, 1.36, -0.24),
        vec3(3.0, 1.38, 0.08),
        None,
        Color::new(0.035, 0.060, 0.115, 1.0),
    );
    draw_sphere(
        vec3(width - 2.22, 1.70, -0.31),
        0.13,
        None,
        Color::new(0.95, 0.87, 0.58, 1.0),
    );

    for index in 0..13 {
        let x = width - 4.18 + (index % 5) as f32 * 0.55 + (index / 5) as f32 * 0.07;
        let y = 0.86 + ((index * 7) % 9) as f32 * 0.105;
        let size = 0.030 + (index % 3) as f32 * 0.008;
        draw_cube(
            vec3(x, y, -0.34),
            vec3(size, size, 0.018),
            None,
            Color::new(0.86, 0.92, 1.0, 0.88),
        );
    }
}

fn draw_floor(config: &GameConfig) {
    let width = config.room_width;
    let depth = config.room_height;
    draw_plane(
        vec3(width * 0.5, -0.012, depth * 0.5),
        vec2(width, depth),
        None,
        Color::new(0.20, 0.13, 0.07, 1.0),
    );

    let plank_count = 13;
    let plank_depth = depth / plank_count as f32;
    for row in 0..plank_count {
        let z = plank_depth * (row as f32 + 0.5);
        draw_cube(
            vec3(width * 0.5, 0.012, z),
            vec3(width, 0.020, plank_depth - 0.018),
            None,
            wood_tone(row),
        );
        draw_cube(
            vec3(width * 0.5, 0.026, z + plank_depth * 0.48),
            vec3(width, 0.010, 0.018),
            None,
            Color::new(0.10, 0.065, 0.035, 0.84),
        );
    }

    let board_width = 2.15;
    let columns = (width / board_width).ceil() as usize;
    for column in 1..columns {
        let x = column as f32 * board_width + ((column * 5) % 3) as f32 * 0.08;
        draw_cube(
            vec3(x.min(width - 0.1), 0.030, depth * 0.5),
            vec3(0.018, 0.012, depth),
            None,
            Color::new(0.09, 0.055, 0.030, 0.50),
        );
    }

    for knot in 0..22 {
        let x = 0.9 + ((knot * 47) % 160) as f32 / 160.0 * (width - 1.8);
        let z = 0.7 + ((knot * 31) % 110) as f32 / 110.0 * (depth - 1.4);
        let scale = 0.05 + (knot % 4) as f32 * 0.012;
        draw_cube(
            vec3(x, 0.038, z),
            vec3(scale * 1.8, 0.010, scale),
            None,
            Color::new(0.08, 0.045, 0.025, 0.36),
        );
    }
}

fn draw_walls(config: &GameConfig) {
    let width = config.room_width;
    let depth = config.room_height;
    let wall = Color::new(0.24, 0.29, 0.28, 1.0);
    let side_wall = Color::new(0.20, 0.25, 0.25, 1.0);

    draw_cube(
        vec3(width * 0.5, 1.18, -0.18),
        vec3(width, 2.36, 0.34),
        None,
        wall,
    );
    draw_cube(
        vec3(width * 0.5, 1.18, depth + 0.18),
        vec3(width, 2.36, 0.34),
        None,
        Color::new(0.18, 0.22, 0.22, 1.0),
    );
    draw_cube(
        vec3(-0.18, 1.18, depth * 0.5),
        vec3(0.34, 2.36, depth),
        None,
        side_wall,
    );
    draw_cube(
        vec3(width + 0.18, 1.18, depth * 0.5),
        vec3(0.34, 2.36, depth),
        None,
        Color::new(0.22, 0.27, 0.26, 1.0),
    );

    draw_wall_trim(config);
    draw_wall_panels(config);
}

fn draw_wall_trim(config: &GameConfig) {
    let width = config.room_width;
    let depth = config.room_height;
    draw_dark_trim(vec3(width * 0.5, 0.20, -0.01), vec3(width, 0.20, 0.12));
    draw_dark_trim(
        vec3(width * 0.5, 0.20, depth + 0.01),
        vec3(width, 0.20, 0.12),
    );
    draw_dark_trim(vec3(-0.01, 0.20, depth * 0.5), vec3(0.12, 0.20, depth));
    draw_dark_trim(
        vec3(width + 0.01, 0.20, depth * 0.5),
        vec3(0.12, 0.20, depth),
    );
    draw_dark_trim(vec3(width * 0.5, 2.32, -0.01), vec3(width, 0.12, 0.12));
    draw_dark_trim(
        vec3(width * 0.5, 2.32, depth + 0.01),
        vec3(width, 0.12, 0.12),
    );
}

fn draw_wall_panels(config: &GameConfig) {
    let width = config.room_width;
    let depth = config.room_height;
    for index in 0..7 {
        let x = 1.0 + index as f32 * (width - 2.0) / 6.0;
        draw_wood_cube(vec3(x, 0.86, -0.02), vec3(0.08, 0.86, 0.12), index);
        draw_wood_cube(
            vec3(x, 0.86, depth + 0.02),
            vec3(0.08, 0.86, 0.12),
            index + 2,
        );
    }

    for index in 0..5 {
        let z = 1.2 + index as f32 * (depth - 2.4) / 4.0;
        draw_wood_cube(vec3(-0.02, 0.86, z), vec3(0.12, 0.86, 0.08), index + 5);
        draw_wood_cube(
            vec3(width + 0.02, 0.86, z),
            vec3(0.12, 0.86, 0.08),
            index + 8,
        );
    }
}

fn draw_ceiling(config: &GameConfig) {
    let width = config.room_width;
    let depth = config.room_height;
    draw_cube(
        vec3(width * 0.5, 2.48, depth * 0.5),
        vec3(width, 0.12, depth),
        None,
        Color::new(0.13, 0.12, 0.105, 1.0),
    );
    for index in 0..5 {
        let z = 0.9 + index as f32 * (depth - 1.8) / 4.0;
        draw_wood_cube(
            vec3(width * 0.5, 2.39, z),
            vec3(width, 0.12, 0.12),
            index + 14,
        );
    }
}

fn draw_front_window(config: &GameConfig) {
    let width = config.room_width;
    let center = vec3(width - 3.1, 1.34, 0.02);
    draw_dark_trim(center, vec3(2.82, 1.34, 0.10));
    draw_cube(
        center + vec3(0.0, 0.0, -0.045),
        vec3(2.52, 1.06, 0.04),
        None,
        Color::new(0.07, 0.11, 0.19, 0.92),
    );
    draw_dark_trim(center + vec3(0.0, 0.0, -0.07), vec3(0.08, 1.18, 0.06));
    draw_dark_trim(center + vec3(0.0, 0.0, -0.07), vec3(2.62, 0.08, 0.06));
    draw_cube_wires(
        center + vec3(0.0, 0.0, -0.075),
        vec3(2.64, 1.18, 0.07),
        Color::new(0.82, 0.90, 1.0, 0.64),
    );
}
