//! High-level game loop, state transitions, and toolkit integration.

use crate::data::GameData;
use crate::state::{
    migrate_save_value, GameSession, InteractionResult, SaveData, ToolPurchaseResult,
};
use crate::ui::{self, DebugOverlay, UiAction, UiContext};
use macroquad::miniquad::window::quit;
use macroquad::prelude::*;
use macroquad_toolkit::assets::{load_texture_from_pack_or_file, AssetPack};
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    load_from_slot_with_migration, save_to_slot_with_version, slot_exists,
};
use macroquad_toolkit::prelude::dark;

const TITLE_TEXTURE_PATH: &str = "assets/toybox_title.png";
const ASSET_PACK_PATH: &str = "assets.zip";

// Vertical field of view. 85 matches the look the game shipped with.
const DEFAULT_FOV_DEGREES: f32 = 85.0;
const MIN_FOV_DEGREES: f32 = 60.0;
const MAX_FOV_DEGREES: f32 = 110.0;
const FOV_STEP_DEGREES: f32 = 5.0;

pub struct Game {
    data: GameData,
    session: GameSession,
    title_texture: Option<Texture2D>,
    notifications: NotificationManager,
    events: EventBus<UiAction>,
    screen: GameScreen,
    has_save_file: bool,
    fullscreen_enabled: bool,
    fov_degrees: f32,
    settings_from_game: bool,
    mouse_locked: bool,
    debug_overlay: DebugOverlay,
    bench: Option<BenchMode>,
}

/// Headless-ish perf probe: set TOYBOX_BENCH_SECONDS=<n> to boot straight
/// into a fresh run, sweep the view for n seconds, print frame stats to
/// stdout, and exit. Used by the roadmap perf gates.
struct BenchMode {
    duration_seconds: f32,
    elapsed_seconds: f32,
    frames: u32,
    worst_frame_seconds: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameScreen {
    Title,
    Settings,
    ToolShop,
    Playing,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().unwrap_or_else(|err| {
            panic!("Toybox embedded data failed to load: {}", err);
        });
        let _loaded_assets = data.texture_manifest.len();

        let asset_pack = AssetPack::load(ASSET_PACK_PATH).await.ok();
        let title_texture = match load_texture_from_pack_or_file(
            asset_pack.as_ref(),
            TITLE_TEXTURE_PATH,
            FilterMode::Linear,
        )
        .await
        {
            Ok(texture) => Some(texture),
            Err(err) => {
                eprintln!("Failed to load title texture '{TITLE_TEXTURE_PATH}': {err}");
                None
            }
        };
        let has_save_file = slot_exists(&data.config.game_name, &data.config.save_slot);
        let notifications = NotificationManager::new();

        let bench = std::env::var("TOYBOX_BENCH_SECONDS")
            .ok()
            .and_then(|raw| raw.parse::<f32>().ok())
            .filter(|seconds| *seconds > 0.0)
            .map(|duration_seconds| BenchMode {
                duration_seconds,
                elapsed_seconds: 0.0,
                frames: 0,
                worst_frame_seconds: 0.0,
            });
        let screen = if bench.is_some() {
            GameScreen::Playing
        } else {
            GameScreen::Title
        };

        let session = GameSession::new(&data);
        Self {
            data,
            session,
            title_texture,
            notifications,
            events: EventBus::new(),
            screen,
            has_save_file,
            fullscreen_enabled: false,
            fov_degrees: DEFAULT_FOV_DEGREES,
            settings_from_game: false,
            mouse_locked: false,
            debug_overlay: DebugOverlay::new(),
            bench,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.notifications.update(dt);
        self.debug_overlay.record_frame(dt);
        if self.data.config.debug_overlay_enabled && is_key_pressed(KeyCode::F3) {
            self.debug_overlay.toggle();
        }
        self.update_bench(dt);

        if self.screen != GameScreen::Playing {
            if self.screen == GameScreen::Settings && is_key_pressed(KeyCode::Escape) {
                self.events.push(UiAction::CloseSettings);
            } else if self.screen == GameScreen::ToolShop
                && (is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::T))
            {
                self.events.push(UiAction::CloseToolShop);
            }
            self.apply_queued_actions();
            return;
        }

        self.session.update_timer(dt);

        let current_mouse_position: Vec2 = mouse_position().into();
        if is_mouse_button_pressed(MouseButton::Left)
            && ui::should_lock_mouse_from_screen_position(current_mouse_position)
        {
            self.set_mouse_locked(true);
        }
        if is_key_pressed(KeyCode::Tab) {
            self.set_mouse_locked(!self.mouse_locked);
        }

        let mouse_delta = ui::continuous_mouse_delta_pixels();
        let look_delta = ui::look_delta_from_input(mouse_delta, self.mouse_locked, dt);
        self.session.update_player_look(look_delta.x, look_delta.y);

        let movement = ui::movement_from_keys();
        self.session.move_player(movement, &self.data, dt);

        if is_key_pressed(KeyCode::Escape) {
            self.events.push(UiAction::Settings);
        }
        if is_key_pressed(KeyCode::R) {
            self.events.push(UiAction::NewGame);
        }
        if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Space) {
            self.events.push(UiAction::Interact);
        }
        if is_key_pressed(KeyCode::Q) {
            self.events.push(UiAction::CycleCarry);
        }
        if is_key_pressed(KeyCode::G) {
            self.events.push(UiAction::DropActive);
        }
        if is_key_pressed(KeyCode::T) {
            self.events.push(UiAction::OpenToolShop);
        }
        if is_key_pressed(KeyCode::S) && is_control_down() {
            self.events.push(UiAction::Save);
        }
        if is_key_pressed(KeyCode::L) && is_control_down() {
            self.events.push(UiAction::Load);
        }

