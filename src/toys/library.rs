//! Shared deterministic toy generation and rendering helpers.

use crate::data::{DisplayDef, ToyCategory};
use crate::state::ToyState;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToyIdentity {
    Bear,
    Duck,
    Rabbit,
    Cat,
    Puppy,
    Elephant,
    Owl,
    Turtle,
    Penguin,
    Octopus,
    CrescentDragon,
    HornedDragon,
    FinDragon,
    SpikeDragon,
    LongtailDragon,
    WyrmDragon,
    PudgyDragon,
    TwinDragon,
    HatchlingDragon,
    CurledDragon,
    AntennaBot,
    ClawBot,
    TreadBot,
    ScreenBot,
    TripodBot,
    DomeBot,
    BoxyBot,
    RollerBot,
    CrabBot,
    RocketBot,
    MazeBox,
    CastleQuest,
    PlanetRace,
    WordTiles,
    TreasureMap,
    TowerBlocks,
    ArchBlocks,
    BridgeBlocks,
    CastleBlocks,
    TrainBlocks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToyProfile {
    pub identity: ToyIdentity,
    pub label: &'static str,
    pub short_code: &'static str,
    pub detail_index: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ToySpawnPose {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub floor_lift: f32,
}

impl ToySpawnPose {
    pub fn is_uninitialized(self) -> bool {
        self.yaw == 0.0 && self.pitch == 0.0 && self.roll == 0.0 && self.floor_lift == 0.0
    }

    #[cfg(test)]
    pub fn is_tumbled(self) -> bool {
        self.pitch.abs() > 0.4 || self.roll.abs() > 0.4
    }
}

impl Default for ToySpawnPose {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            floor_lift: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct IdentityDef {
    identity: ToyIdentity,
    label: &'static str,
    short_code: &'static str,
}

const PLUSH_IDENTITIES: [IdentityDef; 10] = [
    IdentityDef {
        identity: ToyIdentity::Bear,
        label: "Bear",
        short_code: "BR",
    },
    IdentityDef {
        identity: ToyIdentity::Duck,
        label: "Duck",
        short_code: "DK",
    },
    IdentityDef {
        identity: ToyIdentity::Rabbit,
        label: "Rabbit",
        short_code: "RB",
    },
    IdentityDef {
        identity: ToyIdentity::Cat,
        label: "Cat",
        short_code: "CT",
    },
    IdentityDef {
        identity: ToyIdentity::Puppy,
        label: "Puppy",
        short_code: "PP",
    },
    IdentityDef {
        identity: ToyIdentity::Elephant,
        label: "Elephant",
        short_code: "EL",
    },
    IdentityDef {
        identity: ToyIdentity::Owl,
        label: "Owl",
        short_code: "OW",
    },
    IdentityDef {
        identity: ToyIdentity::Turtle,
        label: "Turtle",
        short_code: "TU",
    },
    IdentityDef {
        identity: ToyIdentity::Penguin,
        label: "Penguin",
        short_code: "PG",
    },
    IdentityDef {
        identity: ToyIdentity::Octopus,
        label: "Octopus",
        short_code: "OC",
    },
];

const DRAGON_IDENTITIES: [IdentityDef; 10] = [
    IdentityDef {
        identity: ToyIdentity::CrescentDragon,
        label: "Crescent Dragon",
        short_code: "CR",
    },
    IdentityDef {
        identity: ToyIdentity::HornedDragon,
        label: "Horned Dragon",
        short_code: "HN",
    },
    IdentityDef {
        identity: ToyIdentity::FinDragon,
        label: "Fin Dragon",
        short_code: "FN",
    },
    IdentityDef {
        identity: ToyIdentity::SpikeDragon,
        label: "Spike Dragon",
        short_code: "SP",
    },
    IdentityDef {
        identity: ToyIdentity::LongtailDragon,
        label: "Longtail Dragon",
        short_code: "LT",
    },
    IdentityDef {
        identity: ToyIdentity::WyrmDragon,
        label: "Wyrm Dragon",
        short_code: "WY",
    },
    IdentityDef {
        identity: ToyIdentity::PudgyDragon,
        label: "Pudgy Dragon",
        short_code: "PD",
    },
    IdentityDef {
        identity: ToyIdentity::TwinDragon,
        label: "Twin Dragon",
        short_code: "TD",
    },
    IdentityDef {
        identity: ToyIdentity::HatchlingDragon,
        label: "Hatchling Dragon",
        short_code: "HD",
    },
    IdentityDef {
        identity: ToyIdentity::CurledDragon,
        label: "Curled Dragon",
        short_code: "CD",
    },
];

const ROBOT_IDENTITIES: [IdentityDef; 10] = [
    IdentityDef {
        identity: ToyIdentity::AntennaBot,
        label: "Antenna Bot",
        short_code: "AN",
    },
    IdentityDef {
        identity: ToyIdentity::ClawBot,
        label: "Claw Bot",
        short_code: "CL",
    },
    IdentityDef {
        identity: ToyIdentity::TreadBot,
        label: "Tread Bot",
        short_code: "TR",
    },
    IdentityDef {
        identity: ToyIdentity::ScreenBot,
        label: "Screen Bot",
        short_code: "SC",
    },
    IdentityDef {
        identity: ToyIdentity::TripodBot,
        label: "Tripod Bot",
        short_code: "TP",
    },
    IdentityDef {
        identity: ToyIdentity::DomeBot,
        label: "Dome Bot",
        short_code: "DM",
    },
    IdentityDef {
        identity: ToyIdentity::BoxyBot,
        label: "Boxy Bot",
        short_code: "BX",
    },
    IdentityDef {
        identity: ToyIdentity::RollerBot,
        label: "Roller Bot",
        short_code: "RL",
    },
    IdentityDef {
        identity: ToyIdentity::CrabBot,
        label: "Crab Bot",
        short_code: "CB",
    },
    IdentityDef {
        identity: ToyIdentity::RocketBot,
        label: "Rocket Bot",
        short_code: "RK",
    },
];

const BOARD_GAME_IDENTITIES: [IdentityDef; 5] = [
    IdentityDef {
        identity: ToyIdentity::MazeBox,
        label: "Maze Box",
        short_code: "MZ",
    },
    IdentityDef {
        identity: ToyIdentity::CastleQuest,
        label: "Castle Quest",
        short_code: "CQ",
    },
    IdentityDef {
        identity: ToyIdentity::PlanetRace,
        label: "Planet Race",
        short_code: "PR",
    },
    IdentityDef {
        identity: ToyIdentity::WordTiles,
        label: "Word Tiles",
        short_code: "WT",
    },
    IdentityDef {
        identity: ToyIdentity::TreasureMap,
        label: "Treasure Map",
        short_code: "TM",
    },
];

const BLOCK_IDENTITIES: [IdentityDef; 5] = [
    IdentityDef {
        identity: ToyIdentity::TowerBlocks,
        label: "Tower Blocks",
        short_code: "TW",
    },
    IdentityDef {
        identity: ToyIdentity::ArchBlocks,
        label: "Arch Blocks",
        short_code: "AR",
    },
    IdentityDef {
        identity: ToyIdentity::BridgeBlocks,
        label: "Bridge Blocks",
        short_code: "BG",
    },
    IdentityDef {
        identity: ToyIdentity::CastleBlocks,
        label: "Castle Blocks",
        short_code: "CA",
    },
    IdentityDef {
        identity: ToyIdentity::TrainBlocks,
        label: "Train Blocks",
        short_code: "TN",
    },
];

pub fn toy_profile(category: ToyCategory, slot_number: usize) -> ToyProfile {
    let identities: &[IdentityDef] = match category {
        ToyCategory::Plushies => &PLUSH_IDENTITIES,
        ToyCategory::TinyDragons => &DRAGON_IDENTITIES,
        ToyCategory::ActionFigures => &ROBOT_IDENTITIES,
        ToyCategory::BoardGames => &BOARD_GAME_IDENTITIES,
        ToyCategory::BuildingBlocks => &BLOCK_IDENTITIES,
    };
    let stable_index = slot_number.saturating_sub(1);
    let identity = identities[stable_index % identities.len()];

    ToyProfile {
        identity: identity.identity,
        label: identity.label,
        short_code: identity.short_code,
        detail_index: stable_index / identities.len(),
    }
}

pub fn toy_name(display: &DisplayDef, slot_number: usize) -> String {
    let profile = toy_profile(display.category, slot_number);
    format!("{} {} #{slot_number:02}", display.theme, profile.label)
}

pub fn spawn_pose_for_toy(
    toy_index: usize,
    display_index: usize,
    slot_index: usize,
) -> ToySpawnPose {
    let yaw_seed = (toy_index * 53 + display_index * 17 + slot_index * 29) % 360;
    let yaw = (yaw_seed as f32 + 7.0).to_radians();
    let lean = (((toy_index * 31 + slot_index * 11) % 21) as f32 - 10.0).to_radians();

    match (toy_index * 7 + display_index * 5 + slot_index * 3) % 8 {
        0 => ToySpawnPose {
            yaw,
            pitch: lean * 0.4,
            roll: 0.22,
            floor_lift: 0.03,
        },
        1 => ToySpawnPose {
            yaw,
            pitch: lean,
            roll: std::f32::consts::FRAC_PI_2,
            floor_lift: 0.17,
        },
        2 => ToySpawnPose {
            yaw,
            pitch: -lean,
            roll: -std::f32::consts::FRAC_PI_2,
            floor_lift: 0.17,
        },
        3 => ToySpawnPose {
            yaw,
            pitch: lean * 0.5,
            roll: std::f32::consts::PI,
            floor_lift: 0.48,
        },
        4 => ToySpawnPose {
            yaw,
            pitch: std::f32::consts::FRAC_PI_2,
            roll: lean,
            floor_lift: 0.24,
        },
        5 => ToySpawnPose {
            yaw,
            pitch: -std::f32::consts::FRAC_PI_2,
            roll: -lean,
            floor_lift: 0.24,
        },
        6 => ToySpawnPose {
            yaw,
            pitch: 0.38 + lean * 0.3,
            roll: std::f32::consts::FRAC_PI_2 * 1.35,
            floor_lift: 0.30,
        },
        _ => ToySpawnPose {
            yaw,
            pitch: -0.30 + lean * 0.3,
            roll: -std::f32::consts::FRAC_PI_2 * 1.35,
            floor_lift: 0.30,
        },
    }
}

pub fn toy_color(toy: &ToyState) -> Color {
    let index = toy.slot_number + toy.color_index * 3;
    let base = match index % 9 {
        0 => Color::new(0.78, 0.22, 0.18, 1.0),
        1 => Color::new(0.96, 0.66, 0.22, 1.0),
        2 => Color::new(0.30, 0.72, 0.50, 1.0),
        3 => Color::new(0.22, 0.58, 0.88, 1.0),
        4 => Color::new(0.68, 0.42, 0.86, 1.0),
        5 => Color::new(0.88, 0.36, 0.54, 1.0),
        6 => Color::new(0.18, 0.68, 0.72, 1.0),
        7 => Color::new(0.90, 0.82, 0.46, 1.0),
        _ => Color::new(0.72, 0.56, 0.42, 1.0),
    };
    let offset = ((toy.slot_number * 7 + toy.color_index * 5) % 7) as f32 * 0.015 - 0.045;
    Color::new(
        (base.r + offset).clamp(0.08, 0.98),
        (base.g + offset).clamp(0.08, 0.98),
        (base.b + offset).clamp(0.08, 0.98),
        1.0,
    )
}

pub fn brighten(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r + amount).clamp(0.0, 1.0),
        (color.g + amount).clamp(0.0, 1.0),
        (color.b + amount).clamp(0.0, 1.0),
        color.a,
    )
}

pub fn darken(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r - amount).clamp(0.0, 1.0),
        (color.g - amount).clamp(0.0, 1.0),
        (color.b - amount).clamp(0.0, 1.0),
        color.a,
    )
}

