// Grab some energy (like gatherer) and go do some upgrades.
// Simple cheap starter unit
// Used eventually as a fallback for gatherers

use log::*;
use screeps::objects::Creep;
use screeps::{game, prelude::*, Part, ResourceType, ReturnCode};

use crate::units::{prelude::*, UnitController};

pub struct Upgrader {}

impl UnitController for Upgrader {
    fn get_name(&self) -> &'static str {
        "Upgrader"
    }
    fn get_body(&self) -> &'static [Part] {
        // Needs to cost < 300
        &[Part::Move, Part::Move, Part::Carry, Part::Work]
    }
    fn control_creep(&self, creep: &Creep) {
        let source = creep.nearest_source();
        let controller = creep.room().controller();

        // Move to source or controller
        if creep.is_empty() {
            creep.move_to(&source);
        } else if creep.is_full() && controller.is_some() {
            creep.move_to(controller.as_ref().unwrap());
        }

        // Harvest or upgrade
        match creep.harvest(&source) {
            ReturnCode::Ok => {
                if game::time() % 5 == 0 {
                    creep.say("⏳", true);
                }
            }
            ReturnCode::NotInRange => {
                if controller.is_some() {
                    match creep.upgrade_controller(&controller.as_ref().unwrap()) {
                        ReturnCode::Ok | ReturnCode::NotEnough => (),
                        ReturnCode::NotInRange => {
                            // If creep has a little bit of energy, use the last of it
                            if creep.store_used_capacity(Some(ResourceType::Energy)) > 0 {
                                creep.move_to(&controller.unwrap());
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
