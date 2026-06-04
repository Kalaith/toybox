//! Macroquad 3D renderer for the tiny toy shop.

use crate::data::{DisplayDef, ToyCategory};
use crate::state::{display_slot_position, toy_matches_display, ToyState, WorldPoint};
use crate::ui::UiContext;
use macroquad::prelude::*;

pub fn draw_shop_scene(ctx: &UiContext<'_>) {
    let eye = player_eye(ctx.session.player.position);
    let (front, _, up) = camera_basis(ctx.session.player.yaw, ctx.session.player.pitch);
    let camera = Camera3D {
        position: eye,
        target: eye + front,
        up,
        fovy: 58.0,
        projection: Projection::Perspective,
        aspect: Some(screen_width() / screen_height().max(1.0)),
        ..Default::default()
    };

    set_camera(&camera);
    draw_shop_shell(ctx);
    draw_displays(ctx);
    draw_loose_toys(ctx);
    draw_placed_toys(ctx);
    draw_placement_preview(ctx);
    draw_player_presence(ctx);
}

fn draw_shop_shell(ctx: &UiContext<'_>) {
    let width = ctx.data.config.room_width;
    let depth = ctx.data.config.room_height;
    let center = vec3(width * 0.5, 0.0, depth * 0.5);

    draw_plane(
        center,
        vec2(width, depth),
        None,
        Color::new(0.24, 0.20, 0.15, 1.0),
    );
    draw_grid_lines(width, depth);

    let wall_color = Color::new(0.18, 0.19, 0.22, 1.0);
    draw_cube(
        vec3(width * 0.5, 1.15, -0.18),
        vec3(width, 2.3, 0.34),
        None,
        wall_color,
    );
    draw_cube(
        vec3(width * 0.5, 1.15, depth + 0.18),
        vec3(width, 2.3, 0.34),
        None,
        wall_color,
    );
    draw_cube(
        vec3(-0.18, 1.15, depth * 0.5),
        vec3(0.34, 2.3, depth),
        None,
        wall_color,
    );
    draw_cube(
        vec3(width + 0.18, 1.15, depth * 0.5),
        vec3(0.34, 2.3, depth),
        None,
        wall_color,
    );

    draw_cube(
        vec3(width - 3.1, 1.34, 0.04),
        vec3(2.6, 1.15, 0.08),
        None,
        Color::new(0.40, 0.62, 0.82, 0.82),
    );
    draw_cube_wires(
        vec3(width - 3.1, 1.34, 0.04),
        vec3(2.6, 1.15, 0.08),
        Color::new(0.84, 0.92, 1.0, 0.85),
    );
}

fn draw_grid_lines(width: f32, depth: f32) {
    let line_color = Color::new(0.28, 0.25, 0.20, 0.65);
    let mut x = 0.0;
    while x <= width {
        draw_cube(
            vec3(x, 0.012, depth * 0.5),
            vec3(0.018, 0.018, depth),
            None,
            line_color,
        );
        x += 1.0;
    }

    let mut z = 0.0;
    while z <= depth {
        draw_cube(
            vec3(width * 0.5, 0.014, z),
            vec3(width, 0.018, 0.018),
            None,
            line_color,
        );
        z += 1.0;
    }
}

fn draw_displays(ctx: &UiContext<'_>) {
    for display in &ctx.data.displays {
        let accent = accent_color(display, 1.0);
        let active_match = ctx
            .session
            .active_toy()
            .is_some_and(|toy| toy_matches_display(toy, display));
        let should_glow = active_match && ctx.session.has_upgrade("tag_lantern");
        let is_complete = ctx.session.is_display_complete(&display.id);

        draw_display_mat(display, accent, should_glow);
        match display.id.as_str() {
            "plushie_wall" => draw_wall_display(display, accent),
            "dragon_bin" => draw_dragon_bin(display, accent),
            "robot_pegboard" => draw_robot_pegboard(display, accent),
            "board_game_shelf" => draw_board_shelf(display, accent),
            "blocks_table" => draw_blocks_table(display, accent),
            _ => draw_generic_display(display, accent),
        }
        draw_slot_markers(ctx, display, accent);

        if is_complete {
            draw_completion_lights(display, accent);
        }
    }
}