pub fn draw_cube_with_edges(center: Vec3, size: Vec3, color: Color) {
    draw_cube(center, size, None, color);
    draw_cube_wires(center, size, darken(color, 0.24));
    draw_cube(
        center + vec3(0.0, size.y * 0.51, -size.z * 0.08),
        vec3(size.x * 0.72, size.y * 0.045, size.z * 0.66),
        None,
        Color::new(1.0, 1.0, 0.90, 0.18),
    );
}

/// Drop-in `draw_sphere` replacement at 8x8 tessellation. Macroquad's
/// default is 16x16 and the mesh is regenerated per call per frame; toy
/// spheres are small on screen, so the low-poly version reads the same
/// at a quarter of the vertex cost.
pub fn draw_toy_sphere(center: Vec3, radius: f32, texture: Option<&Texture2D>, color: Color) {
    draw_sphere_ex(
        center,
        radius,
        texture,
        color,
        DrawSphereParams {
            rings: 8,
            slices: 8,
            ..Default::default()
        },
    );
}

pub fn draw_studded_block(center: Vec3, size: Vec3, color: Color) {
    draw_cube_with_edges(center, size, color);
    let stud_color = brighten(color, 0.10);
    let stud_radius = size.x.min(size.z) * 0.12;
    let y = center.y + size.y * 0.55;
    for x in [-0.24_f32, 0.24] {
        for z in [-0.22_f32, 0.22] {
            draw_toy_sphere(
                vec3(center.x + size.x * x, y, center.z + size.z * z),
                stud_radius,
                None,
                stud_color,
            );
        }
    }
}

