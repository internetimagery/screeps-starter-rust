// Actions relating to transporting goods from one place to another. eg energy

use super::{prelude::*, Action, ActionProvider, ActionResult};
use crate::prelude::*;
use log::*;
use screeps::{
    game, prelude::*, Creep, ResourceType, ReturnCode, Source, Structure, StructureSpawn,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HarvestEnergy {
    source: String,
}
#[derive(Serialize, Deserialize)]
pub struct StoreEnergy {
    target: String,
}
#[derive(Serialize, Deserialize)]
pub struct RenewLife {
    spawn: String,
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
    pub fn renew_life(&self, spawn: &StructureSpawn) {
        self.set_action(Action::RenewLife(RenewLife {
            spawn: to_id!(spawn),
        }))
    }
}

// Go get some more energy
impl ActionExecute<Creep> for HarvestEnergy {
    fn execute(&self, creep: &Creep) -> ActionResult {
        if creep.is_full(ResourceType::Energy) {
            return ActionResult::Abort; // Handle cases where energy has been transferred in transit
        }
        let target: Option<Source> = from_id!(&self.source);
        if let Some(target) = target {
            match creep.harvest(&target) {
                ReturnCode::Ok => {
                    if game::time() % 5 == 0 {
                        creep.say("⏳", true);
                    }
                    return ActionResult::Continue;
                }
                ReturnCode::NotInRange => {
                    creep.move_to(&target);
                    return ActionResult::Continue;
                }
                ReturnCode::Full => return ActionResult::Done,
                x => warn!("Failed to harvest: {:?}", x),
            }
        }
        ActionResult::Done
    }
}

// Store the engergy in a silo somewhere
impl ActionExecute<Creep> for StoreEnergy {
    fn execute(&self, creep: &Creep) -> ActionResult {
        let target: Option<Structure> = from_id!(&self.target);
        if let Some(target) = target {
            if let Some(transferable) = target.as_transferable() {
                match creep.transfer_all(transferable, ResourceType::Energy) {
                    ReturnCode::Ok | ReturnCode::Busy => return ActionResult::Continue,
                    ReturnCode::Full | ReturnCode::NotEnough => return ActionResult::Done,
                    ReturnCode::NotInRange => {
                        creep.move_to(&target);
                        return ActionResult::Continue;
                    }
                    x => warn!("Failed to store energy: {:?}", x),
                }
            }
        }
        ActionResult::Done
    }
}

// Run to spawn and try getting a renew
impl ActionExecute<Creep> for RenewLife {
    fn execute(&self, creep: &Creep) -> ActionResult {
        let spawn: Option<StructureSpawn> = from_id!(&self.spawn);
        if let Some(spawn) = spawn {
            match spawn.renew_creep(creep) {
                ReturnCode::Ok | ReturnCode::Full => return ActionResult::Done, // DONE!
                ReturnCode::Busy | ReturnCode::NotEnough | ReturnCode::NotInRange => {
                    creep.move_to(&spawn);
                    return ActionResult::Continue;
                }
                x => warn!("Failed to renew creep {:?}", x),
            }
        }
        ActionResult::Done
    }
}
