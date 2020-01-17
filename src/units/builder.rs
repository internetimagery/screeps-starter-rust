// Build and repair
// Simple cheap starter unit

use screeps::objects::Creep;
use screeps::{find, game, prelude::*, Part, ResourceType};

use crate::actions::prelude::*;
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
            return creep.actions().harvest_energy(&source);
        }

        let my_pos = creep.pos();
        if let Some(structure) = creep
            .room()
            .find(find::STRUCTURES)
            .into_iter()
            .filter(|s| match s.as_owned() {
                Some(unit) => unit.my(),
                None => false,
            })
            .filter(|s| match s.as_attackable() {
                Some(unit) => unit.hits() < unit.hits_max(),
                None => false,
            })
            .min_by_key(|s| s.pos().get_range_to(&my_pos))
        {
            return creep.actions().repair_structure(&structure);
        }

        if let Some(construction) = game::construction_sites::values()
            .into_iter()
            .min_by_key(|c| c.pos().get_range_to(&my_pos))
        {
            return creep.actions().build_site(&construction);
        }
        Gatherer {}.control_creep(creep);
    }
}