pub fn draw_wheel(center: Vec3, radius: f32, width: f32, color: Color) {
    draw_toy_sphere(center, radius, None, color);
    draw_cube(
        center,
        vec3(radius * 1.32, width, radius * 1.32),
        None,
        darken(color, 0.06),
    );
    draw_toy_sphere(
        center,
        radius * 0.42,
        None,
        Color::new(0.82, 0.84, 0.80, 1.0),
    );
}

pub fn draw_face(center: Vec3, y: f32, z: f32, dx: f32, scale: f32) {
    draw_eye_pair(center, y, z, dx, scale);
    draw_face_mark(
        center + vec3(0.0, y - 0.04, z - 0.02) * scale,
        vec3(0.050, 0.034, 0.016) * scale,
    );
    draw_face_mark(
        center + vec3(-dx * 1.25, y - 0.055, z - 0.018) * scale,
        vec3(0.035, 0.018, 0.010) * scale,
    );
    draw_face_mark(
        center + vec3(dx * 1.25, y - 0.055, z - 0.018) * scale,
        vec3(0.035, 0.018, 0.010) * scale,
    );
}

pub fn draw_eye_pair(center: Vec3, y: f32, z: f32, dx: f32, scale: f32) {
    let size = vec3(0.052, 0.052, 0.016) * scale;
    draw_face_mark(center + vec3(-dx, y, z) * scale, size);
    draw_face_mark(center + vec3(dx, y, z) * scale, size);
    let sparkle = vec3(0.016, 0.016, 0.006) * scale;
    draw_cube(
        center + vec3(-dx - 0.010, y + 0.012, z - 0.010) * scale,
        sparkle,
        None,
        Color::new(0.92, 0.94, 0.90, 1.0),
    );
    draw_cube(
        center + vec3(dx - 0.010, y + 0.012, z - 0.010) * scale,
        sparkle,
        None,
        Color::new(0.92, 0.94, 0.90, 1.0),
    );
}

