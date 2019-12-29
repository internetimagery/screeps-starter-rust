// Grab some energy and give it out to everyone.
// Simple cheap starter unit

use crate::units::upgrader::Upgrader;
use log::*;
use screeps::objects::Creep;
use screeps::{Part, ResourceType, ReturnCode};

use crate::units::{prelude::*, UnitController};

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
        // If creep is empty, defer to upgrader for harvesting logic
        if creep.is_empty() {
            return Upgrader {}.control_creep(creep);
        }
        // Get spawn. If we have no spawn, do some upgrades
        let spawn = match creep.get_spawn() {
            Some(spawn) => spawn,
            _ => return Upgrader {}.control_creep(creep),
        };
        // Go give some energy
        if creep.is_full() {
            creep.move_to(&spawn);
        }
        match creep.transfer_all(&spawn, ResourceType::Energy) {
            ReturnCode::Ok | ReturnCode::NotEnough => (),
            ReturnCode::NotInRange => {
                creep.move_to(&spawn); // Handle some energy but not full
            }
            x => warn!("Failed to give spawn energy: {:?}", x),
        }
    }
}
