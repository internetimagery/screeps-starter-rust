// Simple repeditive actions
use super::ACTION;
use crate::actions::Action;
use screeps::Creep;
use std::convert::{From, TryFrom};

pub trait Actionable {
    fn save(&self, _: &Creep) {}
    fn execute(&self, creep: &Creep) -> bool;
}

// Helper methods exposed on the creep
pub trait CreepActions {
    // Run an action if one exists. Return true if the action needs more turns to complete
    fn execute_action(&self) -> bool;
    // Set an action to run (next turn). Action may run over more than one turn till completion
    fn set_action(&self, action: Action);
}

impl CreepActions for Creep {
    fn execute_action(&self) -> bool {
        if let Ok(action) = Action::try_from(self) {
            if action.execute(self) {
                return true;
            }
            self.memory().set(ACTION, 0);
        }
        false
    }
    fn set_action(&self, action: Action) {
        self.memory().set(ACTION, String::from(&action));
        action.save(self);
    }
}
