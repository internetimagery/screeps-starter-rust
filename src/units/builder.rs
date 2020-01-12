// Build and repair
// Simple cheap starter unit

use screeps::objects::Creep;
use screeps::{game, prelude::*, Part, ResourceType};

use crate::actions::prelude::*;
use crate::actions::Action;
use crate::prelude::*;
use crate::units::gatherer::Gatherer;
use crate::units::UnitController;

pub struct Builder {}

impl UnitController for Builder {
    fn get_name(&self) -> &'static str {
        "Builder"
    }
    fn get_body(&self) -> &'static [Part] {
        // Needs to cost < 300
        &[Part::Move, Part::Carry, Part::Work]
    }
    fn control_creep(&self, creep: &Creep) {
        if creep.is_empty(ResourceType::Energy) {
            let source = creep.nearest_source();
            return creep.set_action(Action::harvest_energy(source));
        }
        let my_pos = creep.pos();
        if let Some(structure) = game::structures::values()
            .into_iter()
            .filter(|s| s.needs_repair())
            .min_by_key(|s| s.pos().get_range_to(&my_pos))
        {
            use log::warn;
            warn!("NEEDS REPAIR");
            return creep.set_action(Action::repair_structure(structure));
        }
        if let Some(construction) = game::construction_sites::values()
            .into_iter()
            .min_by_key(|c| c.pos().get_range_to(&my_pos))
        {
            return creep.set_action(Action::build_site(construction));
        }
        Gatherer {}.control_creep(creep);
    }
}
