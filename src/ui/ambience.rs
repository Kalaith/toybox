//! Per-zone night-shift atmosphere: what makes one aisle feel unlike the next
//! when the shop lights are down. All procedural — no textures.

use crate::data::{GameData, ZoneDef};
use macroquad::prelude::*;

/// Blend a lamp's warm base toward a zone's accent.
///
/// Every zone used to be lit by the same bulb, so the departments only read
/// apart from their signs and rugs. A partial blend keeps the shop a warm
/// after-hours space rather than turning each aisle into a coloured stage.
pub(crate) fn zone_tint(zone: &ZoneDef, base: Color, strength: f32) -> Color {
    let blend = strength.clamp(0.0, 1.0);
    Color::new(
        base.r + (zone.accent[0] - base.r) * blend,
        base.g + (zone.accent[1] - base.g) * blend,
        base.b + (zone.accent[2] - base.b) * blend,
        base.a,
    )
}

/// Night sky behind the shopfront glass: stars, a low moon, and a wash of city
/// light along the bottom of the pane.
///
/// Painted onto the pane from the shop side rather than placed behind it.
///
/// Two depth traps here, both of which rendered a flat dark rectangle before
/// they were found. The front wall is a solid slab whose inner face sits at
/// `-(thickness / 2 + 0.01)`, so the window's own glass depth is *inside* that
/// slab and never survives the depth test. And every layer needs real
/// separation: stars first sat 0.003 in front of the sky quad, which shares a
/// front face with them at that spacing, so they z-fought and vanished.
pub(crate) fn draw_window_night_sky(data: &GameData) {
    let window = &data.layout.window;
    let pane_w = window.width - 0.30;
    let pane_h = window.height - 0.28;
    let left = window.x - pane_w * 0.5;
    let bottom = window.center_y - pane_h * 0.5;
    let sky_z = 0.055;

    draw_cube(
        vec3(window.x, window.center_y, sky_z),
        vec3(pane_w, pane_h, 0.01),
        None,
        Color::new(0.04, 0.06, 0.13, 1.0),
    );

    // Deterministic star field — same coprime-stride hashing as the skylights,
    // so stars never crawl between frames.
    for star in 0..34_usize {
        let x = left + 0.06 + ((star * 53) % 97) as f32 / 97.0 * (pane_w - 0.12);
        let y = bottom + 0.06 + ((star * 37) % 89) as f32 / 89.0 * (pane_h - 0.12);
        let size = if star % 7 == 0 { 0.035 } else { 0.020 };
        let glow = if star % 5 == 0 { 0.95 } else { 0.72 };
        draw_cube(
            vec3(x, y, sky_z + 0.020),
            vec3(size, size, 0.004),
            None,
            Color::new(0.92, 0.95, 1.0, glow),
        );
    }

    // A squared-off moon with a soft halo. A crescent was tried and abandoned:
    // subtracting one cube from another gives an L-shaped bite that reads as a
    // broken tile, not a moon. A plain disc suits a shop built out of cubes.
    let moon = vec3(left + pane_w * 0.74, bottom + pane_h * 0.72, sky_z + 0.026);
    draw_cube(
        moon,
        vec3(0.30, 0.30, 0.004),
        None,
        Color::new(0.96, 0.72, 0.34, 0.10),
    );
    draw_cube(
        moon + vec3(0.0, 0.0, 0.004),
        vec3(0.17, 0.17, 0.004),
        None,
        Color::new(0.97, 0.95, 0.84, 1.0),
    );

    // Streetlight haze along the sill: thin and faint, so it reads as glow
    // rather than as a solid ledge across the bottom of the glass.
    draw_cube(
        vec3(window.x, bottom + pane_h * 0.055, sky_z + 0.014),
        vec3(pane_w, pane_h * 0.11, 0.004),
        None,
        Color::new(0.96, 0.72, 0.38, 0.07),
    );
}

/// Till, receipt spike, bag stack and a forgotten mug on the checkout counter.
/// Everything sits low and squared-off so nothing reads as a toy to pick up —
/// the same rule the backroom props follow.
pub(crate) fn draw_checkout_clutter(data: &GameData) {
    let Some(counter) = data.layout.counters.first() else {
        return;
    };
    let top = 0.94;
    let center_x = counter.x + counter.w * 0.5;
    let center_z = counter.y + counter.h * 0.5;
    let metal = Color::new(0.58, 0.60, 0.62, 1.0);

    // Till: body, angled keypad, and a small green readout facing the aisle.
    let till = vec3(center_x - counter.w * 0.28, top + 0.11, center_z);
    draw_cube(
        till,
        vec3(0.52, 0.22, 0.38),
        None,
        Color::new(0.22, 0.24, 0.27, 1.0),
    );
    draw_cube(
        till + vec3(0.0, 0.13, -0.06),
        vec3(0.40, 0.04, 0.22),
        None,
        Color::new(0.32, 0.34, 0.38, 1.0),
    );
    draw_cube(
        till + vec3(0.0, 0.20, 0.10),
        vec3(0.30, 0.12, 0.03),
        None,
        Color::new(0.10, 0.24, 0.18, 1.0),
    );
    draw_cube(
        till + vec3(0.0, 0.20, 0.085),
        vec3(0.22, 0.05, 0.01),
        None,
        Color::new(0.44, 0.94, 0.62, 0.92),
    );

    // Receipt spike with a ragged stack of slips already on it.
    let spike = vec3(center_x + counter.w * 0.10, top, center_z + 0.16);
    draw_cube(
        spike + vec3(0.0, 0.01, 0.0),
        vec3(0.14, 0.02, 0.14),
        None,
        metal,
    );
    draw_cube(
        spike + vec3(0.0, 0.13, 0.0),
        vec3(0.015, 0.26, 0.015),
        None,
        metal,
    );
    for slip in 0..4_usize {
        let lean = (slip as f32 - 1.5) * 0.012;
        draw_cube(
            spike + vec3(lean, 0.045 + slip as f32 * 0.016, lean * 0.6),
            vec3(0.16, 0.012, 0.13),
            None,
            Color::new(0.94, 0.92, 0.84, 1.0),
        );
    }

    // Folded paper bags, and a mug someone left behind at close.
    let bags = vec3(center_x + counter.w * 0.30, top + 0.04, center_z - 0.10);
    for layer in 0..3_usize {
        draw_cube(
            bags + vec3(0.0, layer as f32 * 0.026, layer as f32 * 0.012),
            vec3(0.34, 0.022, 0.26),
            None,
            Color::new(0.78, 0.64, 0.44, 1.0),
        );
    }
    let mug = vec3(center_x + counter.w * 0.30, top + 0.05, center_z + 0.22);
    draw_cube(
        mug,
        vec3(0.10, 0.10, 0.10),
        None,
        Color::new(0.86, 0.88, 0.90, 1.0),
    );
    draw_cube(
        mug + vec3(0.07, 0.0, 0.0),
        vec3(0.035, 0.05, 0.02),
        None,
        Color::new(0.86, 0.88, 0.90, 1.0),
    );
    draw_cube(
        mug + vec3(0.0, 0.045, 0.0),
        vec3(0.075, 0.008, 0.075),
        None,
        Color::new(0.36, 0.24, 0.18, 1.0),
    );
}
