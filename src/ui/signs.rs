//! Static in-world stock signs with procedural sign-face textures.

use crate::data::DisplayDef;
use macroquad::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static SIGN_TEXTURES: RefCell<HashMap<String, Texture2D>> = RefCell::new(HashMap::new());
}

#[derive(Debug, Clone, Copy)]
struct SignLayout {
    panel_center: Vec3,
    panel_size: Vec3,
    face_z_sign: f32,
    hanging: bool,
}

pub fn draw_stock_sign(display: &DisplayDef, accent: Color) {
    let layout = sign_layout(display);
    let board = Color::new(0.12, 0.09, 0.06, 1.0);

    if layout.hanging {
        draw_hanging_sign_wires(layout, accent);
        draw_cube(
            layout.panel_center,
            layout.panel_size,
            None,
            Color::new(0.10, 0.075, 0.055, 1.0),
        );
        draw_textured_face(
            display,
            accent,
            layout.panel_center + vec3(0.0, 0.0, -0.048),
            vec3(
                layout.panel_size.x * 0.90,
                layout.panel_size.y * 0.64,
                0.030,
            ),
        );
        draw_textured_face(
            display,
            accent,
            layout.panel_center + vec3(0.0, 0.0, 0.048),
            vec3(
                0.030,
                layout.panel_size.y * 0.64,
                layout.panel_size.z * 0.90,
            ),
        );
    } else {
        draw_cube(layout.panel_center, layout.panel_size, None, board);
        draw_textured_face(
            display,
            accent,
            layout.panel_center + vec3(0.0, 0.0, layout.face_z_sign * 0.067),
            vec3(
                layout.panel_size.x * 0.90,
                layout.panel_size.y * 0.66,
                0.030,
            ),
        );
        draw_cube_wires(layout.panel_center, layout.panel_size, accent);
        draw_cube(
            layout.panel_center + vec3(-layout.panel_size.x * 0.42, -0.28, 0.0),
            vec3(0.055, 0.48, 0.055),
            None,
            board,
        );
        draw_cube(
            layout.panel_center + vec3(layout.panel_size.x * 0.42, -0.28, 0.0),
            vec3(0.055, 0.48, 0.055),
            None,
            board,
        );
    }
}

fn draw_textured_face(display: &DisplayDef, accent: Color, center: Vec3, size: Vec3) {
    SIGN_TEXTURES.with(|textures| {
        let mut textures = textures.borrow_mut();
        let texture = textures
            .entry(display.id.clone())
            .or_insert_with(|| build_sign_texture(display, accent));
        draw_cube(center, size, Some(texture), WHITE);
    });
}

