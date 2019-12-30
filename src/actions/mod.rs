// Simple repeditive actions

use screeps::Creep;

mod transport;

const ACTION: &'static str = "action";

#[derive(Copy, Clone)]
pub enum Actions {
    None = 0,
    HarvestEnergy = 1,
}

pub trait CreepActions {
    // Run an action if one exists. Return true if the action needs more turns to complete
    fn execute_action(&self) -> bool;
    // Set an action to run (next turn). Action may run over more than one turn till completion
    fn set_action(&self, action: Actions);
}

impl CreepActions for Creep {
    fn execute_action(&self) -> bool {
        if let Ok(Some(action_num)) = self.memory().i32(ACTION) {
            let result = match action_num {
                0 => false, // None
                1 => transport::harvest_energy(&self),
                _ => false,
            };
            if !result {
                self.set_action(Actions::None);
            }
            return result;
        }
        false
    }
    fn set_action(&self, action: Actions) {
        self.memory().set(ACTION, action as i32);
    }
}
