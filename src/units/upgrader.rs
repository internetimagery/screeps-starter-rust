// Grab some energy (like gatherer) and go do some upgrades.
// Simple cheap starter unit

use log::*;
use screeps::objects::Creep;
use screeps::{find, prelude::*, Part, ResourceType, ReturnCode};

use crate::units::UnitController;

pub struct Upgrader {}

impl UnitController for Upgrader {
    fn get_name(&self) -> &'static str {
        "Upgrader"
    }
    fn get_body(&self) -> &'static [Part] {
        &[Part::Move, Part::Carry, Part::Work]
    }
    fn control_creep(&self, creep: &Creep) {
        // Check if our creep is full of energy and head off to upgrade if so
        if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
            if let Some(c) = creep.room().controller() {
                match creep.upgrade_controller(&c) {
                    ReturnCode::NotInRange => {
                        creep.move_to(&c);
                    }
                    ReturnCode::Ok => (), // Success
                    x => warn!("Failed to upgrade controller: {:?}", x),
                }
            } else {
                warn!("Room has no controller!");
            }
        } else {
            // We have no energy, go get some more
            let source = &creep.room().find(find::SOURCES)[0];
            match creep.harvest(source) {
                ReturnCode::NotInRange => {
                    creep.move_to(source);
                }
                ReturnCode::Ok => {
                    creep.say("⏳", true);
                }
                x => warn!("Failed to harvest: {:?}", x),
            }
        }
    }
}
