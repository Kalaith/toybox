use super::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;

pub(crate) fn draw_title_screen(
    title_texture: Option<&Texture2D>,
    continue_enabled: bool,
) -> Vec<UiAction> {
    draw_title_background(title_texture);
    draw_title_scrim();

    let mut actions = Vec::new();
    let mouse = logical_mouse_position();
    let button_w = 148.0;
    let button_h = 38.0;
    let button_gap = 14.0;
    let total_w = button_w * 4.0 + button_gap * 3.0;
    let x = (LOGICAL_WIDTH - total_w) * 0.5;
    let y = 614.0;

    if title_button(
        Rect::new(x, y, button_w, button_h),
        "New Game",
        true,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::NewGame);
    }

    if title_button(
        Rect::new(x + button_w + button_gap, y, button_w, button_h),
        "Continue",
        continue_enabled,
        ButtonTone::Positive,
        mouse,
    ) {
        actions.push(UiAction::Continue);
    }

    if title_button(
        Rect::new(x + (button_w + button_gap) * 2.0, y, button_w, button_h),
        "Settings",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::Settings);
    }

    if title_button(
        Rect::new(x + (button_w + button_gap) * 3.0, y, button_w, button_h),
        "Quit Game",
        true,
        ButtonTone::Danger,
        mouse,
    ) {
        actions.push(UiAction::QuitGame);
    }

    actions
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
    draw_title_button_plaque(
        Rect::new(fov_x + 50.0, row1_y, 96.0, button_h),
        ButtonTone::Muted,
        true,
        false,
        false,
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

    if from_game
        && title_button(
            Rect::new(row2_x + button_w + button_gap, row2_y, button_w, button_h),
            "Quit to Title",
            true,
            ButtonTone::Danger,
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
    let hovered = enabled && rect.contains_point(mouse);
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let activated = hovered && is_mouse_button_released(MouseButton::Left);
    draw_title_button_plaque(rect, tone, enabled, hovered, pressed);

    let palette = title_button_palette(tone);
    let text_color = if enabled {
        palette.text
    } else {
        Color::new(0.42, 0.40, 0.36, 1.0)
    };
    draw_text_centered_in_box_ex(
        text,
        rect.x + 8.0,
        rect.y + if pressed { 1.0 } else { 0.0 } - 1.0,
        rect.w - 16.0,
        rect.h,
        TextStyle::new(15.0, text_color),
    );

    activated
}

fn draw_title_button_plaque(
    rect: Rect,
    tone: ButtonTone,
    enabled: bool,
    hovered: bool,
    pressed: bool,
) {
    let palette = title_button_palette(tone);
    let inset = Rect::new(rect.x + 3.0, rect.y + 3.0, rect.w - 6.0, rect.h - 6.0);
    let face = if !enabled {
        palette.disabled
    } else if pressed {
        palette.pressed
    } else if hovered {
        palette.hovered
    } else {
        palette.normal
    };
    let border = if enabled {
        palette.border
    } else {
        Color::new(0.30, 0.28, 0.24, 0.78)
    };

    draw_rectangle(
        rect.x + 2.0,
        rect.y + 3.0,
        rect.w,
        rect.h,
        Color::new(0.015, 0.012, 0.010, 0.58),
    );
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.040, 0.036, 0.030, 0.94))
            .with_border(1.0, Color::new(0.18, 0.12, 0.06, 0.92))
            .with_inner_border(2.0, 1.0, Color::new(0.80, 0.55, 0.24, 0.20)),
    );
    draw_surface(
        inset,
        &SurfaceStyle::new(face)
            .with_border(1.0, border)
            .with_top_highlight(2.0, Color::new(1.0, 0.82, 0.42, 0.20)),
    );
    draw_title_corner_marks(
        rect,
        if enabled {
            border
        } else {
            Color::new(0.30, 0.28, 0.24, 0.60)
        },
    );
}

fn draw_title_corner_marks(rect: Rect, color: Color) {
    let gap = 6.0;
    let len = 11.0;
    draw_line(
        rect.x + gap,
        rect.y + gap,
        rect.x + gap + len,
        rect.y + gap,
        1.0,
        color,
    );
    draw_line(
        rect.x + gap,
        rect.y + gap,
        rect.x + gap,
        rect.y + gap + len,
        1.0,
        color,
    );
    draw_line(
        rect.right() - gap - len,
        rect.y + gap,
        rect.right() - gap,
        rect.y + gap,
        1.0,
        color,
    );
    draw_line(
        rect.right() - gap,
        rect.y + gap,
        rect.right() - gap,
        rect.y + gap + len,
        1.0,
        color,
    );
    draw_line(
        rect.x + gap,
        rect.bottom() - gap,
        rect.x + gap + len,
        rect.bottom() - gap,
        1.0,
        color,
    );
    draw_line(
        rect.x + gap,
        rect.bottom() - gap - len,
        rect.x + gap,
        rect.bottom() - gap,
        1.0,
        color,
    );
    draw_line(
        rect.right() - gap - len,
        rect.bottom() - gap,
        rect.right() - gap,
        rect.bottom() - gap,
        1.0,
        color,
    );
    draw_line(
        rect.right() - gap,
        rect.bottom() - gap - len,
        rect.right() - gap,
        rect.bottom() - gap,
        1.0,
        color,
    );
}

#[derive(Debug, Clone, Copy)]
struct TitleButtonPalette {
    normal: Color,
    hovered: Color,
    pressed: Color,
    disabled: Color,
    border: Color,
    text: Color,
}

fn title_button_palette(tone: ButtonTone) -> TitleButtonPalette {
    match tone {
        ButtonTone::Primary => TitleButtonPalette {
            normal: Color::new(0.12, 0.20, 0.31, 0.92),
            hovered: Color::new(0.18, 0.29, 0.44, 0.96),
            pressed: Color::new(0.08, 0.14, 0.23, 0.98),
            disabled: Color::new(0.08, 0.10, 0.13, 0.72),
            border: Color::new(0.58, 0.64, 0.74, 0.86),
            text: title_parchment(),
        },
        ButtonTone::Positive => TitleButtonPalette {
            normal: Color::new(0.11, 0.28, 0.17, 0.92),
            hovered: Color::new(0.17, 0.38, 0.23, 0.96),
            pressed: Color::new(0.07, 0.20, 0.12, 0.98),
            disabled: Color::new(0.07, 0.10, 0.08, 0.68),
            border: Color::new(0.55, 0.72, 0.42, 0.84),
            text: title_parchment(),
        },
        ButtonTone::Danger => TitleButtonPalette {
            normal: Color::new(0.31, 0.12, 0.10, 0.92),
            hovered: Color::new(0.45, 0.17, 0.14, 0.96),
            pressed: Color::new(0.22, 0.08, 0.07, 0.98),
            disabled: Color::new(0.14, 0.08, 0.08, 0.70),
            border: Color::new(0.78, 0.42, 0.34, 0.80),
            text: title_parchment(),
        },
        ButtonTone::Muted | ButtonTone::Secondary | ButtonTone::Warning => TitleButtonPalette {
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

fn logical_mouse_position() -> Vec2 {
    let (screen_x, screen_y) = mouse_position();
    vec2(
        screen_x * LOGICAL_WIDTH / screen_width().max(1.0),
        screen_y * LOGICAL_HEIGHT / screen_height().max(1.0),
    )
}
