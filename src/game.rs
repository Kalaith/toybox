//! High-level game loop, state transitions, and toolkit integration.

use crate::data::GameData;
use crate::state::{migrate_save_value, GameSession, InteractionResult, SaveData};
use crate::ui::{self, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    load_from_slot_with_migration, save_to_slot_with_version,
};
use macroquad_toolkit::prelude::dark;

pub struct Game {
    data: GameData,
    session: GameSession,
    notifications: NotificationManager,
    events: EventBus<UiAction>,
    mouse_locked: bool,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().unwrap_or_else(|err| {
            panic!("Toybox embedded data failed to load: {}", err);
        });

        let notifications = NotificationManager::new();

        let session = GameSession::new(&data);
        Self {
            data,
            session,
            notifications,
            events: EventBus::new(),
            mouse_locked: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.notifications.update(dt);
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
        self.session.move_player(movement, &self.data.config, dt);

        if is_key_pressed(KeyCode::Escape) {
            if self.mouse_locked {
                self.set_mouse_locked(false);
            }
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
        if is_key_pressed(KeyCode::S) && is_control_down() {
            self.events.push(UiAction::Save);
        }
        if is_key_pressed(KeyCode::L) && is_control_down() {
            self.events.push(UiAction::Load);
        }

        let actions: Vec<UiAction> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);

        ui::begin_ui_frame();
        let ctx = UiContext {
            data: &self.data,
            session: &self.session,
            mouse_locked: self.mouse_locked,
        };

        let actions = ui::draw_game_ui(ctx);
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

    fn apply_action(&mut self, action: UiAction) {
        match action {
            UiAction::NewGame => {
                self.session = GameSession::new(&self.data);
                self.notifications.info("Fresh cleanup started");
            }
            UiAction::Save => self.save_game(),
            UiAction::Load => self.load_game(),
            UiAction::Interact => self.handle_interaction(),
            UiAction::CycleCarry => self.session.cycle_carried(),
            UiAction::DropActive => {
                if let Some(toy_name) = self.session.drop_active() {
                    self.notifications.info(format!("Dropped {}", toy_name));
                }
            }
        }
    }

    fn handle_interaction(&mut self) {
        match self.session.interact(&self.data) {
            InteractionResult::PickedUp { toy_name } => {
                self.notifications.info(format!("Picked up {}", toy_name));
            }
            InteractionResult::Placed {
                toy_name,
                display_name,
                completed_display,
                unlocked_upgrades,
                finished,
            } => {
                self.notifications
                    .success(format!("Placed {} in {}", toy_name, display_name));
                if let Some(name) = completed_display {
                    self.notifications.success(format!("Completed {}", name));
                }
                for upgrade_name in unlocked_upgrades {
                    self.notifications
                        .success(format!("Unlocked {}", upgrade_name));
                }
                if finished {
                    self.notifications.success("Store restored before opening");
                }
            }
            InteractionResult::InventoryFull => self.notifications.warning("Sorting cart is full"),
            InteractionResult::NothingNearby => {
                self.notifications.info("Move closer to a toy or display");
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
                self.notifications.success("Cleanup saved");
            }
            Err(err) => self.notifications.danger(format!("Save failed: {}", err)),
        }
    }

    fn load_game(&mut self) {
        let loaded: Result<SaveData, String> = load_from_slot_with_migration(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &self.data.config.version,
            |version, value| migrate_save_value(version, value, &self.data),
        );

        match loaded {
            Ok(save) => {
                self.session = GameSession::from_save(save, &self.data);
                self.notifications.success("Loaded cleanup save");
            }
            Err(err) => self.notifications.warning(format!("Load failed: {}", err)),
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