fn build_sign_texture(display: &DisplayDef, accent: Color) -> Texture2D {
    let face = Color::new(
        (accent.r * 0.58).clamp(0.0, 1.0),
        (accent.g * 0.58).clamp(0.0, 1.0),
        (accent.b * 0.58).clamp(0.0, 1.0),
        1.0,
    );
    let mut image = Image::gen_image_color(128, 64, Color::new(0.055, 0.038, 0.025, 1.0));
    fill_rect(&mut image, 4, 4, 120, 56, face);
    fill_rect(
        &mut image,
        8,
        8,
        112,
        48,
        Color::new(
            (face.r + 0.06).clamp(0.0, 1.0),
            (face.g + 0.05).clamp(0.0, 1.0),
            (face.b + 0.04).clamp(0.0, 1.0),
            1.0,
        ),
    );
    draw_border(&mut image, Color::new(0.04, 0.030, 0.020, 1.0));

    let (title, theme) = stock_label(display);
    draw_pixel_text_centered(&mut image, title, 15, 2, Color::new(0.98, 0.94, 0.76, 1.0));
    draw_pixel_text_centered(
        &mut image,
        &theme.to_ascii_uppercase(),
        43,
        1,
        Color::new(0.05, 0.045, 0.038, 1.0),
    );

    flip_image_vertical(&mut image);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

fn flip_image_vertical(image: &mut Image) {
    let width = image.width() as u32;
    let height = image.height() as u32;
    for y in 0..height / 2 {
        let opposite_y = height - 1 - y;
        for x in 0..width {
            let top = image.get_pixel(x, y);
            let bottom = image.get_pixel(x, opposite_y);
            image.set_pixel(x, y, bottom);
            image.set_pixel(x, opposite_y, top);
        }
    }
}

fn fill_rect(image: &mut Image, x: u32, y: u32, w: u32, h: u32, color: Color) {
    for yy in y..(y + h).min(image.height() as u32) {
        for xx in x..(x + w).min(image.width() as u32) {
            image.set_pixel(xx, yy, color);
        }
    }
}

fn draw_border(image: &mut Image, color: Color) {
    fill_rect(image, 4, 4, 120, 3, color);
    fill_rect(image, 4, 57, 120, 3, color);
    fill_rect(image, 4, 4, 3, 56, color);
    fill_rect(image, 121, 4, 3, 56, color);
    for x in (14..114).step_by(12) {
        fill_rect(image, x, 9, 5, 2, Color::new(0.95, 0.82, 0.42, 0.78));
        fill_rect(image, x, 53, 5, 2, Color::new(0.95, 0.82, 0.42, 0.62));
    }
}

fn draw_pixel_text_centered(image: &mut Image, text: &str, y: u32, scale: u32, color: Color) {
    let text = text.to_ascii_uppercase();
    let width = pixel_text_width(&text, scale);
    let x = ((image.width() as i32 - width as i32) / 2).max(8) as u32;
    draw_pixel_text(
        image,
        &text,
        x,
        y,
        scale,
        Color::new(0.02, 0.018, 0.014, 0.82),
    );
    draw_pixel_text(image, &text, x, y.saturating_sub(1), scale, color);
}

fn draw_pixel_text(image: &mut Image, text: &str, mut x: u32, y: u32, scale: u32, color: Color) {
    for ch in text.chars() {
        if ch == ' ' {
            x += 4 * scale;
            continue;
        }
        draw_glyph(image, ch, x, y, scale, color);
        x += 6 * scale;
    }
}

fn pixel_text_width(text: &str, scale: u32) -> u32 {
    text.chars()
        .map(|ch| if ch == ' ' { 4 * scale } else { 6 * scale })
        .sum::<u32>()
        .saturating_sub(scale)
}

fn draw_glyph(image: &mut Image, ch: char, x: u32, y: u32, scale: u32, color: Color) {
    for (row, bits) in glyph(ch).iter().enumerate() {
        for (column, bit) in bits.chars().enumerate() {
            if bit == '1' {
                fill_rect(
                    image,
                    x + column as u32 * scale,
                    y + row as u32 * scale,
                    scale,
                    scale,
                    color,
                );
            }
        }
    }
}

fn glyph(ch: char) -> [&'static str; 7] {
    match ch {
        'A' => [
            "01110", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        'B' => [
            "11110", "10001", "10001", "11110", "10001", "10001", "11110",
        ],
        'C' => [
            "01111", "10000", "10000", "10000", "10000", "10000", "01111",
        ],
        'D' => [
            "11110", "10001", "10001", "10001", "10001", "10001", "11110",
        ],
        'E' => [
            "11111", "10000", "10000", "11110", "10000", "10000", "11111",
        ],
        'F' => [
            "11111", "10000", "10000", "11110", "10000", "10000", "10000",
        ],
        'G' => [
            "01111", "10000", "10000", "10011", "10001", "10001", "01111",
        ],
        'H' => [
            "10001", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        'I' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "11111",
        ],
        'J' => [
            "00111", "00010", "00010", "00010", "10010", "10010", "01100",
        ],
        'K' => [
            "10001", "10010", "10100", "11000", "10100", "10010", "10001",
        ],
        'L' => [
            "10000", "10000", "10000", "10000", "10000", "10000", "11111",
        ],
        'M' => [
            "10001", "11011", "10101", "10101", "10001", "10001", "10001",
        ],
        'N' => [
            "10001", "11001", "10101", "10011", "10001", "10001", "10001",
        ],
        'O' => [
            "01110", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'P' => [
            "11110", "10001", "10001", "11110", "10000", "10000", "10000",
        ],
        'Q' => [
            "01110", "10001", "10001", "10001", "10101", "10010", "01101",
        ],
        'R' => [
            "11110", "10001", "10001", "11110", "10100", "10010", "10001",
        ],
        'S' => [
            "01111", "10000", "10000", "01110", "00001", "00001", "11110",
        ],
        'T' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "00100",
        ],
        'U' => [
            "10001", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'V' => [
            "10001", "10001", "10001", "10001", "10001", "01010", "00100",
        ],
        'W' => [
            "10001", "10001", "10001", "10101", "10101", "10101", "01010",
        ],
        'X' => [
            "10001", "10001", "01010", "00100", "01010", "10001", "10001",
        ],
        'Y' => [
            "10001", "10001", "01010", "00100", "00100", "00100", "00100",
        ],
        'Z' => [
            "11111", "00001", "00010", "00100", "01000", "10000", "11111",
        ],
        '0' => [
            "01110", "10001", "10011", "10101", "11001", "10001", "01110",
        ],
        '1' => [
            "00100", "01100", "00100", "00100", "00100", "00100", "01110",
        ],
        '2' => [
            "01110", "10001", "00001", "00010", "00100", "01000", "11111",
        ],
        '3' => [
            "11110", "00001", "00001", "01110", "00001", "00001", "11110",
        ],
        '4' => [
            "00010", "00110", "01010", "10010", "11111", "00010", "00010",
        ],
        '5' => [
            "11111", "10000", "10000", "11110", "00001", "00001", "11110",
        ],
        '6' => [
            "01110", "10000", "10000", "11110", "10001", "10001", "01110",
        ],
        '7' => [
            "11111", "00001", "00010", "00100", "01000", "01000", "01000",
        ],
        '8' => [
            "01110", "10001", "10001", "01110", "10001", "10001", "01110",
        ],
        '9' => [
            "01110", "10001", "10001", "01111", "00001", "00001", "01110",
        ],
        _ => [
            "00000", "00000", "00000", "00000", "00000", "00000", "00000",
        ],
    }
}

fn draw_hanging_sign_wires(layout: SignLayout, accent: Color) {
    let ceiling_y = 2.24;
    let half_x = layout.panel_size.x * 0.42;
    let half_z = layout.panel_size.z * 0.42;
    for x in [-half_x, half_x] {
        for z in [-half_z, half_z] {
            draw_line_3d(
                layout.panel_center + vec3(x, layout.panel_size.y * 0.55, z),
                vec3(
                    layout.panel_center.x + x,
                    ceiling_y,
                    layout.panel_center.z + z,
                ),
                Color::new(accent.r, accent.g, accent.b, 0.82),
            );
        }
    }
}

fn sign_layout(display: &DisplayDef) -> SignLayout {
    let center_x = display.x + display.w * 0.5;
    let center_z = display.y + display.h * 0.5;
    match display.id.as_str() {
        "blocks_table" => SignLayout {
            panel_center: vec3(center_x, 1.96, center_z),
            panel_size: vec3(display.w * 0.64, 0.46, display.h * 0.28),
            face_z_sign: -1.0,
            hanging: true,
        },
        "dragon_bin" => SignLayout {
            panel_center: vec3(center_x, 1.26, display.y + display.h * 0.54),
            panel_size: vec3(display.w * 0.82, 0.36, 0.10),
            face_z_sign: 1.0,
            hanging: false,
        },
        "board_game_shelf" => SignLayout {
            panel_center: vec3(center_x, 1.90, center_z - 0.48),
            panel_size: vec3(display.w * 0.76, 0.38, 0.10),
            face_z_sign: -1.0,
            hanging: false,
        },
        "robot_pegboard" => SignLayout {
            panel_center: vec3(center_x, 2.32, display.y + display.h - 0.38),
            panel_size: vec3(display.w * 0.78, 0.38, 0.10),
            face_z_sign: -1.0,
            hanging: false,
        },
        "plushie_wall" => SignLayout {
            panel_center: vec3(center_x, 2.32, display.y + 0.34),
            panel_size: vec3(display.w * 0.82, 0.38, 0.10),
            face_z_sign: 1.0,
            hanging: false,
        },
        _ => SignLayout {
            panel_center: vec3(center_x, 1.70, center_z),
            panel_size: vec3(display.w * 0.70, 0.36, 0.10),
            face_z_sign: -1.0,
            hanging: false,
        },
    }
}

fn stock_label(display: &DisplayDef) -> (&'static str, &str) {
    match display.id.as_str() {
        "plushie_wall" => ("PLUSHIES", &display.theme),
        "dragon_bin" => ("DRAGONS", &display.theme),
        "robot_pegboard" => ("ROBOTS", &display.theme),
        "board_game_shelf" => ("BOARD GAMES", &display.theme),
        "blocks_table" => ("BLOCK SETS", &display.theme),
        _ => ("TOYS", &display.theme),
    }
}
