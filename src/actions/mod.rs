// Simple repeditive actions

use screeps::Creep;
use std::convert::{From, TryFrom};

pub mod transport;

const ACTION: &'static str = "action";

pub trait Actionable {
    fn load(creep: &Creep) -> Self;
    fn save(&self, _: &Creep) {}
    fn execute(&self, creep: &Creep) -> bool;
}

macro_rules! ActionIDs {
    ($($name: ident => $logic:ty = $value: expr,)+) => {
        pub enum Action {
            $($name($logic),)+
        }

        impl Action {
            fn save(&self, creep: &Creep) {
                match self {
                    $(Action::$name(x) => x.save(creep),)+
                }
            }
            fn execute(&self, creep: &Creep) -> bool {
                match self {
                    $(Action::$name(x) => x.execute(creep),)+
                }
            }
        }

        impl std::convert::From<&Action> for i32 {
            fn from(value: &Action) -> Self {
                match value {
                    $(Action::$name(_) => $value,)+
                }
            }
        }

        impl std::convert::TryFrom<&Creep> for Action {
            type Error = String;
            fn try_from(creep: &Creep) -> Result<Self, Self::Error> {
                if let Ok(Some(id)) = creep.memory().i32(ACTION) {
                    return match id {
                        $($value => Ok(Action::$name(<$logic>::load(creep))),)+
                        x => Err(format!("Unknown Action {}", x)),
                    }
                }
                Err("No action available".to_string())
            }
        }
    }
}

// Match actions with their associated logic and serialized IDs
ActionIDs! {
    HarvestEnergy => transport::HarvestEnergy = 1,
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
        self.memory().set(ACTION, i32::from(&action));
        action.save(self);
    }
}
