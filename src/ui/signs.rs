//! Static in-world stock signs and projected sign labels.

use crate::data::DisplayDef;
use crate::ui::{UiContext, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::camera::Camera;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct SignLayout {
    panel_center: Vec3,
    label_center: Vec3,
    panel_size: Vec3,
    hanging: bool,
}

pub fn draw_stock_sign(display: &DisplayDef, accent: Color) {
    let layout = sign_layout(display);
    let board = Color::new(0.12, 0.09, 0.06, 1.0);
    let face = Color::new(accent.r * 0.78, accent.g * 0.78, accent.b * 0.78, 1.0);

    if layout.hanging {
        draw_hanging_sign_wires(layout, accent);
        draw_cube(
            layout.panel_center,
            layout.panel_size,
            None,
            Color::new(0.10, 0.075, 0.055, 1.0),
        );
        draw_cube(
            layout.panel_center + vec3(0.0, 0.0, -0.045),
            vec3(
                layout.panel_size.x * 0.90,
                layout.panel_size.y * 0.62,
                0.030,
            ),
            None,
            face,
        );
        draw_cube(
            layout.panel_center + vec3(0.0, 0.0, 0.045),
            vec3(
                0.030,
                layout.panel_size.y * 0.62,
                layout.panel_size.z * 0.90,
            ),
            None,
            face,
        );
    } else {
        draw_cube(layout.panel_center, layout.panel_size, None, board);
        draw_cube(
            layout.panel_center + vec3(0.0, 0.0, -0.050),
            vec3(
                layout.panel_size.x * 0.90,
                layout.panel_size.y * 0.64,
                0.030,
            ),
            None,
            face,
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

pub fn draw_stock_sign_labels(ctx: &UiContext<'_>) {
    let camera = shop_camera(ctx);
    for display in &ctx.data.displays {
        let layout = sign_layout(display);
        let Some(position) = project_world_to_logical(&camera, layout.label_center) else {
            continue;
        };
        if !position.x.is_finite() || !position.y.is_finite() {
            continue;
        }

        let distance = camera.position.distance(layout.label_center).max(0.1);
        let title_size = (24.0 / distance.sqrt()).clamp(11.0, 18.0);
        let theme_size = (title_size * 0.68).clamp(8.0, 12.0);
        let (title, theme) = stock_label(display);
        draw_centered_sign_line(title, position + vec2(0.0, -4.0), title_size, WHITE);
        draw_centered_sign_line(
            theme,
            position + vec2(0.0, theme_size + 8.0),
            theme_size,
            Color::new(0.96, 0.90, 0.72, 1.0),
        );
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
            label_center: vec3(center_x, 1.96, center_z - 0.08),
            panel_size: vec3(display.w * 0.64, 0.46, display.h * 0.28),
            hanging: true,
        },
        "dragon_bin" => SignLayout {
            panel_center: vec3(center_x, 1.26, display.y + display.h * 0.54),
            label_center: vec3(center_x, 1.26, display.y + display.h * 0.54 - 0.08),
            panel_size: vec3(display.w * 0.82, 0.36, 0.10),
            hanging: false,
        },
        "board_game_shelf" => SignLayout {
            panel_center: vec3(center_x, 1.90, center_z - 0.48),
            label_center: vec3(center_x, 1.90, center_z - 0.56),
            panel_size: vec3(display.w * 0.76, 0.38, 0.10),
            hanging: false,
        },
        "robot_pegboard" => SignLayout {
            panel_center: vec3(center_x, 2.32, display.y + display.h - 0.38),
            label_center: vec3(center_x, 2.32, display.y + display.h - 0.46),
            panel_size: vec3(display.w * 0.78, 0.38, 0.10),
            hanging: false,
        },
        "plushie_wall" => SignLayout {
            panel_center: vec3(center_x, 2.32, display.y + 0.34),
            label_center: vec3(center_x, 2.32, display.y + 0.26),
            panel_size: vec3(display.w * 0.82, 0.38, 0.10),
            hanging: false,
        },
        _ => SignLayout {
            panel_center: vec3(center_x, 1.70, center_z),
            label_center: vec3(center_x, 1.70, center_z - 0.08),
            panel_size: vec3(display.w * 0.70, 0.36, 0.10),
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

fn shop_camera(ctx: &UiContext<'_>) -> Camera3D {
    let eye = vec3(
        ctx.session.player.position.x,
        1.08,
        ctx.session.player.position.y,
    );
    let front = vec3(
        ctx.session.player.yaw.cos() * ctx.session.player.pitch.cos(),
        ctx.session.player.pitch.sin(),
        ctx.session.player.yaw.sin() * ctx.session.player.pitch.cos(),
    )
    .normalize();
    let right = front.cross(vec3(0.0, 1.0, 0.0)).normalize_or_zero();
    let up = right.cross(front).normalize_or_zero();

    Camera3D {
        position: eye,
        target: eye + front,
        up,
        fovy: 58.0,
        projection: Projection::Perspective,
        aspect: Some(screen_width() / screen_height().max(1.0)),
        ..Default::default()
    }
}

fn project_world_to_logical(camera: &Camera3D, point: Vec3) -> Option<Vec2> {
    let projected = camera.matrix() * point.extend(1.0);
    if projected.w <= 0.0 {
        return None;
    }

    let ndc = projected.truncate() / projected.w;
    if ndc.z < -1.0 || ndc.z > 1.0 {
        return None;
    }

    let screen = vec2(
        (ndc.x * 0.5 + 0.5) * screen_width(),
        (0.5 - ndc.y * 0.5) * screen_height(),
    );
    Some(vec2(
        screen.x * LOGICAL_WIDTH / screen_width().max(1.0),
        screen.y * LOGICAL_HEIGHT / screen_height().max(1.0),
    ))
}

fn draw_centered_sign_line(text: &str, center: Vec2, size: f32, color: Color) {
    let measured = measure_text(text, None, size as u16, 1.0);
    let x = center.x - measured.width * 0.5;
    let y = center.y + measured.height * 0.5;
    for offset in [
        vec2(-1.0, 0.0),
        vec2(1.0, 0.0),
        vec2(0.0, -1.0),
        vec2(0.0, 1.0),
    ] {
        draw_text_ex(
            text,
            x + offset.x,
            y + offset.y,
            TextParams {
                font_size: size as u16,
                color: Color::new(0.02, 0.025, 0.03, 0.92),
                ..Default::default()
            },
        );
    }
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font_size: size as u16,
            color,
            ..Default::default()
        },
    );
}
