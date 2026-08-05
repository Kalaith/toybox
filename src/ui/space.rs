//! Fixed logical UI space and camera setup.

use crate::ui::{LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct UiSpace;

impl UiSpace {
    pub fn new() -> Self {
        Self
    }
}

pub fn begin_ui_frame(user_scale: f32) -> UiSpace {
    let responsive =
        macroquad_toolkit::ui::set_ui_text_scale_for_screen(LOGICAL_WIDTH, LOGICAL_HEIGHT, 1.45);
    macroquad_toolkit::ui::set_ui_text_scale((responsive * user_scale).clamp(0.80, 1.65));
    set_ui_camera();
    UiSpace::new()
}

pub fn set_ui_camera() {
    macroquad::camera::set_camera(&macroquad::camera::Camera2D {
        target: vec2(LOGICAL_WIDTH * 0.5, LOGICAL_HEIGHT * 0.5),
        zoom: vec2(2.0 / LOGICAL_WIDTH, 2.0 / LOGICAL_HEIGHT),
        ..Default::default()
    });
}

pub fn end_ui_frame() {
    macroquad::camera::set_default_camera();
}
