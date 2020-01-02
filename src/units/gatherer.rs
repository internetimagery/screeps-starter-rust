// Grab some energy and give it out to everyone.
// Simple cheap starter unit

use crate::units::upgrader::Upgrader;
use log::*;
use screeps::objects::Creep;
use screeps::{Part, ResourceType, ReturnCode};

use crate::actions::*;
use crate::actions::transport::HarvestEnergy;
use crate::prelude::*;
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
        if creep.is_empty(ResourceType::Energy) {
            return creep.set_action(HarvestEnergy::new());
        }
        // Get spawn. If we have no spawn, do some upgrades
        if let Some(spawn) = creep.get_spawn() {
            match creep.transfer_all(&spawn, ResourceType::Energy) {
                ReturnCode::Ok | ReturnCode::NotEnough | ReturnCode::Full => (),
                ReturnCode::NotInRange => {
                    creep.move_to(&spawn); // Handle some energy but not full
                }
                x => warn!("Failed to give spawn energy: {:?}", x),
            }
        } else {
            Upgrader {}.control_creep(creep)
        }
    }
}