fn draw_slot_markers(ctx: &UiContext<'_>, display: &DisplayDef, accent: Color) {
    for slot_number in 1..=display.capacity {
        let is_filled = ctx.session.is_display_slot_filled(&display.id, slot_number);
        let slot = display_slot_position(display, slot_number - 1, ctx.data.config.room_width);
        let base_height = placed_height_for_slot(display, slot_number);
        let marker = world_point(slot, base_height - 0.18);
        let marker_color = if is_filled {
            Color::new(accent.r, accent.g, accent.b, 0.34)
        } else {
            Color::new(accent.r * 0.45, accent.g * 0.45, accent.b * 0.45, 0.68)
        };
        draw_cube(marker, vec3(0.30, 0.035, 0.30), None, marker_color);
        if !is_filled {
            draw_cube_wires(
                marker,
                vec3(0.32, 0.05, 0.32),
                Color::new(accent.r, accent.g, accent.b, 0.62),
            );
        }
    }
}

fn draw_display_mat(display: &DisplayDef, accent: Color, should_glow: bool) {
    let center = display_center(display, 0.035);
    let color = if should_glow {
        Color::new(accent.r, accent.g, accent.b, 0.92)
    } else {
        Color::new(accent.r * 0.34, accent.g * 0.34, accent.b * 0.34, 0.88)
    };
    let extra = if should_glow { 0.28 } else { 0.0 };
    draw_cube(
        center,
        vec3(display.w + extra, 0.07, display.h + extra),
        None,
        color,
    );
    draw_cube_wires(center, vec3(display.w, 0.08, display.h), accent);
}

fn draw_wall_display(display: &DisplayDef, accent: Color) {
    let x = display.x + display.w * 0.5;
    let z = display.y + 0.10;
    draw_cube(
        vec3(x, 1.15, z),
        vec3(display.w, 2.10, 0.22),
        None,
        Color::new(0.20, 0.18, 0.17, 1.0),
    );
    draw_cube(
        vec3(x, 0.64, z + 0.20),
        vec3(display.w * 0.92, 0.12, 0.40),
        None,
        accent,
    );
    draw_cube(
        vec3(x, 1.18, z + 0.20),
        vec3(display.w * 0.92, 0.12, 0.40),
        None,
        accent,
    );
    draw_cube(
        vec3(x, 1.72, z + 0.20),
        vec3(display.w * 0.92, 0.12, 0.40),
        None,
        accent,
    );
}

fn draw_dragon_bin(display: &DisplayDef, accent: Color) {
    let center = display_center(display, 0.32);
    draw_cube(
        center,
        vec3(display.w, 0.62, display.h),
        None,
        Color::new(0.18, 0.12, 0.22, 1.0),
    );
    draw_cube_wires(center, vec3(display.w, 0.66, display.h), accent);
    draw_cube(
        center + vec3(0.0, 0.42, -display.h * 0.42),
        vec3(display.w, 0.18, 0.16),
        None,
        accent,
    );
    draw_cube(
        center + vec3(0.0, 0.42, display.h * 0.42),
        vec3(display.w, 0.18, 0.16),
        None,
        accent,
    );
}

fn draw_robot_pegboard(display: &DisplayDef, accent: Color) {
    let x = display.x + display.w * 0.5;
    let z = display.y + display.h - 0.1;
    draw_cube(
        vec3(x, 1.18, z),
        vec3(display.w, 2.0, 0.22),
        None,
        Color::new(0.13, 0.16, 0.19, 1.0),
    );
    for row in 0..3 {
        for column in 0..4 {
            let peg_x = display.x + 0.55 + column as f32 * 0.9;
            let peg_y = 0.66 + row as f32 * 0.46;
            draw_cube(
                vec3(peg_x, peg_y, z - 0.22),
                vec3(0.12, 0.08, 0.34),
                None,
                accent,
            );
        }
    }
}

