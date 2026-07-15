//! Small reusable immediate-mode UI widgets.

use macroquad::prelude::*;
use macroquad_toolkit::prelude::TextStyle;
use macroquad_toolkit::ui::{draw_ui_text_ex, truncate_text_to_width};

pub(super) fn draw_fitted_text(
    text: &str,
    x: f32,
    baseline_y: f32,
    max_width: f32,
    size: f32,
    color: Color,
) {
    let fitted = truncate_text_to_width(text, max_width, size);
    draw_ui_text_ex(&fitted, x, baseline_y, TextStyle::new(size, color).params());
}
