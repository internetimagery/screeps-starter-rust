// Grab some energy and give it out to everyone.
// Simple cheap starter unit

use crate::units::upgrader::Upgrader;
use screeps::objects::Creep;
use screeps::{game, prelude::*, Part, ResourceType};

use crate::actions::prelude::*;

use crate::prelude::*;
use crate::units::UnitController;

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
            return creep.actions().harvest_energy(&source);
        }
        let my_pos = creep.pos();
        // Get spawn. If we have no spawn, do some upgrades
        if let Some(structure) = game::structures::values()
            .into_iter()
            .filter(|s| match s.as_has_energy_for_spawn() {
                Some(store) => store.store_free_capacity(Some(ResourceType::Energy)) > 0,
                None => false,
            })
            .min_by_key(|s| s.pos().get_range_to(&my_pos))
        {
            return creep.actions().store_energy(&structure);
        }

        if let Some(structure) = game::structures::values()
            .into_iter()
            .filter(|s| match s.as_has_store() {
                Some(store) => store.store_free_capacity(Some(ResourceType::Energy)) > 0,
                None => false,
            })
            .min_by_key(|s| s.pos().get_range_to(&my_pos))
        {
            return creep.actions().store_energy(&structure);
        }

        Upgrader {}.control_creep(creep);
    }
}