fn draw_face_mark(center: Vec3, size: Vec3) {
    draw_cube(center, size, None, Color::new(0.035, 0.030, 0.026, 1.0));
}

pub fn draw_dragon_base(center: Vec3, color: Color, scale: f32) {
    draw_toy_sphere(center, 0.24 * scale, None, color);
    draw_toy_sphere(
        center + vec3(0.0, -0.01, -0.15) * scale,
        0.15 * scale,
        None,
        Color::new(0.90, 0.76, 0.54, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.18, -0.30) * scale,
        0.14 * scale,
        None,
        brighten(color, 0.08),
    );
    draw_cube(
        center + vec3(-0.27, 0.06, 0.03) * scale,
        vec3(0.10, 0.26, 0.38) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(-0.31, 0.12, -0.03) * scale,
        vec3(0.055, 0.18, 0.26) * scale,
        None,
        brighten(color, 0.10),
    );
    draw_cube(
        center + vec3(0.27, 0.06, 0.03) * scale,
        vec3(0.10, 0.26, 0.38) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(0.31, 0.12, -0.03) * scale,
        vec3(0.055, 0.18, 0.26) * scale,
        None,
        brighten(color, 0.10),
    );
    for x in [-0.12_f32, 0.12] {
        draw_cube(
            center + vec3(x, -0.18, -0.15) * scale,
            vec3(0.10, 0.08, 0.16) * scale,
            None,
            darken(color, 0.10),
        );
    }
    for index in 0..3 {
        draw_cube(
            center + vec3(0.0, 0.03 + index as f32 * 0.055, -0.23) * scale,
            vec3(0.11, 0.018, 0.030) * scale,
            None,
            Color::new(0.98, 0.86, 0.58, 1.0),
        );
    }
    draw_face(center, 0.20, -0.40, 0.06, scale);
}

