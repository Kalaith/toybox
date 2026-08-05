//! Contextual first-shift guidance. It teaches one action at a time and waits
//! for repair and trolley advice until those mechanics are actually relevant.

use crate::data::GameData;
use crate::state::{GameSession, InteractionResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TutorialStep {
    Navigate,
    PickUp,
    Shelve,
    Repair,
    Tools,
    Trolley,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TutorialHint {
    pub step: TutorialStep,
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub keys: &'static [&'static str],
}

#[derive(Debug, Clone, Default)]
pub struct TutorialProgress {
    active: bool,
    moved: bool,
    looked: bool,
    picked_up: bool,
    shelved_correctly: bool,
    repaired: bool,
    opened_tools: bool,
    cycled_trolley: bool,
}

impl TutorialProgress {
    pub fn new(active: bool) -> Self {
        Self {
            active,
            ..Default::default()
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn observe_navigation(&mut self, moved: bool, looked: bool) {
        if !self.active {
            return;
        }
        self.moved |= moved;
        self.looked |= looked;
    }

    pub fn observe_interaction(&mut self, result: &InteractionResult) {
        if !self.active {
            return;
        }
        match result {
            InteractionResult::PickedUp { .. } => self.picked_up = true,
            InteractionResult::Placed {
                was_wrong: false, ..
            } => self.shelved_correctly = true,
            InteractionResult::Repaired { .. } => self.repaired = true,
            _ => {}
        }
    }

    pub fn opened_tools(&mut self) {
        if self.active {
            self.opened_tools = true;
        }
    }

    pub fn cycled_trolley(&mut self, had_multiple_toys: bool) {
        if self.active && had_multiple_toys {
            self.cycled_trolley = true;
        }
    }

    pub fn skip(&mut self) {
        self.active = false;
    }

    pub fn is_complete(&self) -> bool {
        self.moved
            && self.looked
            && self.picked_up
            && self.shelved_correctly
            && self.repaired
            && self.opened_tools
            && self.cycled_trolley
    }

    pub fn hint(&self, session: &GameSession, data: &GameData) -> Option<TutorialHint> {
        if !self.active || self.is_complete() {
            return None;
        }
        if !self.moved || !self.looked {
            return Some(TutorialHint {
                step: TutorialStep::Navigate,
                eyebrow: "FIRST SHIFT · 1/6",
                title: "LOOK AROUND THE SHOP",
                body: "Use the mouse to look and WASD to walk. Click the shop to lock mouse look.",
                keys: &["WASD", "MOUSE"],
            });
        }
        if !self.picked_up {
            return Some(TutorialHint {
                step: TutorialStep::PickUp,
                eyebrow: "FIRST SHIFT · 2/6",
                title: "PICK UP A TOY",
                body: "Aim at a loose toy. The prompt below always shows what your next action will do.",
                keys: &["E"],
            });
        }
        if !self.shelved_correctly {
            return Some(TutorialHint {
                step: TutorialStep::Shelve,
                eyebrow: "FIRST SHIFT · 3/6",
                title: "MATCH THE CATEGORY",
                body: "Carry the toy to a display with the same category, then aim at an empty shelf spot.",
                keys: &["E"],
            });
        }

        let carrying_part = session.active_toy().is_some_and(|toy| toy.is_repair_part());
        if !self.repaired && carrying_part {
            return Some(TutorialHint {
                step: TutorialStep::Repair,
                eyebrow: "FIRST SHIFT · 4/6",
                title: "MEND A MATCHED PAIR",
                body: "Find this toy's other half, place both on one repair bench, then repair it.",
                keys: &["E"],
            });
        }
        if !self.opened_tools && session.next_available_upgrade(data).is_some() {
            return Some(TutorialHint {
                step: TutorialStep::Tools,
                eyebrow: "FIRST SHIFT · 5/6",
                title: "SPEND A DISPLAY CREDIT",
                body: "Each restored display earns one credit. Open the tool rack to improve this shift.",
                keys: &["T"],
            });
        }
        if !self.cycled_trolley
            && session.has_upgrade("sorting_trolley")
            && session.player.carried_toy_ids.len() > 1
        {
            return Some(TutorialHint {
                step: TutorialStep::Trolley,
                eyebrow: "FIRST SHIFT · 6/6",
                title: "CHOOSE THE ACTIVE TOY",
                body: "The bright trolley token is in your hands. Cycle it before shelving or dropping.",
                keys: &["Q"],
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guidance_advances_from_navigation_to_the_sorting_loop() {
        let data = GameData::load().unwrap();
        let session = GameSession::new(&data);
        let mut tutorial = TutorialProgress::new(true);
        assert_eq!(
            tutorial.hint(&session, &data).unwrap().step,
            TutorialStep::Navigate
        );

        tutorial.observe_navigation(true, true);
        assert_eq!(
            tutorial.hint(&session, &data).unwrap().step,
            TutorialStep::PickUp
        );
        tutorial.observe_interaction(&InteractionResult::PickedUp {
            toy_name: "Test Toy".to_owned(),
        });
        assert_eq!(
            tutorial.hint(&session, &data).unwrap().step,
            TutorialStep::Shelve
        );
    }

    #[test]
    fn contextual_lessons_wait_until_their_mechanic_is_relevant() {
        let data = GameData::load().unwrap();
        let session = GameSession::new(&data);
        let mut tutorial = TutorialProgress::new(true);
        tutorial.moved = true;
        tutorial.looked = true;
        tutorial.picked_up = true;
        tutorial.shelved_correctly = true;

        assert!(tutorial.hint(&session, &data).is_none());
    }

    #[test]
    fn every_taught_action_closes_the_guide() {
        let mut tutorial = TutorialProgress::new(true);
        tutorial.observe_navigation(true, true);
        tutorial.observe_interaction(&InteractionResult::PickedUp {
            toy_name: "Test Toy".to_owned(),
        });
        tutorial.observe_interaction(&InteractionResult::Placed {
            toy_name: "Test Toy".to_owned(),
            display_name: "Test Display".to_owned(),
            was_wrong: false,
            completed_display: None,
            completed_zone: None,
            available_tools: Vec::new(),
            finished: false,
        });
        tutorial.observe_interaction(&InteractionResult::Repaired {
            toy_name: "Test Toy".to_owned(),
        });
        tutorial.opened_tools();
        tutorial.cycled_trolley(true);

        assert!(tutorial.is_complete());
    }
}
