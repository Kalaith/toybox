//! Procedural toy library and per-identity renderers.

use crate::state::ToyState;
use library::ToyIdentity;
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
mod duck;
mod fin_dragon;
mod horned_dragon;
pub mod library;
mod longtail_dragon;
mod maze_box;
mod planet_race;
mod puppy;
mod rabbit;
mod screen_bot;
mod spike_dragon;
mod tower_blocks;
mod train_blocks;
mod tread_bot;
mod treasure_map;
mod tripod_bot;
mod word_tiles;

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

pub fn draw_toy_3d(toy: &ToyState, center: Vec3, color: Color, scale: f32) {
    let profile = toy_profile(toy.category, toy.slot_number);
    match profile.identity {
        ToyIdentity::Bear => bear::draw(center, color, scale),
        ToyIdentity::Duck => duck::draw(center, color, scale),
        ToyIdentity::Rabbit => rabbit::draw(center, color, scale),
        ToyIdentity::Cat => cat::draw(center, color, scale),
        ToyIdentity::Puppy => puppy::draw(center, color, scale),
        ToyIdentity::CrescentDragon => crescent_dragon::draw(center, color, scale),
        ToyIdentity::HornedDragon => horned_dragon::draw(center, color, scale),
        ToyIdentity::FinDragon => fin_dragon::draw(center, color, scale),
        ToyIdentity::SpikeDragon => spike_dragon::draw(center, color, scale),
        ToyIdentity::LongtailDragon => longtail_dragon::draw(center, color, scale),
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
    library::draw_tag(center, scale);
}