pub fn draw_robot_core(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(
        center + vec3(0.0, 0.08, 0.0) * scale,
        vec3(0.34, 0.36, 0.30) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.39, -0.01) * scale,
        vec3(0.28, 0.24, 0.26) * scale,
        brighten(color, 0.10),
    );
    draw_cube(
        center + vec3(0.0, 0.09, -0.17) * scale,
        vec3(0.18, 0.12, 0.025) * scale,
        None,
        Color::new(0.08, 0.12, 0.14, 1.0),
    );
    for x in [-0.07_f32, 0.07] {
        draw_toy_sphere(
            center + vec3(x, 0.11, -0.19) * scale,
            0.020 * scale,
            None,
            Color::new(0.56, 0.94, 0.88, 1.0),
        );
    }
    draw_eye_pair(center, 0.42, -0.16, 0.07, scale);
}

pub fn draw_robot_arms(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(
        center + vec3(-0.26, 0.08, 0.0) * scale,
        vec3(0.10, 0.30, 0.12) * scale,
        darken(color, 0.12),
    );
    draw_cube_with_edges(
        center + vec3(0.26, 0.08, 0.0) * scale,
        vec3(0.10, 0.30, 0.12) * scale,
        darken(color, 0.12),
    );
    for x in [-0.26_f32, 0.26] {
        draw_toy_sphere(
            center + vec3(x, -0.10, -0.01) * scale,
            0.055 * scale,
            None,
            Color::new(0.80, 0.84, 0.84, 1.0),
        );
    }
}

pub fn draw_game_box(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(center, vec3(0.56, 0.12, 0.40) * scale, color);
    draw_cube(
        center + vec3(0.0, 0.08, -0.01) * scale,
        vec3(0.46, 0.035, 0.30) * scale,
        None,
        brighten(color, 0.12),
    );
    draw_cube(
        center + vec3(-0.23, 0.105, -0.01) * scale,
        vec3(0.035, 0.020, 0.31) * scale,
        None,
        Color::new(0.95, 0.82, 0.38, 1.0),
    );
    draw_cube(
        center + vec3(0.06, 0.110, -0.14) * scale,
        vec3(0.22, 0.018, 0.030) * scale,
        None,
        Color::new(0.98, 0.94, 0.76, 1.0),
    );
}

pub fn shift_block_color(color: Color, index: usize) -> Color {
    match index % 3 {
        0 => color,
        1 => brighten(color, 0.10),
        _ => darken(color, 0.08),
    }
}
