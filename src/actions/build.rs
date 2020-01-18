use super::{Action, ActionExecute, ActionProvider};
use log::*;
use screeps::{prelude::*, ConstructionSite, Creep, ReturnCode, Structure};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BuildSite {
    target: String,
}
#[derive(Serialize, Deserialize)]
pub struct RepairStructure {
    target: String,
}

impl ActionProvider<'_, Creep> {
    pub fn build_site(&self, target: &ConstructionSite) {
        self.set_action(Action::BuildSite(BuildSite {
            target: to_id!(target),
        }))
    }
    pub fn repair_structure(&self, target: &Structure) {
        self.set_action(Action::RepairStructure(RepairStructure {
            target: to_id!(target),
        }))
    }
}

// Build up a construction site
impl ActionExecute<Creep> for BuildSite {
    fn execute(&self, creep: &Creep) -> bool {
        let target: Option<ConstructionSite> = from_id!(&self.target);
        if let Some(target) = target {
            match creep.build(&target) {
                ReturnCode::Ok | ReturnCode::Busy => return true,
                ReturnCode::NotEnough => return false,
                ReturnCode::NotInRange => {
                    creep.move_to(&target);
                    return true;
                }
                x => warn!("Failed to build {:?}", x),
            }
        }
        false
    }
}

// Repair something. Yay!
impl ActionExecute<Creep> for RepairStructure {
    fn execute(&self, creep: &Creep) -> bool {
        let target: Option<Structure> = from_id!(&self.target);
        if let Some(target) = target {
            match creep.repair(&target) {
                ReturnCode::Busy => return true,
                ReturnCode::Ok | ReturnCode::NotEnough => return false,
                ReturnCode::NotInRange => {
                    creep.move_to(&target);
                    return true;
                }
                x => warn!("Failed to repair {:?}", x),
            }
        }
        false
    }
}
