//! The end-of-shift score screen.
//!
//! Two endings arrive here and they are not the same news: the shop restored,
//! or the doors opening on work still on the floor. The panel reports the same
//! figures either way — what changes is the heading, the accent, and the line
//! telling the player what to do next.

use super::{UiContext, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::state::{GamePhase, ShiftRecord, ShiftSummary, ZoneProgress};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, format_mmss};

const PANEL: Rect = Rect {
    x: 300.0,
    y: 96.0,
    w: 680.0,
    h: 528.0,
};

pub(super) fn draw_score_screen(ctx: &UiContext<'_>) {
    let summary = ctx.session.shift_summary(ctx.data);
    let restored = ctx.session.phase == GamePhase::Finished;
    let accent = if restored {
        Color::new(0.62, 0.92, 0.68, 1.0)
    } else {
        Color::new(0.98, 0.62, 0.42, 1.0)
    };

    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.02, 0.025, 0.03, 0.72),
    );
    draw_surface(
        PANEL,
        &SurfaceStyle::new(Color::new(0.070, 0.078, 0.092, 0.98))
            .with_border(1.0, accent)
            .with_inner_border(3.0, 1.0, Color::new(0.94, 0.76, 0.42, 0.18)),
    );

    draw_heading(&summary, ctx.session.shift_mode.label(), restored, accent);
    draw_grade_badge(&summary, accent);

    let stats_y = PANEL.y + 148.0;
    draw_stat_row(
        stats_y,
        &[
            (
                "Toys shelved",
                format!("{} / {}", summary.toys_shelved, summary.toy_count),
            ),
            (
                "Repairs",
                format!("{} / {}", summary.repairs, summary.breaks_total),
            ),
            ("Wrong shelves", summary.mistakes.to_string()),
            ("Time", format_mmss(summary.elapsed_seconds)),
        ],
    );

    draw_zone_table(&summary, PANEL.y + 226.0, accent);
    draw_best_run(ctx.best_run, ctx.beat_record, accent);
    draw_footer(restored);
}

fn draw_heading(summary: &ShiftSummary, mode_label: &str, restored: bool, accent: Color) {
    let heading = if restored {
        "Store Restored"
    } else {
        "Doors Open"
    };
    draw_text_centered_in_box(
        heading,
        PANEL.x,
        PANEL.y + 26.0,
        PANEL.w,
        40.0,
        32.0,
        accent,
    );

    draw_text_centered_in_box(
        &format!(
            "{mode_label}  -  {} of {} aisles restored",
            summary.zones_restored, summary.zones_with_shelves
        ),
        PANEL.x,
        PANEL.y + 68.0,
        PANEL.w,
        26.0,
        16.0,
        dark::TEXT_DIM,
    );
}

fn draw_grade_badge(summary: &ShiftSummary, accent: Color) {
    let badge = Rect::new(PANEL.x + PANEL.w - 118.0, PANEL.y + 22.0, 92.0, 92.0);
    draw_surface(
        badge,
        &SurfaceStyle::new(Color::new(0.035, 0.040, 0.048, 0.92)).with_border(1.0, accent),
    );
    draw_text_centered_in_box(
        summary.grade(),
        badge.x,
        badge.y + 12.0,
        badge.w,
        52.0,
        46.0,
        accent,
    );
    draw_text_centered_in_box(
        &format!("{:.0}%", summary.completion() * 100.0),
        badge.x,
        badge.y + 64.0,
        badge.w,
        20.0,
        15.0,
        dark::TEXT_DIM,
    );
}

fn draw_stat_row(y: f32, cells: &[(&str, String)]) {
    let inner_x = PANEL.x + 28.0;
    let inner_w = PANEL.w - 56.0;
    let cell_w = inner_w / cells.len() as f32;

    for (index, (label, value)) in cells.iter().enumerate() {
        let x = inner_x + cell_w * index as f32;
        draw_ui_text_ex(label, x, y, TextStyle::new(13.0, dark::TEXT_DIM).params());
        draw_ui_text_ex(
            value,
            x,
            y + 26.0,
            TextStyle::new(22.0, dark::TEXT_BRIGHT).params(),
        );
    }
}

