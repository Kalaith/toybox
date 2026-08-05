//! Toybox After Hours entry point.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod capture_scenes;
mod data;
mod gallery;
mod game;
mod preferences;
mod state;
mod toys;
mod tutorial;
mod ui;

use game::Game;

fn window_conf() -> Conf {
    let mut conf = capture::capture_window_conf(
        "TOYBOX",
        "Toybox After Hours: Closing Shift",
        ui::LOGICAL_WIDTH as i32,
        ui::LOGICAL_HEIGHT as i32,
    );
    // A perf probe that cannot see past vsync measures the monitor, not the
    // game: every run came back at 59.5 fps however much work the frame did,
    // so a regression would only ever show up once the game was *already*
    // dropping frames. Uncapped for a bench run only — normal play stays
    // synced. `window_conf` is the earliest hook there is, which is why the
    // env var is read here rather than passed down from `Game`.
    if game::bench_seconds().is_some() {
        conf.platform.swap_interval = Some(0);
    }
    conf
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    if let Some(config) = capture::CaptureConfig::from_env("TOYBOX") {
        game.begin_capture_scene(&config.scene);
        capture::run_capture(&config, |dt| {
            game.update(dt);
            game.draw();
        })
        .await;
        return;
    }

    loop {
        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
