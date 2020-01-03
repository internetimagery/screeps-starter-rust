// Grab some energy (like gatherer) and go do some upgrades.
// Simple cheap starter unit
// Used eventually as a fallback for gatherers

use log::*;
use screeps::objects::Creep;
use screeps::{prelude::*, Part, ResourceType, ReturnCode};

use crate::actions::*;

use crate::prelude::*;
use crate::units::UnitController;

pub struct Upgrader {}

impl UnitController for Upgrader {
    fn get_name(&self) -> &'static str {
        "Upgrader"
    }
    fn get_body(&self) -> &'static [Part] {
        // Needs to cost < 300 for early game building
        &[Part::Move, Part::Move, Part::Carry, Part::Work]
    }
    fn control_creep(&self, creep: &Creep) {
        if creep.is_empty(ResourceType::Energy) {
            return creep.set_action(Action::harvest_energy());
        }
        if let Some(controller) = creep.room().controller() {
            match creep.upgrade_controller(&controller) {
                ReturnCode::Ok | ReturnCode::NotEnough => (),
                ReturnCode::NotInRange => {
                    creep.move_to(&controller);
                }
                x => warn!("Failed to upgrade controller: {:?}", x),
            }
        } else {
            warn!("Creep has no controller. What to do? {}", creep.name());
        }
    }
}
