// Grab some energy and give it out to everyone.
// Simple cheap starter unit

use crate::units::upgrader::Upgrader;
use screeps::objects::Creep;
use screeps::{Part, ResourceType, Structure};

use crate::actions::prelude::*;
use crate::actions::Action;

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
            let source = creep.nearest_source();
            return creep.set_action(Action::harvest_energy(source));
        }
        // Get spawn. If we have no spawn, do some upgrades
        if let Some(spawn) = creep.get_spawn() {
            return creep.set_action(Action::store_energy(Structure::Spawn(spawn)));
        } else {
            Upgrader {}.control_creep(creep)
        }
    }
}
