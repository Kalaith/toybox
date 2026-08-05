//! First-shift guidance card. Gameplay prompts remain the authority for the
//! immediate action; this card explains the larger reason for that action.

use super::hud_chrome::{brass, draw_hud_panel, draw_keycap, parchment, warm_panel};
use crate::tutorial::TutorialHint;
use crate::ui::widgets::{draw_wrapped_text, WrapStyle};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::TextStyle;
use macroquad_toolkit::ui::draw_ui_text_ex;

pub(super) fn draw_tutorial_hint(hint: &TutorialHint) {
    let rect = Rect::new(630.0, 16.0, 364.0, 124.0);
    draw_hud_panel(rect, warm_panel(0.94), brass(0.72));
    draw_ui_text_ex(
        hint.eyebrow,
        rect.x + 18.0,
        rect.y + 25.0,
        TextStyle::new(10.0, Color::new(1.0, 0.66, 0.20, 0.88)).params(),
    );
    draw_ui_text_ex(
        hint.title,
        rect.x + 18.0,
        rect.y + 49.0,
        TextStyle::new(17.0, parchment(1.0)).params(),
    );
    draw_wrapped_text(
        hint.body,
        rect.x + 18.0,
        rect.y + 69.0,
        rect.w - 36.0,
        WrapStyle {
            size: 12.0,
            line_height: 16.0,
            max_lines: 2,
            color: parchment(0.68),
        },
    );

    let mut x = rect.x + 18.0;
    for key in hint.keys {
        let width = if key.len() > 2 { 48.0 } else { 26.0 };
        draw_keycap(Rect::new(x, rect.bottom() - 25.0, width, 19.0), key, false);
        x += width + 7.0;
    }
    draw_keycap(
        Rect::new(rect.right() - 78.0, rect.bottom() - 25.0, 24.0, 19.0),
        "H",
        false,
    );
    draw_ui_text_ex(
        "SKIP",
        rect.right() - 47.0,
        rect.bottom() - 10.0,
        TextStyle::new(10.0, parchment(0.50)).params(),
    );
}