fn draw_board_shelf(display: &DisplayDef, accent: Color) {
    let x = display.x + display.w * 0.5;
    let z = display.y + display.h * 0.5;
    draw_cube(
        vec3(x, 0.90, z),
        vec3(display.w, 1.70, display.h * 0.34),
        None,
        Color::new(0.22, 0.14, 0.09, 1.0),
    );
    for shelf in 0..3 {
        draw_cube(
            vec3(x, 0.35 + shelf as f32 * 0.52, z - 0.42),
            vec3(display.w * 0.92, 0.10, 0.34),
            None,
            accent,
        );
    }
}

fn draw_blocks_table(display: &DisplayDef, accent: Color) {
    let center = display_center(display, 0.66);
    draw_cube(
        center,
        vec3(display.w, 0.28, display.h),
        None,
        Color::new(0.30, 0.21, 0.12, 1.0),
    );
    draw_cube(
        center + vec3(-display.w * 0.38, -0.42, -display.h * 0.34),
        vec3(0.18, 0.82, 0.18),
        None,
        accent,
    );
    draw_cube(
        center + vec3(display.w * 0.38, -0.42, -display.h * 0.34),
        vec3(0.18, 0.82, 0.18),
        None,
        accent,
    );
    draw_cube(
        center + vec3(-display.w * 0.38, -0.42, display.h * 0.34),
        vec3(0.18, 0.82, 0.18),
        None,
        accent,
    );
    draw_cube(
        center + vec3(display.w * 0.38, -0.42, display.h * 0.34),
        vec3(0.18, 0.82, 0.18),
        None,
        accent,
    );
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

fn draw_loose_toys(ctx: &UiContext<'_>) {
    for (index, toy) in ctx.session.toys.iter().enumerate() {
        if !toy.is_held && toy.placed_display_id.is_none() {
            let layer = (index % 7) as f32;
            let height = 0.20 + layer * 0.035;
            let scale = 0.88 + ((index * 13) % 9) as f32 * 0.025;
            draw_toy_3d(
                toy,
                world_point(toy.position, height),
                toy_color(toy),
                scale,
            );
        }
    }
}

fn draw_placed_toys(ctx: &UiContext<'_>) {
    for display in &ctx.data.displays {
        for toy in ctx.session.placed_toys_for_display(&display.id) {
            let height = placed_height(display, toy);
            let center = world_point(toy.position, height);
            draw_toy_3d(toy, center, toy_color(toy), 0.92);
            if toy.wrong_marker_seconds > 0.0 {
                draw_wrong_placement_marker(center);
            }
        }
    }
}

fn draw_wrong_placement_marker(center: Vec3) {
    let color = Color::new(0.98, 0.18, 0.14, 0.92);
    draw_cube_wires(center + vec3(0.0, 0.04, 0.0), vec3(0.72, 0.72, 0.72), color);
    draw_line_3d(
        center + vec3(-0.42, 0.48, -0.42),
        center + vec3(0.42, 0.48, 0.42),
        color,
    );
    draw_line_3d(
        center + vec3(-0.42, 0.48, 0.42),
        center + vec3(0.42, 0.48, -0.42),
        color,
    );
}

fn draw_placement_preview(ctx: &UiContext<'_>) {
    let Some(toy) = ctx.session.active_toy() else {
        return;
    };
    let Some(display) = nearest_display_to_player(ctx) else {
        return;
    };

    let accent = Color::new(0.98, 0.80, 0.30, 1.0);
    let slot = display_slot_position(
        display,
        toy.slot_number.saturating_sub(1),
        ctx.data.config.room_width,
    );
    let height = placed_height(display, toy);
    let center = world_point(slot, height);

    draw_cube(
        center + vec3(0.0, -0.30, 0.0),
        vec3(0.62, 0.045, 0.62),
        None,
        Color::new(accent.r, accent.g, accent.b, 0.42),
    );
    draw_cube_wires(center, vec3(0.70, 0.70, 0.70), accent);
    draw_line_3d(
        center + vec3(-0.42, 0.0, -0.42),
        center + vec3(0.42, 0.0, 0.42),
        accent,
    );
    draw_line_3d(
        center + vec3(-0.42, 0.0, 0.42),
        center + vec3(0.42, 0.0, -0.42),
        accent,
    );
    draw_sphere(center + vec3(0.0, 0.52, 0.0), 0.10, None, accent);
}

fn nearest_display_to_player<'a>(ctx: &'a UiContext<'a>) -> Option<&'a DisplayDef> {
    let player = ctx.session.player.position.to_vec2();
    let max_distance_sq = ctx.data.config.interaction_radius * ctx.data.config.interaction_radius;

    ctx.data
        .displays
        .iter()
        .filter_map(|display| {
            let nearest_point = vec2(
                player.x.clamp(display.x, display.x + display.w),
                player.y.clamp(display.y, display.y + display.h),
            );
            let distance_sq = nearest_point.distance_squared(player);
            (distance_sq <= max_distance_sq).then_some((display, distance_sq))
        })
        .min_by(|(_, left), (_, right)| left.total_cmp(right))
        .map(|(display, _)| display)
}

fn draw_player_presence(ctx: &UiContext<'_>) {
    let pos = world_point(ctx.session.player.position, 0.0);
    draw_cube_wires(
        pos + vec3(0.0, 0.05, 0.0),
        vec3(
            ctx.data.config.interaction_radius * 2.0,
            0.05,
            ctx.data.config.interaction_radius * 2.0,
        ),
        Color::new(0.95, 0.78, 0.30, 0.25),
    );

    if let Some(toy) = ctx.session.active_toy() {
        let eye = player_eye(ctx.session.player.position);
        let (front, right, up) = camera_basis(ctx.session.player.yaw, ctx.session.player.pitch);
        draw_toy_3d(
            toy,
            eye + front * 0.82 + right * 0.30 - up * 0.22,
            brighten(toy_color(toy), 0.08),
            0.58,
        );
    }
}

fn draw_toy_3d(toy: &ToyState, center: Vec3, color: Color, scale: f32) {
    match toy.category {
        ToyCategory::Plushies => draw_plush(center, color, scale),
        ToyCategory::TinyDragons => draw_dragon(center, color, scale),
        ToyCategory::ActionFigures => draw_robot(center, color, scale),
        ToyCategory::BoardGames => draw_board_game(center, color, scale),
        ToyCategory::BuildingBlocks => draw_blocks(center, color, scale),
    }
    draw_cube(
        center + vec3(0.23 * scale, 0.08 * scale, -0.18 * scale),
        vec3(0.16, 0.05, 0.10) * scale,
        None,
        Color::new(0.95, 0.90, 0.72, 1.0),
    );
}

fn draw_plush(center: Vec3, color: Color, scale: f32) {
    draw_sphere(center, 0.28 * scale, None, color);
    draw_sphere(
        center + vec3(-0.20, 0.22, 0.0) * scale,
        0.13 * scale,
        None,
        color,
    );
    draw_sphere(
        center + vec3(0.20, 0.22, 0.0) * scale,
        0.13 * scale,
        None,
        color,
    );
    draw_sphere(
        center + vec3(-0.08, 0.05, -0.25) * scale,
        0.035 * scale,
        None,
        BLACK,
    );
    draw_sphere(
        center + vec3(0.08, 0.05, -0.25) * scale,
        0.035 * scale,
        None,
        BLACK,
    );
}

fn draw_dragon(center: Vec3, color: Color, scale: f32) {
    draw_sphere(center, 0.25 * scale, None, color);
    draw_sphere(
        center + vec3(0.0, 0.18, -0.30) * scale,
        0.15 * scale,
        None,
        brighten(color, 0.08),
    );
    draw_cube(
        center + vec3(-0.26, 0.06, 0.02) * scale,
        vec3(0.10, 0.26, 0.38) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(0.26, 0.06, 0.02) * scale,
        vec3(0.10, 0.26, 0.38) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(0.0, 0.38, -0.34) * scale,
        vec3(0.10, 0.16, 0.08) * scale,
        None,
        Color::new(0.96, 0.88, 0.58, 1.0),
    );
}

fn draw_robot(center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(0.0, 0.08, 0.0) * scale,
        vec3(0.34, 0.36, 0.30) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.0, 0.39, -0.01) * scale,
        vec3(0.28, 0.24, 0.26) * scale,
        None,
        brighten(color, 0.10),
    );
    draw_cube(
        center + vec3(-0.26, 0.08, 0.0) * scale,
        vec3(0.10, 0.30, 0.12) * scale,
        None,
        darken(color, 0.12),
    );
    draw_cube(
        center + vec3(0.26, 0.08, 0.0) * scale,
        vec3(0.10, 0.30, 0.12) * scale,
        None,
        darken(color, 0.12),
    );
}

