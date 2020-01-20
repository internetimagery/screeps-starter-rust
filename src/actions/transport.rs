// Actions relating to transporting goods from one place to another. eg energy

use super::{Action, ActionProvider, ActionResult};
use crate::exception::Res;
use screeps::*;

// Initializers ///////////////////////////////////////////////////

impl ActionProvider<'_, Creep> {
    // pub fn go_to<T: HasPosition>(&self, target: T) {
    //     self.set_action(Action::GoTo {
    //         location: target.pos(),
    //     })
    // }
    pub fn harvest_energy(&self, source: &Source) {
        self.set_action(Action::HarvestEnergy {
            source: source.id(),
        })
    }
    pub fn store_energy(&self, store: &Structure) {
        self.set_action(Action::StoreEnergy { store: store.id() })
    }
    pub fn renew_life(&self, spawn: &StructureSpawn) {
        self.set_action(Action::RenewLife { spawn: spawn.id() })
    }
}

// Funtionality ////////////////////////////////////////////////////

// Travel somewhere
// pub fn go_to(creep: &Creep, location: &Position) -> Res<ActionResult> {
//     if creep.pos().is_near_to(location) {
//         return Ok(ActionResult::Done);
//     }
//     creep.move_to(location);
//     Ok(ActionResult::Continue)
// }

// Travel to a source and harvest it. Stop traveling if a creep gives it some energy.
pub fn harvest_energy(creep: &Creep, source: &Source) -> Res<ActionResult> {
    if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
        return Ok(ActionResult::Abort); // Handle cases where energy has been transferred in transit
    }
    match creep.harvest(source) {
        ReturnCode::Ok => {
            if game::time() % 5 == 0 {
                creep.say("⏳", true);
            }
            Ok(ActionResult::Continue)
        }
        ReturnCode::NotInRange => {
            creep.move_to(source);
            Ok(ActionResult::Continue)
        }
        ReturnCode::Full => Ok(ActionResult::Done),
        x => Err(format!("Failed to harvest: {:?}", x))?,
    }
}

// Go give energy to a structure. Store those acorns for the winter!
pub fn store_energy(creep: &Creep, store: &Structure) -> Res<ActionResult> {
    let transferable = store.as_transferable().ok_or("Not transferrable")?;
    match creep.transfer_all(transferable, ResourceType::Energy) {
        ReturnCode::Ok | ReturnCode::Busy => Ok(ActionResult::Continue),
        ReturnCode::Full | ReturnCode::NotEnough => Ok(ActionResult::Done),
        ReturnCode::NotInRange => {
            creep.move_to(store);
            Ok(ActionResult::Continue)
        }
        x => Err(format!("Failed to store energy: {:?}", x))?,
    }
}

pub fn renew_life(creep: &Creep, spawn: &StructureSpawn) -> Res<ActionResult> {
    match spawn.renew_creep(creep) {
        ReturnCode::Ok | ReturnCode::Full => Ok(ActionResult::Done), // DONE!
        ReturnCode::Busy | ReturnCode::NotEnough | ReturnCode::NotInRange => {
            creep.move_to(spawn);
            Ok(ActionResult::Continue)
        }
        x => Err(format!("Failed to renew creep {:?}", x))?,
    }
}
