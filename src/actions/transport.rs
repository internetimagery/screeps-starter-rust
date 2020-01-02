// Actions relating to transporting goods from one place to another. eg energy

use crate::prelude::*;
use log::*;
use screeps::{game, Creep, ResourceType, ReturnCode};
use crate::actions::Actionable;


pub struct HarvestEnergy {}

impl HarvestEnergy {
    pub fn new() -> Self {
        Self{}
    }
}

impl Actionable for HarvestEnergy {
    fn execute(creep: &Creep) -> bool {
        if creep.is_full(ResourceType::Energy) {
            return false;
        }
        let source = creep.nearest_source();
        match creep.harvest(&source) {
            ReturnCode::Ok => {
                if game::time() % 5 == 0 {
                    creep.say("⏳", true);
                }
            }
            ReturnCode::NotInRange => {}
            x => warn!("Failed to harvest: {:?}", x),
        }
        creep.move_to(&source);
        true // Need more energy!
    }
}
