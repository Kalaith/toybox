//! Procedural HUD minimap: zones, fixtures, benches, and the player.

use super::hud_chrome::{brass, draw_hud_panel, parchment, warm_card, warm_panel};
use crate::ui::{UiContext, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

pub(crate) fn draw_minimap(ctx: &UiContext<'_>) {
    let room_w = ctx.data.config.room_width;
    let room_h = ctx.data.config.room_height;
    let map_w = 210.0;
    let map_h = map_w * room_h / room_w.max(1.0);
    let map = Rect::new(
        LOGICAL_WIDTH - map_w - 16.0,
        LOGICAL_HEIGHT - map_h - 16.0,
        map_w,
        map_h,
    );
    let panel = Rect::new(map.x - 8.0, map.y - 36.0, map.w + 16.0, map.h + 44.0);

    draw_hud_panel(panel, warm_panel(0.90), brass(0.62));
    draw_ui_text_ex(
        "STORE DIRECTORY",
        panel.x + 14.0,
        panel.y + 24.0,
        TextStyle::new(12.0, parchment(0.86)).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} / {} HOME",
            ctx.session.total_placed_toys(),
            ctx.data.config.toy_count
        ),
        panel.right() - 92.0,
        panel.y + 24.0,
        TextStyle::new(10.0, parchment(0.58)).params(),
    );
    draw_surface(
        map,
        &SurfaceStyle::new(warm_card(0.94))
            .with_border(1.0, brass(0.40))
            .with_inner_border(2.0, 1.0, Color::new(1.0, 0.82, 0.44, 0.08)),
    );

    let scale = map_w / room_w.max(1.0);
    let to_map = |x: f32, y: f32| vec2(map.x + x * scale, map.y + y * scale);

    // Zones fill in as their shelves fill up, and carry a completion bar along
    // the bottom edge, so the map answers "where is there still work" without
    // counting dots.
    let progress = ctx.session.zone_progress(ctx.data);
    for (zone_index, zone) in ctx.data.layout.zones.iter().enumerate() {
        let origin = to_map(zone.x, zone.y);
        let size = vec2(zone.w * scale, zone.h * scale);
        let here = progress[zone_index];
        let fraction = here.fraction();
        draw_rectangle(
            origin.x,
            origin.y,
            size.x,
            size.y,
            Color::new(
                zone.accent[0],
                zone.accent[1],
                zone.accent[2],
                0.10 + 0.24 * fraction,
            ),
        );
        draw_rectangle_lines(
            origin.x,
            origin.y,
            size.x,
            size.y,
            1.0,
            Color::new(zone.accent[0], zone.accent[1], zone.accent[2], 0.42),
        );

        if !here.has_displays() {
            continue;
        }
        let bar = Rect::new(origin.x + 2.0, origin.y + size.y - 4.0, size.x - 4.0, 2.5);
        draw_rectangle(bar.x, bar.y, bar.w, bar.h, Color::new(0.0, 0.0, 0.0, 0.45));
        draw_rectangle(
            bar.x,
            bar.y,
            bar.w * fraction,
            bar.h,
            Color::new(zone.accent[0], zone.accent[1], zone.accent[2], 0.95),
        );
    }

    for shelf in ctx
        .data
        .layout
        .shelving
        .iter()
        .chain(ctx.data.layout.counters.iter())
    {
        let origin = to_map(shelf.x, shelf.y);
        draw_rectangle(
            origin.x,
            origin.y,
            shelf.w * scale,
            shelf.h * scale,
            Color::new(0.45, 0.38, 0.28, 0.85),
        );
    }

    // Loose toys as faint density dots; repair parts always glow warm so
    // counterpart hunts have a target.
    //
    // This used to draw only every fourth whole toy, because "thousands of
    // quads per frame sink the FPS" — true of the 4000-toy shop it was written
    // for, and meaningless at the 240 that ship. Measured either way it is a
    // wash: three runs each land inside the bench's own ±30% run-to-run
    // spread, i.e. 180 extra quads a frame is not detectable. So the map now
    // shows the floor as it actually is, rather than a quarter of it.
    for toy in &ctx.session.toys {
        if toy.is_held || toy.placed_display_id.is_some() || toy.is_consumed_repair_part() {
            continue;
        }
        let is_part = toy.is_repair_part();
        let dot = to_map(toy.position.x, toy.position.y);
        let color = if is_part {
            Color::new(0.96, 0.48, 0.20, 0.95)
        } else {
            Color::new(0.92, 0.90, 0.80, 0.45)
        };
        draw_rectangle(dot.x - 0.8, dot.y - 0.8, 1.6, 1.6, color);
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

    // The paid late-shift service should remain legible while the player is
    // looking down at the map, not only as a beacon in the 3D scene.
    if ctx.session.stockroom_spotlight_active() {
        if let Some(target) = ctx.session.stockroom_spotlight_target() {
            let target_dot = to_map(target.position.x, target.position.y);
            let gold = Color::new(1.0, 0.76, 0.18, 1.0);
            draw_circle_lines(target_dot.x, target_dot.y, 4.8, 1.5, gold);
            draw_line(
                target_dot.x - 2.4,
                target_dot.y,
                target_dot.x + 2.4,
                target_dot.y,
                1.0,
                gold,
            );
            draw_line(
                target_dot.x,
                target_dot.y - 2.4,
                target_dot.x,
                target_dot.y + 2.4,
                1.0,
                gold,
            );
        }
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
