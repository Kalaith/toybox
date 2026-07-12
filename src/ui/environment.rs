use crate::data::GameData;
use crate::ui::wood::{draw_dark_trim, draw_wood_cube, wood_tone};
use macroquad::prelude::*;

pub(crate) fn draw_shop_environment(data: &GameData) {
    draw_sky_glow(data);
    draw_floor(data);
    draw_zone_rugs(data);
    draw_zone_lamps(data);
    draw_walls(data);
    draw_ceiling(data);
    draw_skylights(data);
    draw_front_window(data);
}

/// Recessed night-sky skylights: dark glass, star field, moon sliver, and a
/// cool moonlight pool on the floor below to contrast the warm lamps.
fn draw_skylights(data: &GameData) {
    let ceiling_bottom = data.layout.wall.height + 0.06;
    for skylight in &data.layout.skylights {
        let center_x = skylight.x + skylight.w * 0.5;
        let center_z = skylight.y + skylight.h * 0.5;

        // Frame ring just under the ceiling, then the glass panel.
        for side in [-1.0_f32, 1.0] {
            draw_dark_trim(
                vec3(
                    center_x,
                    ceiling_bottom - 0.02,
                    center_z + side * skylight.h * 0.5,
                ),
                vec3(skylight.w + 0.16, 0.08, 0.16),
            );
            draw_dark_trim(
                vec3(
                    center_x + side * skylight.w * 0.5,
                    ceiling_bottom - 0.02,
                    center_z,
                ),
                vec3(0.16, 0.08, skylight.h + 0.16),
            );
        }
        draw_cube(
            vec3(center_x, ceiling_bottom + 0.01, center_z),
            vec3(skylight.w, 0.02, skylight.h),
            None,
            Color::new(0.030, 0.050, 0.105, 1.0),
        );

        // Star field and a low moon sliver.
        for star in 0..16 {
            let sx = skylight.x + 0.3 + ((star * 47) % 90) as f32 / 90.0 * (skylight.w - 0.6);
            let sz = skylight.y + 0.3 + ((star * 31) % 70) as f32 / 70.0 * (skylight.h - 0.6);
            let size = 0.024 + (star % 3) as f32 * 0.008;
            draw_cube(
                vec3(sx, ceiling_bottom - 0.005, sz),
                vec3(size, 0.012, size),
                None,
                Color::new(0.86, 0.92, 1.0, 0.90),
            );
        }
        draw_cube(
            vec3(
                skylight.x + skylight.w * 0.78,
                ceiling_bottom - 0.008,
                skylight.y + skylight.h * 0.30,
            ),
            vec3(0.16, 0.014, 0.16),
            None,
            Color::new(0.95, 0.90, 0.68, 1.0),
        );

        // Moonlight pool on the floor beneath.
        draw_cube(
            vec3(center_x, 0.055, center_z),
            vec3(skylight.w + 1.0, 0.006, skylight.h + 1.0),
            None,
            Color::new(0.72, 0.82, 0.98, 0.06),
        );
        draw_cube(
            vec3(center_x, 0.059, center_z),
            vec3(skylight.w * 0.7, 0.006, skylight.h * 0.7),
            None,
            Color::new(0.78, 0.87, 1.0, 0.09),
        );
    }
}

/// Two pendant lamps per zone with warm floor pools — the night shift is
/// lit, not fluorescent-bright. Offsets keep them clear of the zone signs.
fn draw_zone_lamps(data: &GameData) {
    let shade = Color::new(0.15, 0.18, 0.17, 1.0);
    let warm = Color::new(0.95, 0.84, 0.55, 1.0);
    let ceiling_y = data.layout.wall.height + 0.06;

    for (zone_index, zone) in data.layout.zones.iter().enumerate() {
        for (lamp_index, offset) in [-0.26_f32, 0.26].iter().enumerate() {
            let x = zone.x + zone.w * (0.5 + offset);
            let drift = ((zone_index * 3 + lamp_index * 5) % 5) as f32 * 0.3 - 0.6;
            let z = zone.y + zone.h * 0.5 + drift;

            // Rod, shade, and glowing bulb.
            draw_cube(
                vec3(x, ceiling_y - 0.20, z),
                vec3(0.03, 0.40, 0.03),
                None,
                shade,
            );
            draw_cube(vec3(x, 2.00, z), vec3(0.30, 0.10, 0.30), None, shade);
            draw_cube(
                vec3(x, 1.945, z),
                vec3(0.34, 0.02, 0.34),
                None,
                Color::new(0.42, 0.36, 0.26, 1.0),
            );
            draw_cube(vec3(x, 1.91, z), vec3(0.09, 0.07, 0.09), None, warm);

            // Soft warm pool on the floor, brighter core inside a wide wash.
            draw_cube(
                vec3(x, 0.058, z),
                vec3(2.8, 0.006, 2.8),
                None,
                Color::new(0.95, 0.82, 0.50, 0.07),
            );
            draw_cube(
                vec3(x, 0.062, z),
                vec3(1.5, 0.006, 1.5),
                None,
                Color::new(0.96, 0.86, 0.55, 0.10),
            );
        }
    }
}

