// Actions relating to transporting goods from one place to another. eg energy

use super::{prelude::*, Action, ActionProvider};
use crate::prelude::*;
use log::*;
use screeps::{game, prelude::*, Creep, ResourceType, ReturnCode, Source, Structure};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HarvestEnergy {
    source: String,
}
#[derive(Serialize, Deserialize)]
pub struct StoreEnergy {
    target: String,
}

impl ActionProvider<'_, Creep> {
    pub fn harvest_energy(&self, source: &Source) {
        self.set_action(Action::HarvestEnergy(HarvestEnergy {
            source: to_id!(source),
        }));
    }
    pub fn store_energy(&self, target: &Structure) {
        self.set_action(Action::StoreEnergy(StoreEnergy {
            target: to_id!(target),
        }))
    }
}

// Go get some more energy
impl ActionExecute for HarvestEnergy {
    fn execute(&self, creep: &Creep) -> bool {
        if creep.is_full(ResourceType::Energy) {
            return false; // Handle cases where energy has been transferred in transit
        }
        let target: Option<Source> = from_id!(&self.source);
        if let Some(target) = target {
            match creep.harvest(&target) {
                ReturnCode::Ok => {
                    if game::time() % 5 == 0 {
                        creep.say("⏳", true);
                    }
                    return true;
                }
                ReturnCode::NotInRange => {
                    creep.move_to(&target);
                    return true;
                }
                ReturnCode::Full => return false,
                x => warn!("Failed to harvest: {:?}", x),
            }
        }
        false
    }
}

// Store the engergy in a silo somewhere
impl ActionExecute for StoreEnergy {
    fn execute(&self, creep: &Creep) -> bool {
        let target: Option<Structure> = from_id!(&self.target);
        if let Some(target) = target {
            if let Some(transferable) = target.as_transferable() {
                match creep.transfer_all(transferable, ResourceType::Energy) {
                    ReturnCode::Ok | ReturnCode::Busy => return true,
                    ReturnCode::Full | ReturnCode::NotEnough => return false,
                    ReturnCode::NotInRange => {
                        creep.move_to(&target);
                        return true;
                    }
                    x => warn!("Failed to store energy: {:?}", x),
                }
            }
        }
        false
    }
}
