//! Procedural toy library and per-identity renderers.

use crate::state::{RepairPartKind, ToyState};
use library::{draw_toy_sphere, ToyIdentity};
use macroquad::prelude::*;

mod antenna_bot;
mod arch_blocks;
mod bear;
mod bridge_blocks;
mod castle_blocks;
mod castle_quest;
mod cat;
mod claw_bot;
mod crescent_dragon;
mod curled_dragon;
mod duck;
mod elephant;
mod fin_dragon;
mod hatchling_dragon;
mod horned_dragon;
pub mod library;
mod longtail_dragon;
mod maze_box;
mod octopus;
mod owl;
mod penguin;
mod planet_race;
mod pudgy_dragon;
mod puppy;
mod rabbit;
mod screen_bot;
mod spike_dragon;
mod tower_blocks;
mod train_blocks;
mod tread_bot;
mod treasure_map;
mod tripod_bot;
mod turtle;
mod twin_dragon;
mod word_tiles;
mod wyrm_dragon;

pub use library::{brighten, spawn_pose_for_toy, toy_color, toy_name, toy_profile, ToySpawnPose};

pub fn draw_loose_toy_3d(toy: &ToyState, center: Vec3, color: Color, scale: f32) {
    let matrix = glam::Mat4::from_translation(center)
        * glam::Mat4::from_rotation_y(toy.spawn_pose.yaw)
        * glam::Mat4::from_rotation_z(toy.spawn_pose.roll)
        * glam::Mat4::from_rotation_x(toy.spawn_pose.pitch)
        * glam::Mat4::from_translation(-center);

    unsafe {
        get_internal_gl().quad_gl.push_model_matrix(matrix);
    }
    draw_toy_3d(toy, center, color, scale);
    unsafe {
        get_internal_gl().quad_gl.pop_model_matrix();
    }
}

/// Distant stand-in: a single cube in the toy's color. Full per-identity
/// detail (and spawn-pose tumble) only returns near the player.
pub fn draw_toy_lod_3d(center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(0.0, 0.16, 0.0) * scale,
        vec3(0.40, 0.34, 0.36) * scale,
        None,
        color,
    );
}

pub fn draw_toy_3d(toy: &ToyState, center: Vec3, color: Color, scale: f32) {
    if let Some(part) = toy.repair_part_kind() {
        draw_repair_part_3d(part, center, color, scale);
        return;
    }

    let profile = toy_profile(toy.category, toy.slot_number);
    match profile.identity {
        ToyIdentity::Bear => bear::draw(center, color, scale),
        ToyIdentity::Duck => duck::draw(center, color, scale),
        ToyIdentity::Rabbit => rabbit::draw(center, color, scale),
        ToyIdentity::Cat => cat::draw(center, color, scale),
        ToyIdentity::Puppy => puppy::draw(center, color, scale),
        ToyIdentity::Elephant => elephant::draw(center, color, scale),
        ToyIdentity::Owl => owl::draw(center, color, scale),
        ToyIdentity::Turtle => turtle::draw(center, color, scale),
        ToyIdentity::Penguin => penguin::draw(center, color, scale),
        ToyIdentity::Octopus => octopus::draw(center, color, scale),
        ToyIdentity::CrescentDragon => crescent_dragon::draw(center, color, scale),
        ToyIdentity::HornedDragon => horned_dragon::draw(center, color, scale),
        ToyIdentity::FinDragon => fin_dragon::draw(center, color, scale),
        ToyIdentity::SpikeDragon => spike_dragon::draw(center, color, scale),
        ToyIdentity::LongtailDragon => longtail_dragon::draw(center, color, scale),
        ToyIdentity::WyrmDragon => wyrm_dragon::draw(center, color, scale),
        ToyIdentity::PudgyDragon => pudgy_dragon::draw(center, color, scale),
        ToyIdentity::TwinDragon => twin_dragon::draw(center, color, scale),
        ToyIdentity::HatchlingDragon => hatchling_dragon::draw(center, color, scale),
        ToyIdentity::CurledDragon => curled_dragon::draw(center, color, scale),
        ToyIdentity::AntennaBot => antenna_bot::draw(center, color, scale),
        ToyIdentity::ClawBot => claw_bot::draw(center, color, scale),
        ToyIdentity::TreadBot => tread_bot::draw(center, color, scale),
        ToyIdentity::ScreenBot => screen_bot::draw(center, color, scale),
        ToyIdentity::TripodBot => tripod_bot::draw(center, color, scale),
        ToyIdentity::MazeBox => maze_box::draw(center, color, scale),
        ToyIdentity::CastleQuest => castle_quest::draw(center, color, scale),
        ToyIdentity::PlanetRace => planet_race::draw(center, color, scale),
        ToyIdentity::WordTiles => word_tiles::draw(center, color, scale),
        ToyIdentity::TreasureMap => treasure_map::draw(center, color, scale),
        ToyIdentity::TowerBlocks => tower_blocks::draw(center, color, scale),
        ToyIdentity::ArchBlocks => arch_blocks::draw(center, color, scale),
        ToyIdentity::BridgeBlocks => bridge_blocks::draw(center, color, scale),
        ToyIdentity::CastleBlocks => castle_blocks::draw(center, color, scale),
        ToyIdentity::TrainBlocks => train_blocks::draw(center, color, scale),
    }
}

fn draw_repair_part_3d(part: RepairPartKind, center: Vec3, color: Color, scale: f32) {
    match part {
        RepairPartKind::Head => {
            draw_cube(
                center + vec3(0.0, 0.18, 0.0) * scale,
                vec3(0.36, 0.28, 0.32) * scale,
                None,
                library::brighten(color, 0.08),
            );
            library::draw_eye_pair(center, 0.23, -0.18, 0.08, scale);
            draw_cube(
                center + vec3(0.0, 0.36, 0.0) * scale,
                vec3(0.055, 0.18, 0.055) * scale,
                None,
                library::darken(color, 0.18),
            );
            draw_toy_sphere(
                center + vec3(0.0, 0.48, 0.0) * scale,
                0.055 * scale,
                None,
                Color::new(0.94, 0.76, 0.28, 1.0),
            );
        }
        RepairPartKind::Body => {
            draw_cube(
                center + vec3(0.0, 0.12, 0.0) * scale,
                vec3(0.42, 0.36, 0.30) * scale,
                None,
                color,
            );
            draw_cube(
                center + vec3(-0.32, 0.12, 0.0) * scale,
                vec3(0.12, 0.28, 0.12) * scale,
                None,
                library::darken(color, 0.12),
            );
            draw_cube(
                center + vec3(0.32, 0.12, 0.0) * scale,
                vec3(0.12, 0.28, 0.12) * scale,
                None,
                library::darken(color, 0.12),
            );
            draw_cube_wires(
                center + vec3(0.0, 0.42, 0.0) * scale,
                vec3(0.28, 0.12, 0.22) * scale,
                Color::new(0.98, 0.78, 0.34, 0.92),
            );
        }
    }
}
