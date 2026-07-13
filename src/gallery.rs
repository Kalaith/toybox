//! Capture-only toy gallery: renders a single toy identity from four
//! compass directions in a 2x2 grid. Driven by the screenshot harness
//! (`TOYBOX_CAPTURE_SCENE=toy_gallery` + `TOYBOX_CAPTURE_TOY=<slug>`);
//! never reachable from normal play.

use crate::data::ToyCategory;
use crate::state::{RepairState, ToyState, WorldPoint};
use crate::toys::{draw_toy_3d, toy_color, toy_profile, ToySpawnPose};
use macroquad::prelude::*;

const CATEGORIES: [ToyCategory; 5] = [
    ToyCategory::Plushies,
    ToyCategory::TinyDragons,
    ToyCategory::ActionFigures,
    ToyCategory::BoardGames,
    ToyCategory::BuildingBlocks,
];

pub struct GalleryScene {
    toy: ToyState,
    label: String,
}

impl GalleryScene {
    /// Build the scene for a toy slug (identity module name, e.g.
    /// "antenna_bot"). Unknown slugs fall back to the bear so the capture
    /// still produces an image, with the problem spelled out in the label.
    pub fn new(slug: &str) -> Self {
        let (category, slot_number, label) = match find_identity(slug) {
            Some(found) => found,
            None => (
                ToyCategory::Plushies,
                1,
                format!("UNKNOWN TOY '{slug}' (showing bear)"),
            ),
        };
        let toy = ToyState {
            id: "gallery".to_owned(),
            name: label.clone(),
            category,
            theme: "Gallery".to_owned(),
            slot_number,
            color_index: 0,
            position: WorldPoint { x: 0.0, y: 0.0 },
            spawn_pose: ToySpawnPose::default(),
            is_held: false,
            placed_display_id: None,
            placed_slot_index: None,
            bench_slot_index: None,
            bench_id: None,
            wrong_marker_seconds: 0.0,
            repair_state: RepairState::Whole,
        };
        Self { toy, label }
    }

    pub fn draw(&self) {
        clear_background(Color::new(0.10, 0.11, 0.13, 1.0));
        let half_w = (screen_width() * 0.5) as i32;
        let half_h = (screen_height() * 0.5) as i32;

        // Viewport origin is bottom-left (GL convention), so the top row
        // uses y = half_h. Reading order on screen: Front, Right / Back, Left.
        let views = [
            (vec3(0.0, 0.62, -2.15), (0, half_h)),
            (vec3(2.15, 0.62, 0.0), (half_w, half_h)),
            (vec3(0.0, 0.62, 2.15), (0, 0)),
            (vec3(-2.15, 0.62, 0.0), (half_w, 0)),
        ];
        for (eye, (view_x, view_y)) in views {
            set_camera(&Camera3D {
                position: eye,
                target: vec3(0.0, 0.33, 0.0),
                up: vec3(0.0, 1.0, 0.0),
                fovy: 45.0_f32.to_radians(),
                projection: Projection::Perspective,
                aspect: Some(half_w as f32 / half_h.max(1) as f32),
                viewport: Some((view_x, view_y, half_w, half_h)),
                ..Default::default()
            });
            draw_plane(
                vec3(0.0, 0.0, 0.0),
                vec2(4.0, 4.0),
                None,
                Color::new(0.17, 0.18, 0.20, 1.0),
            );
            draw_toy_3d(&self.toy, vec3(0.0, 0.30, 0.0), toy_color(&self.toy), 1.0);
        }

        // 2D overlay: quadrant separators, view labels, toy name.
        set_default_camera();
        let width = screen_width();
        let height = screen_height();
        let line = Color::new(0.30, 0.32, 0.36, 1.0);
        draw_line(width * 0.5, 0.0, width * 0.5, height, 2.0, line);
        draw_line(0.0, height * 0.5, width, height * 0.5, 2.0, line);

        let text = Color::new(0.92, 0.88, 0.78, 1.0);
        let labels = [
            ("Front", 14.0, 30.0),
            ("Right", width * 0.5 + 14.0, 30.0),
            ("Back", 14.0, height * 0.5 + 30.0),
            ("Left", width * 0.5 + 14.0, height * 0.5 + 30.0),
        ];
        for (label, x, y) in labels {
            draw_text(label, x, y, 26.0, text);
        }
        let name_size = 30.0;
        let name_width = measure_text(&self.label, None, name_size as u16, 1.0).width;
        draw_text(
            &self.label,
            (width - name_width) * 0.5,
            height - 18.0,
            name_size,
            text,
        );
    }
}

fn find_identity(slug: &str) -> Option<(ToyCategory, usize, String)> {
    for category in CATEGORIES {
        for slot_number in 1..=10 {
            let label = toy_profile(category, slot_number).label;
            if label.to_lowercase().replace(' ', "_") == slug {
                return Some((category, slot_number, label.to_owned()));
            }
        }
    }
    None
}
