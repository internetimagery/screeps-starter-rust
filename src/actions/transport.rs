// Actions relating to transporting goods from one place to another. eg energy

use crate::prelude::*;
use crate::{get_id, set_id};
use log::*;
use screeps::{game, prelude::*, Creep, ResourceType, ReturnCode, Source, Structure};

// Go get some more energy
action_target! {
    fn harvest_energy(target: Source) -> HarvestEnergy;
    fn execute(&self, creep: &Creep) -> bool {
        if let Some(target) = &self.target {
            match creep.harvest(target) {
                ReturnCode::Ok => {
                    if game::time() % 5 == 0 {
                        creep.say("⏳", true);
                        return true;
                    }
                }
                ReturnCode::NotInRange => {
                    creep.move_to(target);
                    return true
                }
                ReturnCode::Full => return false,
                x => warn!("Failed to harvest: {:?}", x),
            }
        }
        false
    }
}

// Store the engergy in a silo somewhere
action_target! {
    fn store_energy(target: Structure) -> StoreEnergy;
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
