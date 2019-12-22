// Grab some energy and give it out to everyone.
// Simple cheap starter unit

use crate::units::upgrader::Upgrader;
use log::*;
use screeps::objects::Creep;
use screeps::{prelude::*, Part, ResourceType, ReturnCode};

use crate::units::{CreepSpawn, UnitController};

pub struct Gatherer {}

impl UnitController for Gatherer {
    fn get_name(&self) -> &'static str {
        "Gatherer"
    }
    fn get_body(&self) -> &'static [Part] {
        // Needs to cost < 300
        &[Part::Move, Part::Move, Part::Carry, Part::Work]
    }
    fn control_creep(&self, creep: &Creep) {
        let full = creep.store_free_capacity(Some(ResourceType::Energy)) == 0;
        let empty = creep.store_used_capacity(Some(ResourceType::Energy)) == 0;
        let spawn = creep.get_spawn();

        // If empty or the spawn is already full, perform the task of an upgrader
        if empty
            || (spawn.is_some()
                && spawn
                    .as_ref()
                    .unwrap()
                    .store_free_capacity(Some(ResourceType::Energy))
                    == 0)
        {
            Upgrader {}.control_creep(creep);
            return;
        }

        // Go give the spawn some energy
        if spawn.is_some() {
            if full {
                creep.move_to(spawn.as_ref().unwrap());
            }
            match creep.transfer_all(spawn.as_ref().unwrap(), ResourceType::Energy) {
                ReturnCode::Ok | ReturnCode::NotEnough => (),
                ReturnCode::NotInRange => {
                    // If creep has a little bit of energy, use the last of it
                    if creep.store_used_capacity(Some(ResourceType::Energy)) > 0 {
                        creep.move_to(&spawn.unwrap());
                    }
                }
                x => warn!("Failed to give spawn energy: {:?}", x),
            }
        }
    }
}