        self.apply_queued_actions();
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);

        ui::begin_ui_frame();
        let actions = match self.screen {
            GameScreen::Title => {
                ui::draw_title_screen(self.title_texture.as_ref(), self.has_save_file)
            }
            GameScreen::Settings => ui::draw_settings_screen(
                self.title_texture.as_ref(),
                self.fullscreen_enabled,
                self.fov_degrees,
                self.settings_from_game,
            ),
            GameScreen::ToolShop => {
                let ctx = UiContext {
                    data: &self.data,
                    session: &self.session,
                    mouse_locked: self.mouse_locked,
                    fov_degrees: self.fov_degrees,
                };
                ui::draw_tool_shop_screen(ctx)
            }
            GameScreen::Playing => {
                let ctx = UiContext {
                    data: &self.data,
                    session: &self.session,
                    mouse_locked: self.mouse_locked,
                    fov_degrees: self.fov_degrees,
                };
                ui::draw_game_ui(ctx, &self.debug_overlay)
            }
        };
        ui::end_ui_frame();

        for action in actions {
            self.events.push(action);
        }

        self.notifications
            .draw_with_config(&NotificationRenderConfig {
                anchor: NotificationAnchor::TopRight,
                ..Default::default()
            });
    }

    fn update_bench(&mut self, dt: f32) {
        let Some(bench) = &mut self.bench else {
            return;
        };
        bench.frames += 1;
        bench.elapsed_seconds += dt;
        bench.worst_frame_seconds = bench.worst_frame_seconds.max(dt);
        // Slow sweep so the run exercises culling across view directions.
        self.session.update_player_look(0.55 * dt, 0.0);

        if bench.elapsed_seconds >= bench.duration_seconds {
            let average_fps = bench.frames as f32 / bench.elapsed_seconds.max(f32::EPSILON);
            println!(
                "BENCH toys={} frames={} seconds={:.2} avg_fps={:.1} worst_frame_ms={:.2}",
                self.session.toys.len(),
                bench.frames,
                bench.elapsed_seconds,
                average_fps,
                bench.worst_frame_seconds * 1000.0
            );
            quit();
        }
    }

    fn apply_queued_actions(&mut self) {
        let actions: Vec<UiAction> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
    }

    fn apply_action(&mut self, action: UiAction) {
        match action {
            UiAction::NewGame => {
                self.session = GameSession::new(&self.data);
                self.screen = GameScreen::Playing;
                self.notifications.info("Fresh cleanup started");
            }
            UiAction::Continue => {
                if self.load_game() {
                    self.screen = GameScreen::Playing;
                }
            }
            UiAction::Settings => {
                self.settings_from_game = self.screen == GameScreen::Playing;
                self.set_mouse_locked(false);
                self.screen = GameScreen::Settings;
            }
            UiAction::CloseSettings => {
                self.screen = if self.settings_from_game {
                    GameScreen::Playing
                } else {
                    GameScreen::Title
                };
                self.settings_from_game = false;
            }
            UiAction::BackToTitle => {
                self.settings_from_game = false;
                self.set_mouse_locked(false);
                self.screen = GameScreen::Title;
                self.has_save_file =
                    slot_exists(&self.data.config.game_name, &self.data.config.save_slot);
            }
            UiAction::OpenToolShop => {
                self.set_mouse_locked(false);
                self.screen = GameScreen::ToolShop;
            }
            UiAction::CloseToolShop => {
                self.screen = GameScreen::Playing;
            }
            UiAction::ToggleFullscreen => {
                self.fullscreen_enabled = !self.fullscreen_enabled;
                set_fullscreen(self.fullscreen_enabled);
            }
            UiAction::FovIncrease => {
                self.fov_degrees =
                    (self.fov_degrees + FOV_STEP_DEGREES).clamp(MIN_FOV_DEGREES, MAX_FOV_DEGREES);
            }
            UiAction::FovDecrease => {
                self.fov_degrees =
                    (self.fov_degrees - FOV_STEP_DEGREES).clamp(MIN_FOV_DEGREES, MAX_FOV_DEGREES);
            }
            UiAction::QuitGame => {
                self.set_mouse_locked(false);
                quit();
            }
            UiAction::Save => self.save_game(),
            UiAction::Load => {
                self.load_game();
            }
            UiAction::Interact => self.handle_interaction(),
            UiAction::CycleCarry => self.session.cycle_carried(),
            UiAction::DropActive => {
                if let Some(toy_name) = self.session.drop_active(&self.data) {
                    self.notifications
                        .info(format!("Placed {} on the floor", toy_name));
                }
            }
            UiAction::BuyTool(tool_id) => self.handle_tool_purchase(&tool_id),
        }
    }

    fn handle_interaction(&mut self) {
        match self.session.interact(&self.data) {
            InteractionResult::PickedUp { toy_name } => {
                self.notifications.info(format!("Picked up {}", toy_name));
            }
            InteractionResult::Dropped { toy_name } => {
                self.notifications
                    .info(format!("Placed {} on the floor", toy_name));
            }
            InteractionResult::Placed {
                toy_name,
                display_name,
                was_wrong,
                completed_display,
                available_tools,
                finished,
            } => {
                if was_wrong {
                    self.notifications
                        .warning(format!("{} does not belong in {}", toy_name, display_name));
                } else {
                    self.notifications
                        .success(format!("Placed {} in {}", toy_name, display_name));
                }
                if let Some(name) = completed_display {
                    self.notifications.success(format!("Completed {}", name));
                }
                for tool_name in available_tools {
                    self.notifications
                        .info(format!("Tool available: {} (press T)", tool_name));
                }
                if finished {
                    self.notifications.success("Store restored before opening");
                }
            }
            InteractionResult::PlacedOnRepairBench { toy_name } => {
                self.notifications
                    .info(format!("Placed {} on the repair bench", toy_name));
            }
            InteractionResult::Repaired { toy_name } => {
                self.notifications
                    .success(format!("Repaired {}. Ready for display", toy_name));
            }
            InteractionResult::NeedsRepair { toy_name } => {
                self.notifications
                    .warning(format!("Repair {} before shelving it", toy_name));
            }
            InteractionResult::NeedsRepairParts { toy_name } => {
                self.notifications
                    .warning(format!("Find the matching part for {}", toy_name));
            }
            InteractionResult::InventoryFull => self.notifications.warning("Sorting cart is full"),
            InteractionResult::RepairBenchFull => {
                self.notifications.warning("Repair bench is full")
            }
            InteractionResult::RepairMismatch => self
                .notifications
                .warning("Those repair parts do not match"),
            InteractionResult::ShelfFull => self.notifications.warning("That shelf is full"),
            InteractionResult::ShelfSlotUnavailable => {
                self.notifications.warning("Look at an empty shelf spot")
            }
            InteractionResult::NothingNearby => {
                self.notifications.info("Move closer to a toy or display");
            }
        }
    }

    fn handle_tool_purchase(&mut self, tool_id: &str) {
        match self.session.purchase_tool(&self.data, tool_id) {
            ToolPurchaseResult::Purchased {
                tool_name,
                remaining_credits,
            } => self.notifications.success(format!(
                "Purchased {}. Tool credits left: {}",
                tool_name, remaining_credits
            )),
            ToolPurchaseResult::NeedMoreCredits {
                tool_name,
                cost,
                available_credits,
            } => self.notifications.warning(format!(
                "{} needs {} tool credit(s). You have {}",
                tool_name, cost, available_credits
            )),
            ToolPurchaseResult::AlreadyOwned { tool_name } => self
                .notifications
                .info(format!("{} already owned", tool_name)),
            ToolPurchaseResult::Locked {
                tool_name,
                required_displays,
                completed_displays,
            } => self.notifications.warning(format!(
                "{} unlocks at {}/{} restored displays",
                tool_name, completed_displays, required_displays
            )),
            ToolPurchaseResult::NoToolsAvailable => {
                self.notifications.info("No tools available to buy")
            }
        }
    }

    fn save_game(&mut self) {
        let save = self.session.to_save(&self.data.config.version);
        match save_to_slot_with_version(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &save,
            &self.data.config.version,
        ) {
            Ok(()) => {
                self.has_save_file = true;
                self.notifications.success("Cleanup saved");
            }
            Err(err) => self.notifications.danger(format!("Save failed: {}", err)),
        }
    }

    fn load_game(&mut self) -> bool {
        let loaded: Result<SaveData, String> = load_from_slot_with_migration(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &self.data.config.version,
            |version, value| migrate_save_value(version, value, &self.data),
        );

        match loaded {
            Ok(save) => {
                self.session = GameSession::from_save(save, &self.data);
                self.has_save_file = true;
                self.notifications.success("Loaded cleanup save");
                true
            }
            Err(err) => {
                self.has_save_file =
                    slot_exists(&self.data.config.game_name, &self.data.config.save_slot);
                self.notifications.warning(format!("Load failed: {}", err));
                false
            }
        }
    }

    fn set_mouse_locked(&mut self, locked: bool) {
        self.mouse_locked = locked;
        set_cursor_grab(locked);
        show_mouse(!locked);
    }
}

fn is_control_down() -> bool {
    is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)
}
