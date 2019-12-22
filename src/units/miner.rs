// Mine energy quickly and transfer it to anyone that needs it.

use crate::units::gatherer::Gatherer;
use log::*;
use screeps::objects::Creep;
use screeps::{find, prelude::*, Part, ResourceType, ReturnCode};

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
        let full = creep.store_free_capacity(Some(ResourceType::Energy)) == 0;
        let empty = creep.store_used_capacity(Some(ResourceType::Energy)) == 0;
        let source = &creep.room().find(find::SOURCES)[0];
        let creeps = creep
            .room()
            .find(find::CREEPS)
            .into_iter()
            .filter(|c| c.store_free_capacity(Some(ResourceType::Energy)) > 0)
            .collect::<Vec<_>>();

        if creeps.len() == 0 || empty {
            // There is nobody to help. Might as well help gather.
            Gatherer {}.control_creep(creep);
            return;
        }
        let my_pos = creep.pos();
        let closest_creep = creeps
            .into_iter()
            .min_by_key(|c| c.pos().get_range_to(&my_pos))
            .unwrap();

        if full {
            creep.move_to(&closest_creep);
        } else {
            creep.move_to(source);
        }

        // Harvest or transfer
        match creep.harvest(source) {
            ReturnCode::Ok => {
                creep.say("⏳", true);
            }
            ReturnCode::NotInRange => {
                match creep.transfer_all(&closest_creep, ResourceType::Energy) {
                    ReturnCode::Ok | ReturnCode::NotEnough => (),
                    ReturnCode::NotInRange => {
                        // If creep has a little bit of energy, use the last of it
                        if creep.store_used_capacity(Some(ResourceType::Energy)) > 0 {
                            creep.move_to(&closest_creep);
                        }
                    }
                    x => warn!("Failed to transfer: {:?}", x),
                }
            }
            x => warn!("Failed to harvest: {:?}", x),
        }
    }
}