fn draw_board_game(center: Vec3, color: Color, scale: f32) {
    draw_cube(center, vec3(0.52, 0.12, 0.38) * scale, None, color);
    draw_cube(
        center + vec3(0.0, 0.08, -0.01) * scale,
        vec3(0.42, 0.035, 0.28) * scale,
        None,
        brighten(color, 0.12),
    );
}

fn draw_blocks(center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(-0.13, 0.0, -0.06) * scale,
        vec3(0.22, 0.22, 0.22) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.12, 0.0, 0.02) * scale,
        vec3(0.22, 0.22, 0.22) * scale,
        None,
        brighten(color, 0.10),
    );
    draw_cube(
        center + vec3(0.0, 0.22, 0.0) * scale,
        vec3(0.22, 0.22, 0.22) * scale,
        None,
        darken(color, 0.08),
    );
}

fn placed_height(display: &DisplayDef, toy: &ToyState) -> f32 {
    placed_height_for_slot(display, toy.slot_number)
}

fn placed_height_for_slot(display: &DisplayDef, slot_number: usize) -> f32 {
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

fn world_point(point: WorldPoint, height: f32) -> Vec3 {
    vec3(point.x, height, point.y)
}

fn player_eye(point: WorldPoint) -> Vec3 {
    world_point(point, 1.08)
}

fn camera_basis(yaw: f32, pitch: f32) -> (Vec3, Vec3, Vec3) {
    let world_up = vec3(0.0, 1.0, 0.0);
    let front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let right = front.cross(world_up).normalize_or_zero();
    let up = right.cross(front).normalize_or_zero();
    (front, right, up)
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

fn toy_color(toy: &ToyState) -> Color {
    let base = match toy.category {
        ToyCategory::Plushies => Color::new(0.34, 0.78, 0.50, 1.0),
        ToyCategory::TinyDragons => Color::new(0.70, 0.42, 0.94, 1.0),
        ToyCategory::ActionFigures => Color::new(0.52, 0.74, 0.90, 1.0),
        ToyCategory::BoardGames => Color::new(0.92, 0.62, 0.30, 1.0),
        ToyCategory::BuildingBlocks => Color::new(0.94, 0.80, 0.26, 1.0),
    };
    let offset = toy.color_index as f32 * 0.025 - 0.045;
    Color::new(
        (base.r + offset).clamp(0.08, 0.98),
        (base.g + offset).clamp(0.08, 0.98),
        (base.b + offset).clamp(0.08, 0.98),
        1.0,
    )
}

fn brighten(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r + amount).clamp(0.0, 1.0),
        (color.g + amount).clamp(0.0, 1.0),
        (color.b + amount).clamp(0.0, 1.0),
        color.a,
    )
}

fn darken(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r - amount).clamp(0.0, 1.0),
        (color.g - amount).clamp(0.0, 1.0),
        (color.b - amount).clamp(0.0, 1.0),
        color.a,
    )
}
