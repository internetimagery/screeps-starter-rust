// Actions relating to transporting goods from one place to another. eg energy

use crate::actions::prelude::*;
use crate::actions::Action;
use crate::prelude::*;
use log::*;
use screeps::{game, Creep, ResourceType, ReturnCode, Source};
use std::convert::From;

const SOURCE: &'static str = "source";

pub struct HarvestEnergy {
    source: Option<Source>,
}

impl Action {
    pub fn harvest_energy(source: Source) -> Action {
        Action::HarvestEnergy(HarvestEnergy {
            source: Some(source),
        })
    }
}

impl From<&Creep> for HarvestEnergy {
    fn from(creep: &Creep) -> Self {
        if let Ok(Some(source)) = creep.memory().get(SOURCE) {
            return Self {
                source: Some(source),
            };
        }
        Self { source: None }
    }
}

impl Actionable for HarvestEnergy {
    fn save(&self, creep: &Creep) {
        if let Some(source) = &self.source {
            creep.memory().set(SOURCE, source);
        }
    }
    fn execute(&self, creep: &Creep) -> bool {
        if creep.is_full(ResourceType::Energy) {
            return false;
        }
        if let Some(source) = &self.source {
            match creep.harvest(source) {
                ReturnCode::Ok => {
                    if game::time() % 5 == 0 {
                        creep.say("⏳", true);
                    }
                }
                ReturnCode::NotInRange => {
                    creep.move_to(source);
                }
                x => warn!("Failed to harvest: {:?}", x),
            }
            return true; // Need more energy!
        }
        false
    }
}
