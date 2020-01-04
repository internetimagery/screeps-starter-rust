// Build and repair
// Simple cheap starter unit

use log::*;
use screeps::objects::Creep;
use screeps::{game, prelude::*, Part, ResourceType, ReturnCode};

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
            return creep.set_action(Action::harvest_energy());
        }
        let my_pos = creep.pos();
        if let Some(structure) = game::structures::values()
            .into_iter()
            .filter(|s| s.needs_repair())
            .min_by_key(|s| s.pos().get_range_to(&my_pos))
        {
            match creep.repair(&structure) {
                ReturnCode::NotEnough => creep.set_action(Action::harvest_energy()),
                ReturnCode::NotInRange => {
                    creep.move_to(&structure);
                }
                ReturnCode::Ok => (),
                x => warn!("Failed to build {:?}", x),
            }
            return;
        }
        if let Some(construction) = game::construction_sites::values()
            .into_iter()
            .min_by_key(|c| c.pos().get_range_to(&my_pos))
        {
            match creep.build(&construction) {
                ReturnCode::NotEnough => creep.set_action(Action::harvest_energy()),
                ReturnCode::NotInRange => {
                    creep.move_to(&construction);
                }
                ReturnCode::Ok => (),
                x => warn!("Failed to build {:?}", x),
            }
            return;
        }
        Gatherer {}.control_creep(creep);
    }
}
