// Simple repeditive actions
use crate::actions::Action;
use screeps::Creep;

pub trait Actionable {
    fn load(creep: &Creep) -> Self;
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
