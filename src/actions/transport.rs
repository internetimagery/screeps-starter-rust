// Actions relating to transporting goods from one place to another. eg energy

use super::prelude::*;
use super::{Action, TARGET};
use crate::prelude::*;
use crate::{get_id, set_id};
use log::*;
use screeps::{game, prelude::*, Creep, ResourceType, ReturnCode, Source, Structure};
use std::convert::From;

pub struct HarvestEnergy {
    target: Option<Source>,
}

impl Action {
    pub fn harvest_energy(target: Source) -> Action {
        Action::HarvestEnergy(HarvestEnergy {
            target: Some(target),
        })
    }
}

impl From<&Creep> for HarvestEnergy {
    fn from(creep: &Creep) -> Self {
        Self {
            target: get_id!(creep, TARGET),
        }
    }
}

impl Actionable for HarvestEnergy {
    fn save(&self, creep: &Creep) {
        if let Some(target) = &self.target {
            set_id!(creep, TARGET, target);
        }
    }
    fn execute(&self, creep: &Creep) -> bool {
        if creep.is_full(ResourceType::Energy) {
            return false;
        }
        if let Some(target) = &self.target {
            match creep.harvest(target) {
                ReturnCode::Ok => {
                    if game::time() % 5 == 0 {
                        creep.say("⏳", true);
                    }
                }
                ReturnCode::NotInRange => {
                    creep.move_to(target);
                }
                x => warn!("Failed to harvest: {:?}", x),
            }
            return true; // Need more energy!
        }
        false
    }
}

// Store energy in spawn point or other structure
pub struct StoreEnergy {
    target: Option<Structure>,
}

impl Action {
    pub fn store_energy(target: Structure) -> Action {
        Action::StoreEnergy(StoreEnergy {
            target: Some(target),
        })
    }
}

impl From<&Creep> for StoreEnergy {
    fn from(creep: &Creep) -> Self {
        Self {
            target: get_id!(creep, TARGET),
        }
    }
}

impl Actionable for StoreEnergy {
    fn save(&self, creep: &Creep) {
        if let Some(target) = &self.target {
            set_id!(creep, TARGET, target);
        }
    }
    fn execute(&self, creep: &Creep) -> bool {
        if creep.is_empty(ResourceType::Energy) {
            return false;
        }
        if let Some(target) = &self.target {
            if let Some(transferable) = target.as_transferable() {
                match creep.transfer_all(transferable, ResourceType::Energy) {
                    ReturnCode::Ok => return true,
                    ReturnCode::Full | ReturnCode::NotEnough => return false,
                    ReturnCode::NotInRange => {
                        creep.move_to(target);
                        return true;
                    }
                    x => warn!("Failed to store energy: {:?}", x),
                }
            }
        }
        false
    }
}
