// Grab some energy and give it out to everyone.
// Simple cheap starter unit

use crate::units::upgrader::Upgrader;
use log::*;
use screeps::objects::Creep;
use screeps::{find, prelude::*, Part, ResourceType, ReturnCode};

use crate::units::{CreepSpawn, UnitController};

pub struct Gatherer {}

impl UnitController for Gatherer {
    fn get_name(&self) -> &'static str {
        "Gatherer"
    }
    fn get_body(&self) -> &'static [Part] {
        &[Part::Move, Part::Carry, Part::Work]
    }
    fn control_creep(&self, creep: &Creep) {
        let full = creep.store_free_capacity(Some(ResourceType::Energy)) == 0;
        let empty = creep.store_used_capacity(Some(ResourceType::Energy)) == 0;
        let source = &creep.room().find(find::SOURCES)[0];
        let spawn = creep.get_spawn();

        // If the spawn is already full, perform the task of an upgrader
        if spawn.is_some()
            && spawn
                .as_ref()
                .unwrap()
                .store_free_capacity(Some(ResourceType::Energy))
                == 0
        {
            Upgrader {}.control_creep(creep);
            return;
        }

        // Go get some energy or upgrade
        if empty {
            creep.move_to(source);
        } else if full && spawn.is_some() {
            creep.move_to(spawn.as_ref().unwrap());
        }

        // Harvest or transfer
        match creep.harvest(source) {
            ReturnCode::Ok => {
                creep.say("⏳", true);
            }
            ReturnCode::NotInRange => {
                if spawn.is_some() {
                    match creep.transfer_all(spawn.as_ref().unwrap(), ResourceType::Energy) {
                        ReturnCode::Ok | ReturnCode::NotEnough => (),
                        ReturnCode::NotInRange => {
                            // If creep has a little bit of energy, use the last of it
                            if creep.store_used_capacity(Some(ResourceType::Energy)) > 0 {
                                creep.move_to(&spawn.unwrap());
                            }
                        }
                        x => warn!("Failed to upgrade controller: {:?}", x),
                    }
                }
            }
            x => warn!("Failed to harvest: {:?}", x),
        }
    }
}
