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
        let full = creep.store_free_capacity(Some(ResourceType::Energy)) == 0;
        let empty = creep.store_used_capacity(Some(ResourceType::Energy)) == 0;
        let source = &creep.room().find(find::SOURCES)[0];
        let controller = creep.room().controller();

        // Go get some energy or upgrade
        if empty {
            creep.move_to(source);
        } else if full && controller.is_some() {
            creep.move_to(controller.as_ref().unwrap());
        }

        // Harvest or upgrade
        match creep.harvest(source) {
            ReturnCode::Ok => {
                creep.say("⏳", true);
            }
            ReturnCode::NotInRange => {
                if controller.is_some() {
                    match creep.upgrade_controller(&controller.as_ref().unwrap()) {
                        ReturnCode::Ok => (), // Success
                        ReturnCode::NotInRange => {
                            if creep.store_used_capacity(Some(ResourceType::Energy)) > 0 {
                                creep.move_to(&controller.unwrap());
                            }
                        },
                        x => warn!("Failed to upgrade controller: {:?}", x),
                    }
                }
            }
            x => warn!("Failed to harvest: {:?}", x),
        }
        //
        // if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
        //     // Full Go to the controller!
        // } else if creep.store_used_capacity(Some(ResourceType::Energy)) == 0 {
        //     // Empty, go find some energy!
        // }
        //
        // if let Some(c) = creep.room().controller() {
        //     match creep.upgrade_controller(&c) {
        //         ReturnCode::NotInRange => {
        //             creep.move_to(&c);
        //         }
        //         ReturnCode::Ok => (), // Success
        //         x => warn!("Failed to upgrade controller: {:?}", x),
        //     }
        // } else {
        //     warn!("Room has no controller!");
        // }
        //
        //
        // // Check if our creep is full of energy and head off to upgrade if so
        // if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
        //     if let Some(c) = creep.room().controller() {
        //         match creep.upgrade_controller(&c) {
        //             ReturnCode::NotInRange => {
        //                 creep.move_to(&c);
        //             }
        //             ReturnCode::Ok => (), // Success
        //             x => warn!("Failed to upgrade controller: {:?}", x),
        //         }
        //     } else {
        //         warn!("Room has no controller!");
        //     }
        // } else {
        //     // We have no energy, go get some more
        //     let source = &creep.room().find(find::SOURCES)[0];
        //     match creep.harvest(source) {
        //         ReturnCode::NotInRange => {
        //             creep.move_to(source);
        //         }
        //         ReturnCode::Ok => {
        //             creep.say("⏳", true);
        //         }
        //         x => warn!("Failed to harvest: {:?}", x),
        //     }
        // }
    }
}
