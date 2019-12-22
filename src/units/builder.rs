// Go get energy, or build something
// Simple cheap starter unit

use log::*;
use screeps::objects::Creep;
use screeps::{find, game, prelude::*, Part, ResourceType, ReturnCode};

use crate::units::upgrader::Upgrader;
use crate::units::UnitController;

pub struct Builder {}

impl UnitController for Builder {
    fn get_name(&self) -> &'static str {
        "Builder"
    }
    fn get_body(&self) -> &'static [Part] {
        &[
            Part::Move,
            Part::Carry,
            Part::Carry,
            Part::Work,
            Part::Tough,
        ]
    }
    fn control_creep(&self, creep: &Creep) {
        let full = creep.store_free_capacity(Some(ResourceType::Energy)) == 0;
        let empty = creep.store_used_capacity(Some(ResourceType::Energy)) == 0;
        let source = &creep.room().find(find::SOURCES)[0];
        let constructions = game::structures::values()
            .into_iter()
            .filter(|structure| match structure.as_has_store() {
                Some(construct) => construct.store_free_capacity(Some(ResourceType::Energy)) > 0,
                None => false,
            })
            .collect::<Vec<_>>();

        // There is nothing to build. Help out with upgrading
        if constructions.len() == 0 {
            Upgrader {}.control_creep(creep);
            return;
        }
        let construction = constructions
            .into_iter()
            .min_by_key(|c| c.as_has_store().unwrap().energy())
            .unwrap();

        // Go get some energy or build
        if empty {
            creep.move_to(source);
        } else if full {
            creep.move_to(&construction);
        }

        // Harvest or transfer
        match creep.harvest(source) {
            ReturnCode::Ok => (),
            ReturnCode::NotInRange => {
                match creep.transfer_all(
                    construction.as_transferable().unwrap(),
                    ResourceType::Energy,
                ) {
                    ReturnCode::Ok | ReturnCode::NotEnough => (),
                    ReturnCode::NotInRange => {
                        // If creep has a little bit of energy, use the last of it
                        if creep.store_used_capacity(Some(ResourceType::Energy)) > 0 {
                            creep.move_to(&construction);
                        }
                    }
                    x => warn!("Failed to upgrade controller: {:?}", x),
                }
            }
            x => warn!("Failed to harvest: {:?}", x),
        }
    }
}