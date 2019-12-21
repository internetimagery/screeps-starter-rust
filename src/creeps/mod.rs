
use log::*;
use screeps::objects::Creep;
use screeps::{prelude::*, find, ResourceType, ReturnCode};


pub fn run_creep(creep: Creep) {
    debug!("Running creep {}", creep.name());
    if creep.spawning() {
        return;
    }

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
