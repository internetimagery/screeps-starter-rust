// Simple repeditive actions

use screeps::Creep;
use std::convert::{From, TryFrom};

#[macro_use]
mod macros;
pub mod prelude;
pub mod transport;

const ACTION: &'static str = "action";

// Match actions with their associated logic and serialized IDs
register_actions! {
    HarvestEnergy(transport::HarvestEnergy) = 1,
}

use prelude::*;

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
        self.memory().set(ACTION, i32::from(&action));
        action.save(self);
    }
}
