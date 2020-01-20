use super::{Action, ActionProvider, ActionResult};
use crate::exception::Res;
use screeps::{prelude::*, ConstructionSite, Creep, ReturnCode, Structure};

// Initializers /////////////////////////////////
impl ActionProvider<'_, Creep> {
    pub fn build_site(&self, site: &ConstructionSite) {
        self.set_action(Action::BuildSite { site: site.id() })
    }
    pub fn repair_structure(&self, target: &Structure) {
        self.set_action(Action::RepairStructure {
            target: target.id(),
        })
    }
}

// Functionality ///////////////////////////////

pub fn build_site(creep: &Creep, site: &ConstructionSite) -> Res<ActionResult> {
    match creep.build(site) {
        ReturnCode::Ok | ReturnCode::Busy => Ok(ActionResult::Continue),
        ReturnCode::NotEnough => Ok(ActionResult::Done),
        ReturnCode::NotInRange => {
            creep.move_to(site);
            Ok(ActionResult::Continue)
        }
        x => Err(format!("Failed to build {:?}", x))?,
    }
}

pub fn repair_structure(creep: &Creep, target: &Structure) -> Res<ActionResult> {
    match creep.repair(target) {
        ReturnCode::Busy => Ok(ActionResult::Continue),
        ReturnCode::Ok | ReturnCode::NotEnough => Ok(ActionResult::Done),
        ReturnCode::NotInRange => {
            creep.move_to(target);
            Ok(ActionResult::Continue)
        }
        x => Err(format!("Failed to repair {:?}", x))?,
    }
}
