//! Small reusable immediate-mode UI widgets.

use macroquad::prelude::*;
use macroquad_toolkit::prelude::TextStyle;
use macroquad_toolkit::ui::{draw_ui_text_ex, measure_ui_text};

pub(super) fn draw_fitted_text(
    text: &str,
    x: f32,
    baseline_y: f32,
    max_width: f32,
    size: f32,
    color: Color,
) {
    let mut fitted = text.to_owned();
    while !fitted.is_empty() && measure_ui_text(&fitted, None, size as u16, 1.0).width > max_width {
        fitted.pop();
    }
    if fitted.len() < text.len() && fitted.len() > 3 {
        fitted.truncate(fitted.len() - 3);
        fitted.push_str("...");
    }
    draw_ui_text_ex(&fitted, x, baseline_y, TextStyle::new(size, color).params());
}
