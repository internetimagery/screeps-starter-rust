// Mine energy quickly and transfer it to anyone that needs it.

use crate::prelude::*;
use crate::units::gatherer::Gatherer;
use log::*;
use screeps::objects::Creep;
use screeps::{find, prelude::*, Part, ResourceType, ReturnCode};

use crate::actions::*;
use crate::units::UnitController;

pub struct Miner {}

impl UnitController for Miner {
    fn get_name(&self) -> &'static str {
        "Miner"
    }
    fn get_body(&self) -> &'static [Part] {
        &[Part::Move, Part::Carry, Part::Work, Part::Work]
    }
    fn control_creep(&self, creep: &Creep) {
        if creep.is_empty(ResourceType::Energy) {
            return creep.set_action(Actions::HarvestEnergy);
        }
        // Creeps that need energy
        let my_pos = creep.pos();
        if let Some(nearby_creep) = creep
            .room()
            .find(find::CREEPS)
            .into_iter()
            .filter(|c| !c.is_full(ResourceType::Energy))
            .min_by_key(|c| c.pos().get_range_to(&my_pos))
        {
            match creep.transfer_all(&nearby_creep, ResourceType::Energy) {
                ReturnCode::Ok | ReturnCode::NotEnough => (),
                ReturnCode::NotInRange => {
                    creep.move_to(&nearby_creep);
                }
                x => warn!("Failed to transfer: {:?}", x),
            }
        } else {
            // Nothing needs energy, perform functions of a gatherer
            Gatherer {}.control_creep(creep);
        }
    }
}
