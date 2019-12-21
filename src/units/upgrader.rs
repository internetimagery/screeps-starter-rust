// Grab some energy (like gatherer) and go do some upgrades.
// Simple cheap starter unit

use log::*;
use screeps::objects::Creep;
use screeps::{find, prelude::*, Part, ResourceType, ReturnCode};

use crate::units::UnitController;

pub struct Upgrader {}

impl UnitController for Upgrader {
    fn get_name(&self) -> &'static str {
        "Gatherer"
    }
    fn get_body(&self) -> &'static [Part] {
        &[Part::Move, Part::Carry, Part::Work]
    }
    fn control_creep(&self, creep: &Creep) {
        if creep.memory().bool("harvesting") {
            if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
                creep.memory().set("harvesting", false);
            }
        } else {
            if creep.store_used_capacity(None) == 0 {
                creep.memory().set("harvesting", true);
            }
        }

        if creep.memory().bool("harvesting") {
            let source = &creep.room().find(find::SOURCES)[0];
            if creep.pos().is_near_to(source) {
                creep.say("⏳", true);
                let r = creep.harvest(source);
                if r != ReturnCode::Ok {
                    warn!("couldn't harvest: {:?}", r);
                }
            } else {
                creep.move_to(source);
            }
        } else {
            if let Some(c) = creep.room().controller() {
                let r = creep.upgrade_controller(&c);
                if r == ReturnCode::NotInRange {
                    creep.move_to(&c);
                } else if r != ReturnCode::Ok {
                    warn!("couldn't upgrade: {:?}", r);
                }
            } else {
                warn!("creep room has no controller!");
            }
        }
    }
}