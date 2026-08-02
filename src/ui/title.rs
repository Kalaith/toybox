use super::{logical_mouse_position, UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::state::{BestRuns, ShiftMode};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;

pub(crate) fn draw_title_screen(
    title_texture: Option<&Texture2D>,
    continue_enabled: bool,
    best_runs: &BestRuns,
) -> Vec<UiAction> {
    draw_title_background(title_texture);
    draw_title_scrim();

    let mut actions = Vec::new();
    let mouse = logical_mouse_position();
    let button_w = 148.0;
    let button_h = 38.0;
    let button_gap = 14.0;
    let buttons: [(&str, UiAction, bool, ButtonTone); 5] = [
        (
            "Closing Shift",
            UiAction::NewGame,
            true,
            ButtonTone::Primary,
        ),
        (
            "Relaxed Run",
            UiAction::NewRelaxedGame,
            true,
            ButtonTone::Muted,
        ),
        (
            "Continue",
            UiAction::Continue,
            continue_enabled,
            ButtonTone::Positive,
        ),
        ("Settings", UiAction::Settings, true, ButtonTone::Muted),
        ("Quit Game", UiAction::QuitGame, true, ButtonTone::Danger),
    ];
    let count = buttons.len() as f32;
    let total_w = button_w * count + button_gap * (count - 1.0);
    let x = (LOGICAL_WIDTH - total_w) * 0.5;
    let y = 614.0;

    for (index, (label, action, enabled, tone)) in buttons.into_iter().enumerate() {
        let slot_x = x + (button_w + button_gap) * index as f32;
        if title_button(
            Rect::new(slot_x, y, button_w, button_h),
            label,
            enabled,
            tone,
            mouse,
        ) {
            actions.push(action);
        }
    }

    // The two ways to start differ only in whether the clock can end the run,
    // which no button label can carry on its own.
    draw_title_caption(
        "Closing Shift runs against opening time. Relaxed Run never ends the shift.",
        y + button_h + 22.0,
    );
    draw_best_runs(best_runs, y - 18.0);

    actions
}

/// The player's best run for each mode, above the buttons that pick one.
///
/// The score screen already reports a record once a shift ends, but by then the
/// choice has been made. Sitting at the title deciding between a timed run and
/// a relaxed one, what you are chasing is the thing worth knowing — and with
/// nothing carrying between shifts, the record is the only reason to pick the
/// clock at all.
fn draw_best_runs(best: &BestRuns, y: f32) {
    let parts: Vec<String> = [ShiftMode::Timed, ShiftMode::Relaxed]
        .into_iter()
        .filter_map(|mode| {
            best.best_for(mode)
                .map(|record| format!("{}: {} toys", mode.label(), record.toys_shelved))
        })
        .collect();

    if parts.is_empty() {
        return;
    }
    draw_title_caption(&format!("Best - {}", parts.join("   ")), y);
}

/// A centred line of small text over the title art, on a plate.
///
/// The art behind it is a lit shop full of bright toys, so unbacked 15px text
/// lands on whatever colour happens to be there and is barely readable over the
/// pale ones. The plate is the same idiom as the HUD panels: dark, mostly
/// transparent, sized to the text rather than the screen.
fn draw_title_caption(text: &str, y: f32) {
    let size = 15.0_f32;
    let width = measure_ui_text(text, None, size as u16, 1.0).width;
    let x = (LOGICAL_WIDTH - width) * 0.5;

    draw_rectangle(
        x - 14.0,
        y - size - 4.0,
        width + 28.0,
        size + 12.0,
        Color::new(0.02, 0.025, 0.03, 0.62),
    );
    let style = TextStyle::new(size, Color::new(0.90, 0.91, 0.94, 0.95));
    draw_ui_text_ex(text, x, y, style.params());
}

pub(crate) fn draw_settings_screen(
    title_texture: Option<&Texture2D>,
    fullscreen_enabled: bool,
    fov_degrees: f32,
    from_game: bool,
) -> Vec<UiAction> {
    draw_title_background(title_texture);
    draw_title_scrim();

    let mut actions = Vec::new();
    let mouse = logical_mouse_position();
    let button_w = 184.0;
    let button_h = 38.0;
    let button_gap = 14.0;

    draw_text_centered_in_box(
        if from_game { "Paused" } else { "Settings" },
        0.0,
        498.0,
        LOGICAL_WIDTH,
        38.0,
        24.0,
        title_parchment(),
    );

    // Row 1: fullscreen toggle and field-of-view stepper.
    let row1_y = 556.0;
    let fov_group_w = 44.0 + 6.0 + 96.0 + 6.0 + 44.0;
    let row1_w = button_w + button_gap + fov_group_w;
    let row1_x = (LOGICAL_WIDTH - row1_w) * 0.5;

    let fullscreen_label = if fullscreen_enabled {
        "Fullscreen: On"
    } else {
        "Fullscreen: Off"
    };
    if title_button(
        Rect::new(row1_x, row1_y, button_w, button_h),
        fullscreen_label,
        true,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::ToggleFullscreen);
    }

    let fov_x = row1_x + button_w + button_gap;
    if title_button(
        Rect::new(fov_x, row1_y, 44.0, button_h),
        "-",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::FovDecrease);
    }
    draw_plaque(
        Rect::new(fov_x + 50.0, row1_y, 96.0, button_h),
        &title_plaque_style(),
        &title_button_palette(ButtonTone::Muted),
        PlaqueState::idle(true),
    );
    draw_text_centered_in_box(
        &format!("FOV {}", fov_degrees.round() as i32),
        fov_x + 50.0,
        row1_y - 1.0,
        96.0,
        button_h,
        15.0,
        title_parchment(),
    );
    if title_button(
        Rect::new(fov_x + 152.0, row1_y, 44.0, button_h),
        "+",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::FovIncrease);
    }

    // Row 2: return where you came from, plus quit-to-title while paused.
    let row2_y = 614.0;
    let row2_w = if from_game {
        button_w * 2.0 + button_gap
    } else {
        button_w
    };
    let row2_x = (LOGICAL_WIDTH - row2_w) * 0.5;

    if title_button(
        Rect::new(row2_x, row2_y, button_w, button_h),
        if from_game { "Resume" } else { "Back" },
        true,
        if from_game {
            ButtonTone::Positive
        } else {
            ButtonTone::Muted
        },
        mouse,
    ) {
        actions.push(UiAction::CloseSettings);
    }

    // Named for what it does rather than what it used to: leaving writes the
    // shift to the save slot, so the title's Continue picks it back up. A
    // Danger tone said the opposite — that the run was about to be thrown
    // away, which is exactly what used to happen.
    if from_game
        && title_button(
            Rect::new(row2_x + button_w + button_gap, row2_y, button_w, button_h),
            "Save & Quit",
            true,
            ButtonTone::Muted,
            mouse,
        )
    {
        actions.push(UiAction::BackToTitle);
    }

    actions
}