fn draw_zone_table(summary: &ShiftSummary, top: f32, accent: Color) {
    draw_ui_text_ex(
        "Aisles",
        PANEL.x + 28.0,
        top,
        TextStyle::new(15.0, dark::TEXT).params(),
    );

    for (index, (name, zone)) in summary.zones.iter().enumerate() {
        draw_zone_row(name, *zone, top + 22.0 + index as f32 * 34.0, accent);
    }
}

fn draw_zone_row(name: &str, zone: ZoneProgress, y: f32, accent: Color) {
    let x = PANEL.x + 28.0;
    let width = PANEL.w - 56.0;
    let restored = zone.is_restored();
    let label_color = if restored { accent } else { dark::TEXT };

    draw_ui_text_ex(
        name,
        x,
        y + 14.0,
        TextStyle::new(15.0, label_color).params(),
    );
    // "43 / 48" alone sends a player who shelved the whole aisle back out to
    // hunt five toys that are lying around in halves. Split the shortfall into
    // the two jobs it actually is — and show both, because an aisle can want
    // more searching *and* more mending, and reporting only one of those is how
    // the bare shortfall misled in the first place.
    let mut shortfall = Vec::new();
    if zone.still_to_find() > 0 {
        shortfall.push(format!("{} to find", zone.still_to_find()));
    }
    if zone.broken > 0 {
        shortfall.push(format!("{} to mend", zone.broken));
    }
    if !shortfall.is_empty() {
        draw_ui_text_ex(
            &shortfall.join("  -  "),
            x + 168.0,
            y + 14.0,
            TextStyle::new(13.0, Color::new(0.86, 0.72, 0.94, 1.0)).params(),
        );
    }

    draw_ui_text_ex(
        &format!("{} / {}", zone.placed, zone.capacity),
        x + width - 128.0,
        y + 14.0,
        TextStyle::new(14.0, dark::TEXT_DIM).params(),
    );

    // A bar rather than a bare percentage: an aisle one toy short of restored
    // and an aisle half done should not read the same at a glance.
    let bar = Rect::new(x + width - 72.0, y + 4.0, 72.0, 12.0);
    draw_rectangle(
        bar.x,
        bar.y,
        bar.w,
        bar.h,
        Color::new(0.10, 0.11, 0.13, 0.92),
    );
    draw_rectangle(
        bar.x,
        bar.y,
        bar.w * zone.fraction().clamp(0.0, 1.0),
        bar.h,
        if restored {
            accent
        } else {
            Color::new(0.96, 0.74, 0.38, 0.86)
        },
    );
}

/// The line that turns a grade into something to beat.
///
/// Nothing carries between shifts by design — tools are earned and lost inside
/// one run — so without this the game scores you and forgets. A record is the
/// only thread between runs, which is why it is worth the separate save slot.
fn draw_best_run(best: Option<ShiftRecord>, beat_record: bool, accent: Color) {
    let y = PANEL.y + PANEL.h - 96.0;
    let text = match best {
        Some(record) if beat_record => format!(
            "New best: {} toys in {}",
            record.toys_shelved,
            format_mmss(record.elapsed_seconds)
        ),
        Some(record) => format!(
            "Best so far: {} toys, {} wrong, {}",
            record.toys_shelved,
            record.mistakes,
            format_mmss(record.elapsed_seconds)
        ),
        None => "No record kept for this mode yet.".to_owned(),
    };
    let colour = if beat_record { accent } else { dark::TEXT_DIM };
    draw_text_centered_in_box(&text, PANEL.x, y, PANEL.w, 24.0, 16.0, colour);
}

fn draw_footer(restored: bool) {
    draw_text_centered_in_box(
        if restored {
            "The snow globe shop is ready for sunrise."
        } else {
            "Opening time caught up with you."
        },
        PANEL.x,
        PANEL.y + PANEL.h - 68.0,
        PANEL.w,
        24.0,
        16.0,
        dark::TEXT_DIM,
    );
    draw_text_centered_in_box(
        "R starts another shift  -  Esc for the menu",
        PANEL.x,
        PANEL.y + PANEL.h - 40.0,
        PANEL.w,
        24.0,
        14.0,
        dark::TEXT_DIM,
    );
}
