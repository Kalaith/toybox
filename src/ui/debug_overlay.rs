//! F3-toggled debug overlay: frame timing, draw counts, and player pose.

use crate::ui::scene3d::SceneStats;
use crate::ui::UiContext;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::draw_ui_text_ex;

pub struct DebugOverlay {
    visible: bool,
    smoothed_frame_seconds: f32,
}

impl DebugOverlay {
    pub fn new() -> Self {
        Self {
            visible: false,
            smoothed_frame_seconds: 0.0,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn record_frame(&mut self, dt: f32) {
        if self.smoothed_frame_seconds <= 0.0 {
            self.smoothed_frame_seconds = dt;
        } else {
            // Exponential moving average keeps the readout steady enough to read.
            self.smoothed_frame_seconds += (dt - self.smoothed_frame_seconds) * 0.08;
        }
    }

    pub fn draw(&self, ctx: &UiContext<'_>, stats: &SceneStats) {
        if !self.visible {
            return;
        }

        let frame_ms = self.smoothed_frame_seconds * 1000.0;
        let fps = if self.smoothed_frame_seconds > 0.0 {
            1.0 / self.smoothed_frame_seconds
        } else {
            0.0
        };
        let player = &ctx.session.player;
        let lines = [
            format!("{:.0} FPS  {:.2} ms", fps, frame_ms),
            format!(
                "Toys drawn {} / {}",
                stats.drawn_toys,
                ctx.session.toys.len()
            ),
            format!(
                "Pos {:.1}, {:.1}  Yaw {:.0}",
                player.position.x,
                player.position.y,
                player.yaw.to_degrees().rem_euclid(360.0)
            ),
        ];

        // Sits to the right of the HUD status panel (18,16,190x166).
        let panel = Rect::new(222.0, 16.0, 236.0, 16.0 + lines.len() as f32 * 22.0);
        draw_surface(
            panel,
            &SurfaceStyle::new(Color::new(0.02, 0.03, 0.04, 0.82))
                .with_border(1.0, Color::new(0.55, 0.64, 0.72, 0.55)),
        );
        for (index, line) in lines.iter().enumerate() {
            draw_ui_text_ex(
                line,
                panel.x + 12.0,
                panel.y + 24.0 + index as f32 * 22.0,
                TextStyle::new(15.0, dark::TEXT_BRIGHT).params(),
            );
        }
    }
}