fn draw_title_background(texture: Option<&Texture2D>) {
    if let Some(texture) = texture {
        let texture_size = texture.size();
        let scale = (LOGICAL_WIDTH / texture_size.x).max(LOGICAL_HEIGHT / texture_size.y);
        let dest_size = texture_size * scale;
        let x = (LOGICAL_WIDTH - dest_size.x) * 0.5;
        let y = (LOGICAL_HEIGHT - dest_size.y) * 0.5;

        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size),
                ..Default::default()
            },
        );
    } else {
        clear_background(Color::new(0.05, 0.045, 0.055, 1.0));
        draw_text_centered_in_box(
            "Toybox After Hours",
            0.0,
            230.0,
            LOGICAL_WIDTH,
            70.0,
            54.0,
            dark::TEXT_BRIGHT,
        );
    }
}

fn draw_title_scrim() {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.02, 0.014, 0.010, 0.16),
    );
}

fn title_button(rect: Rect, text: &str, enabled: bool, tone: ButtonTone, mouse: Vec2) -> bool {
    plaque_button(
        rect,
        text,
        &title_plaque_style(),
        &title_button_palette(tone),
        enabled,
        mouse,
    )
}

/// Toolkit plaque defaults were extracted from these exact title buttons;
/// only the fixed label size and disabled-border alpha differ.
fn title_plaque_style() -> PlaqueStyle {
    PlaqueStyle {
        font_size: Some(15.0),
        disabled_border: Color::new(0.30, 0.28, 0.24, 0.78),
        ..PlaqueStyle::default()
    }
}

fn title_button_palette(tone: ButtonTone) -> PlaquePalette {
    match tone {
        ButtonTone::Primary => PlaquePalette {
            normal: Color::new(0.12, 0.20, 0.31, 0.92),
            hovered: Color::new(0.18, 0.29, 0.44, 0.96),
            pressed: Color::new(0.08, 0.14, 0.23, 0.98),
            disabled: Color::new(0.08, 0.10, 0.13, 0.72),
            border: Color::new(0.58, 0.64, 0.74, 0.86),
            text: title_parchment(),
        },
        ButtonTone::Positive => PlaquePalette {
            normal: Color::new(0.11, 0.28, 0.17, 0.92),
            hovered: Color::new(0.17, 0.38, 0.23, 0.96),
            pressed: Color::new(0.07, 0.20, 0.12, 0.98),
            disabled: Color::new(0.07, 0.10, 0.08, 0.68),
            border: Color::new(0.55, 0.72, 0.42, 0.84),
            text: title_parchment(),
        },
        ButtonTone::Danger => PlaquePalette {
            normal: Color::new(0.31, 0.12, 0.10, 0.92),
            hovered: Color::new(0.45, 0.17, 0.14, 0.96),
            pressed: Color::new(0.22, 0.08, 0.07, 0.98),
            disabled: Color::new(0.14, 0.08, 0.08, 0.70),
            border: Color::new(0.78, 0.42, 0.34, 0.80),
            text: title_parchment(),
        },
        ButtonTone::Muted | ButtonTone::Secondary | ButtonTone::Warning => PlaquePalette {
            normal: Color::new(0.080, 0.083, 0.090, 0.90),
            hovered: Color::new(0.13, 0.135, 0.145, 0.96),
            pressed: Color::new(0.055, 0.058, 0.064, 0.98),
            disabled: Color::new(0.055, 0.055, 0.060, 0.68),
            border: Color::new(0.54, 0.48, 0.38, 0.72),
            text: title_parchment(),
        },
    }
}

fn title_parchment() -> Color {
    Color::new(0.92, 0.82, 0.62, 1.0)
}
