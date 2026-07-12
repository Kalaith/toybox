//! Procedural HUD minimap: zones, fixtures, benches, and the player.

use crate::ui::{UiContext, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;

pub(crate) fn draw_minimap(ctx: &UiContext<'_>) {
    let room_w = ctx.data.config.room_width;
    let room_h = ctx.data.config.room_height;
    let map_w = 210.0;
    let map_h = map_w * room_h / room_w.max(1.0);
    let panel = Rect::new(
        LOGICAL_WIDTH - map_w - 16.0,
        LOGICAL_HEIGHT - map_h - 16.0,
        map_w,
        map_h,
    );

    draw_surface(
        panel,
        &SurfaceStyle::new(Color::new(0.030, 0.040, 0.050, 0.86))
            .with_border(1.0, Color::new(0.55, 0.64, 0.72, 0.60)),
    );

    let scale = map_w / room_w.max(1.0);
    let to_map = |x: f32, y: f32| vec2(panel.x + x * scale, panel.y + y * scale);

    for zone in &ctx.data.layout.zones {
        let origin = to_map(zone.x, zone.y);
        draw_rectangle(
            origin.x,
            origin.y,
            zone.w * scale,
            zone.h * scale,
            Color::new(zone.accent[0], zone.accent[1], zone.accent[2], 0.14),
        );
        draw_rectangle_lines(
            origin.x,
            origin.y,
            zone.w * scale,
            zone.h * scale,
            1.0,
            Color::new(zone.accent[0], zone.accent[1], zone.accent[2], 0.42),
        );
    }

    for shelf in &ctx.data.layout.shelving {
        let origin = to_map(shelf.x, shelf.y);
        draw_rectangle(
            origin.x,
            origin.y,
            shelf.w * scale,
            shelf.h * scale,
            Color::new(0.45, 0.38, 0.28, 0.85),
        );
    }

    for display in &ctx.data.displays {
        let origin = to_map(display.x, display.y);
        let alpha = if ctx.session.is_display_complete(&display.id) {
            1.0
        } else {
            0.72
        };
        draw_rectangle(
            origin.x,
            origin.y,
            display.w * scale,
            display.h * scale,
            Color::new(
                display.accent[0],
                display.accent[1],
                display.accent[2],
                alpha,
            ),
        );
    }

    for bench in &ctx.data.layout.benches {
        let center = to_map(bench.x, bench.y);
        draw_rectangle(
            center.x - bench.w * scale * 0.5,
            center.y - bench.h * scale * 0.5,
            bench.w * scale,
            bench.h * scale,
            Color::new(0.88, 0.52, 0.24, 0.95),
        );
    }

    let player = ctx.session.player.position;
    let dot = to_map(player.x, player.y);
    let facing = vec2(ctx.session.player.yaw.cos(), ctx.session.player.yaw.sin());
    draw_line(
        dot.x,
        dot.y,
        dot.x + facing.x * 9.0,
        dot.y + facing.y * 9.0,
        2.0,
        Color::new(0.98, 0.90, 0.55, 0.95),
    );
    draw_circle(dot.x, dot.y, 3.4, Color::new(0.98, 0.92, 0.62, 1.0));
}
