// Actions relating to transporting goods from one place to another. eg energy

use super::prelude::*;
use super::{Action, TARGET};
use crate::prelude::*;
use log::*;
use screeps::{prelude::*, game, Creep, ResourceType, ReturnCode, Source};
use std::convert::From;

pub struct HarvestEnergy {
    target: Option<Source>,
}

impl Action {
    pub fn harvest_energy(source: Source) -> Action {
        Action::HarvestEnergy(HarvestEnergy {
            target: Some(source),
        })
    }
}

impl From<&Creep> for HarvestEnergy {
    fn from(creep: &Creep) -> Self {
        if let Ok(Some(source)) = creep.memory().get(TARGET) {
            return Self {
                target: Some(source),
            };
        }
        Self { target: None }
    }
}

impl Actionable for HarvestEnergy {
    fn save(&self, creep: &Creep) {
        if let Some(source) = &self.target {
            creep.memory().set(TARGET, source);
        }
    }
    fn execute(&self, creep: &Creep) -> bool {
        if creep.is_full(ResourceType::Energy) {
            return false;
        }
        if let Some(source) = &self.target {
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

// Deliver energy
pub struct DeliverEnergy<T: HasPosition + HasStore> {
    target: Option<T>,
}

impl Action {
    pub fn deliver_energy<T: HasPosition + HasStore>(target: T) -> Action {
        Action::DeliverEnergy(DeliverEnergy {
            source: Some(target),
        })
    }
}
//
// impl From<&Creep> for DeliverEnergy {
//     fn from(creep: &Creep) -> Self {
//         if let Ok(Some(target)) = creep.memory().get(TARGET) {
//             return Self {
//                 target: Some(target),
//             };
//         }
//         Self { target: None }
//     }
// }
//
// impl Actionable for DeliverEnergy<T: HasPosition + HasStore> {
//     fn save(&self, creep: &Creep) {
//         if let Some(target) = &self.target {
//             creep.memory().set(TARGET, target);
//         }
//     }
//     fn execute(&self, creep: &Creep) -> bool {
//         if creep.is_empty(ResourceType::Energy) {
//             return false;
//         }
//         if let Some(source) = &self.source {
//             match creep.transfer_all(source, ResourceType::Energy) {
//                 ReturnCode::Ok => {
//                     if game::time() % 5 == 0 {
//                         creep.say("⏳", true);
//                     }
//                 }
//                 ReturnCode::NotInRange => {
//                     creep.move_to(source);
//                 }
//                 x => warn!("Failed to harvest: {:?}", x),
//             }
//             return true; // Need more energy!
//         }
//         false
//     }
// }
