use super::{Action, Actionable, TARGET};
use log::*;
use screeps::{prelude::*, ConstructionSite, Creep, ReturnCode, Structure};

pub struct BuildSite {
    target: Option<ConstructionSite>,
}

impl Action {
    pub fn build_site(target: ConstructionSite) -> Action {
        Action::BuildSite(BuildSite {
            target: Some(target),
        })
    }
}

impl From<&Creep> for BuildSite {
    fn from(creep: &Creep) -> Self {
        Self {
            target: get_id!(creep, TARGET),
        }
    }
}

impl Actionable for BuildSite {
    fn save(&self, creep: &Creep) {
        if let Some(target) = &self.target {
            set_id!(creep, TARGET, target);
        }
    }
    fn execute(&self, creep: &Creep) -> bool {
        if let Some(target) = &self.target {
            match creep.build(target) {
                ReturnCode::Ok => return true,
                ReturnCode::NotEnough => return false,
                ReturnCode::NotInRange => {
                    creep.move_to(target);
                    return true;
                }
                x => warn!("Failed to build {:?}", x),
            }
        }
        false
    }
}

pub struct RepairStructure {
    target: Option<Structure>,
}

impl Action {
    pub fn repair_structure(target: Structure) -> Action {
        Action::RepairStructure(RepairStructure {
            target: Some(target),
        })
    }
}

impl From<&Creep> for RepairStructure {
    fn from(creep: &Creep) -> Self {
        Self {
            target: get_id!(creep, TARGET),
        }
    }
}

impl Actionable for RepairStructure {
    fn save(&self, creep: &Creep) {
        if let Some(target) = &self.target {
            set_id!(creep, TARGET, target);
        }
    }
    fn execute(&self, creep: &Creep) -> bool {
        if let Some(target) = &self.target {
            match creep.repair(target) {
                ReturnCode::Ok | ReturnCode::NotEnough => return false,
                ReturnCode::NotInRange => {
                    creep.move_to(target);
                    return true;
                }
                x => warn!("Failed to repair {:?}", x),
            }
        }
        false
    }
}