/// Accent-tinted rug per zone so each department reads at floor level,
/// not just from the hanging signs.
fn draw_zone_rugs(data: &GameData) {
    for zone in &data.layout.zones {
        let accent = &zone.accent;
        let center_x = zone.x + zone.w * 0.5;
        let center_z = zone.y + zone.h * 0.5;
        let rug_w = zone.w * 0.78;
        let rug_d = zone.h * 0.78;

        draw_cube(
            vec3(center_x, 0.046, center_z),
            vec3(rug_w, 0.012, rug_d),
            None,
            Color::new(accent[0], accent[1], accent[2], 0.14),
        );

        // Border band and corner ticks in a stronger accent.
        let border = Color::new(accent[0], accent[1], accent[2], 0.38);
        let half_w = rug_w * 0.5;
        let half_d = rug_d * 0.5;
        for side in [-1.0_f32, 1.0] {
            draw_cube(
                vec3(center_x, 0.052, center_z + side * (half_d - 0.09)),
                vec3(rug_w, 0.010, 0.14),
                None,
                border,
            );
            draw_cube(
                vec3(center_x + side * (half_w - 0.09), 0.052, center_z),
                vec3(0.14, 0.010, rug_d - 0.36),
                None,
                border,
            );
        }
    }
}

fn draw_sky_glow(data: &GameData) {
    let window = &data.layout.window;
    draw_cube(
        vec3(window.x, window.center_y + 0.02, -0.24),
        vec3(window.width + 0.18, window.height + 0.04, 0.08),
        None,
        Color::new(0.035, 0.060, 0.115, 1.0),
    );
    draw_sphere(
        vec3(window.x + 0.88, window.center_y + 0.36, -0.31),
        0.13,
        None,
        Color::new(0.95, 0.87, 0.58, 1.0),
    );

    for index in 0..13 {
        let x = window.x - 1.08 + (index % 5) as f32 * 0.55 + (index / 5) as f32 * 0.07;
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

fn draw_floor(data: &GameData) {
    let width = data.config.room_width;
    let depth = data.config.room_height;
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

fn draw_walls(data: &GameData) {
    let width = data.config.room_width;
    let depth = data.config.room_height;
    let wall_spec = &data.layout.wall;
    let center_y = wall_spec.height * 0.5;
    let inset = wall_spec.thickness * 0.5 + 0.01;
    let wall = Color::new(0.24, 0.29, 0.28, 1.0);
    let side_wall = Color::new(0.20, 0.25, 0.25, 1.0);

    draw_cube(
        vec3(width * 0.5, center_y, -inset),
        vec3(width, wall_spec.height, wall_spec.thickness),
        None,
        wall,
    );
    draw_cube(
        vec3(width * 0.5, center_y, depth + inset),
        vec3(width, wall_spec.height, wall_spec.thickness),
        None,
        Color::new(0.18, 0.22, 0.22, 1.0),
    );
    draw_cube(
        vec3(-inset, center_y, depth * 0.5),
        vec3(wall_spec.thickness, wall_spec.height, depth),
        None,
        side_wall,
    );
    draw_cube(
        vec3(width + inset, center_y, depth * 0.5),
        vec3(wall_spec.thickness, wall_spec.height, depth),
        None,
        Color::new(0.22, 0.27, 0.26, 1.0),
    );

    draw_wall_trim(data);
    draw_wall_panels(data);
}

fn draw_wall_trim(data: &GameData) {
    let width = data.config.room_width;
    let depth = data.config.room_height;
    let top_y = data.layout.wall.height - 0.04;
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
    draw_dark_trim(vec3(width * 0.5, top_y, -0.01), vec3(width, 0.12, 0.12));
    draw_dark_trim(
        vec3(width * 0.5, top_y, depth + 0.01),
        vec3(width, 0.12, 0.12),
    );
}

fn draw_wall_panels(data: &GameData) {
    let width = data.config.room_width;
    let depth = data.config.room_height;
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

fn draw_ceiling(data: &GameData) {
    let width = data.config.room_width;
    let depth = data.config.room_height;
    let ceiling_y = data.layout.wall.height + 0.12;
    draw_cube(
        vec3(width * 0.5, ceiling_y, depth * 0.5),
        vec3(width, 0.12, depth),
        None,
        Color::new(0.13, 0.12, 0.105, 1.0),
    );
    for index in 0..5 {
        let z = 0.9 + index as f32 * (depth - 1.8) / 4.0;
        draw_wood_cube(
            vec3(width * 0.5, ceiling_y - 0.09, z),
            vec3(width, 0.12, 0.12),
            index + 14,
        );
    }
}

fn draw_front_window(data: &GameData) {
    let window = &data.layout.window;
    let center = vec3(window.x, window.center_y, 0.02);
    draw_dark_trim(center, vec3(window.width, window.height, 0.10));
    draw_cube(
        center + vec3(0.0, 0.0, -0.045),
        vec3(window.width - 0.30, window.height - 0.28, 0.04),
        None,
        Color::new(0.07, 0.11, 0.19, 0.92),
    );
    draw_dark_trim(
        center + vec3(0.0, 0.0, -0.07),
        vec3(0.08, window.height - 0.16, 0.06),
    );
    draw_dark_trim(
        center + vec3(0.0, 0.0, -0.07),
        vec3(window.width - 0.20, 0.08, 0.06),
    );
    draw_cube_wires(
        center + vec3(0.0, 0.0, -0.075),
        vec3(window.width - 0.18, window.height - 0.16, 0.07),
        Color::new(0.82, 0.90, 1.0, 0.64),
    );
}
