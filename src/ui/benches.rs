//! Repair benches: the bench itself, its worktop dressing, and the status
//! beacon that reports what each bench is holding from across the room.

use crate::data::BenchDef;
use crate::state::{BenchStage, BenchStatus};
use crate::ui::wood::draw_wood_cube;
use crate::ui::UiContext;
use macroquad::prelude::*;

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
        draw_bench_status_beacon(center, bench, ctx.session.bench_status(bench));

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
fn draw_bench_worktop(center: Vec3, bench: &BenchDef) {
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

/// Status beacon on the bench's back-right corner: a post, a lamp head whose
/// colour encodes the bench stage, and one pip per slot showing how many parts
/// are waiting. Sits above the worktop clutter so it stays readable at range.
fn draw_bench_status_beacon(center: Vec3, bench: &BenchDef, status: BenchStatus) {
    let base = center + vec3(bench.w * 0.5 - 0.10, 0.175, bench.h * 0.32);
    let metal = Color::new(0.58, 0.61, 0.63, 1.0);
    let (lamp, pulses) = beacon_lamp(status.stage);

    draw_cube(
        base + vec3(0.0, 0.015, 0.0),
        vec3(0.14, 0.03, 0.14),
        None,
        Color::new(0.20, 0.23, 0.24, 1.0),
    );
    draw_cube(
        base + vec3(0.0, 0.40, 0.0),
        vec3(0.05, 0.75, 0.05),
        None,
        metal,
    );

    // Lamp head: dark housing, the status lamp itself, and a soft halo that
    // breathes for the two stages that want the player to walk over.
    let head = base + vec3(0.0, 0.86, 0.0);
    draw_cube(
        head,
        vec3(0.24, 0.20, 0.24),
        None,
        Color::new(0.09, 0.10, 0.12, 1.0),
    );
    draw_cube(head, vec3(0.27, 0.10, 0.27), None, lamp);
    let breath = if pulses {
        0.55 + 0.45 * ((get_time() as f32 * 2.2).sin() * 0.5 + 0.5)
    } else {
        0.30
    };
    draw_cube(
        head,
        vec3(0.34, 0.34, 0.34),
        None,
        Color::new(lamp.r, lamp.g, lamp.b, 0.12 * breath),
    );

    // One pip per bench slot on the front face, lit left to right.
    let pip_span = 0.115;
    let first = -(status.capacity.saturating_sub(1) as f32) * pip_span * 0.5;
    for slot in 0..status.capacity {
        let lit = slot < status.filled;
        draw_cube(
            head + vec3(first + slot as f32 * pip_span, -0.135, -0.125),
            vec3(0.075, 0.075, 0.025),
            None,
            if lit {
                lamp
            } else {
                Color::new(0.24, 0.25, 0.27, 1.0)
            },
        );
    }
}

/// Beacon colour per stage, plus whether it should pulse for attention.
fn beacon_lamp(stage: BenchStage) -> (Color, bool) {
    match stage {
        BenchStage::Empty => (Color::new(0.30, 0.33, 0.36, 1.0), false),
        BenchStage::AwaitingMatch => (Color::new(0.98, 0.72, 0.26, 1.0), true),
        BenchStage::Mismatched => (Color::new(0.90, 0.28, 0.24, 1.0), false),
        BenchStage::Ready => (Color::new(0.34, 0.95, 0.60, 1.0), true),
    }
}
