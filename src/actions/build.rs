use log::*;
use screeps::{prelude::*, ConstructionSite, Creep, ReturnCode, Structure};

// Build up a construction site
action_target! {
    fn build_site(target: ConstructionSite) -> BuildSite;
    fn execute(&self, creep: &Creep) -> bool {
        if let Some(target) = &self.target {
            match creep.build(target) {
                ReturnCode::Ok => return true,
                ReturnCode::NotEnough => return false,
                ReturnCode::NotInRange => {
                    creep.move_to(target);
                    return true
                }
                x => warn!("Failed to build {:?}", x),
            }
        }
        false
    }
}

// Repair something. Yay!
action_target! {
    fn repair_structure(target: Structure) -> RepairStructure;
    fn execute(&self, creep: &Creep) -> bool {
        if let Some(target) = &self.target {
            match creep.repair(target) {
                ReturnCode::Ok | ReturnCode::NotEnough => return false,
                ReturnCode::NotInRange => {
                    creep.move_to(target);
                    return true
                }
                x => warn!("Failed to repair {:?}", x),
            }
        }
        false
    }
}
