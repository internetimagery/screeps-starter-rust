// Simple repeditive actions

use screeps::Creep;


pub mod transport;

const ACTION: &'static str = "action";

pub trait Actionable {
    fn attach_action(creep: &Creep) {}
    fn execute(creep: &Creep) -> bool;
}

macro_rules! ActionIDs {
    ($($name: path = $value: expr,)+) => {
        fn get_action_id<T: Actionable>(action: T) -> i32 {
            match action {
                $($name => $value,)+
                _ => 0,
            }
        }
        fn get_action<T: Actionable>(id: i32) -> Option<T> {
            match id {
                $($value => $name,)+
                _ => None,
            }
        }
    }
}

// Match actions with their IDs
ActionIDs! {
    transport::HarvestEnergy = 1,
}

// Helper methods exposed on the creep
pub trait CreepActions {
    // Run an action if one exists. Return true if the action needs more turns to complete
    fn execute_action(&self) -> bool;
    // Set an action to run (next turn). Action may run over more than one turn till completion
    fn set_action<T: Actionable>(&self, action: T);
}

impl CreepActions for Creep {
    fn execute_action(&self) -> bool {
        if let Ok(Some(action_num)) = self.memory().i32(ACTION) {
            if let Some(action) = get_action(action_num) {
                if action::execute(&self) {
                    return true;
                } else {
                    self.memory().set(ACTION, 0);
                }
            }
        }
        false
    }
    fn set_action<T: Actionable>(&self, action: T) {
        self.memory().set(ACTION, get_action_id(action));
        action::attach_action(&self);
    }
}
