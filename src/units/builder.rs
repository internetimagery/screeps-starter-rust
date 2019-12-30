// Go get energy, or build something
// Simple cheap starter unit

use log::*;
use screeps::objects::Creep;
use screeps::{game, prelude::*, Part, ResourceType, ReturnCode};

use crate::actions::*;
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
            return creep.set_action(Actions::HarvestEnergy);
        }
        let full = creep.is_full(ResourceType::Energy);
        let empty = creep.is_empty(ResourceType::Energy);
        let source = creep.nearest_source();
        let constructions = game::construction_sites::values();
        // let constructions = game::structures::values()
        //     .into_iter()
        //     .filter(|structure| match structure.as_has_store() {
        //         Some(construct) => construct.store_free_capacity(Some(ResourceType::Energy)) > 0,
        //         None => false,
        //     })
        //     .collect::<Vec<_>>();

        // There is nothing to build. Help out with gathering
        if empty || constructions.len() == 0 {
            Gatherer {}.control_creep(creep);
            return;
        }
        // let construction = constructions
        //     .into_iter()
        //     .min_by_key(|c| c.as_has_store().unwrap().energy())
        //     .unwrap();
        let my_pos = creep.pos();
        let construction = constructions
            .into_iter()
            .min_by_key(|c| c.pos().get_range_to(&my_pos))
            .unwrap();

        if full {
            creep.move_to(&construction);
        }

        // Harvest or transfer
        match creep.harvest(&source) {
            ReturnCode::Ok => (),
            ReturnCode::NotInRange => {
                match creep.build(&construction) {
                    ReturnCode::Ok | ReturnCode::NotEnough => (),
                    ReturnCode::NotInRange => {
                        // If creep has a little bit of energy, use the last of it
                        if creep.store_used_capacity(Some(ResourceType::Energy)) > 0 {
                            creep.move_to(&construction);
                        }
                    }
                    x => warn!("Failed to build: {:?}", x),
                }
            }
            x => warn!("Failed to harvest: {:?}", x),
        }
    }

    //
    // // Harvest or transfer
    // match creep.harvest(source) {
    //     ReturnCode::Ok => (),
    //     ReturnCode::NotInRange => {
    //         match creep.transfer_all(
    //             construction.as_transferable().unwrap(),
    //             ResourceType::Energy,
    //         ) {
    //             ReturnCode::Ok | ReturnCode::NotEnough => (),
    //             ReturnCode::NotInRange => {
    //                 // If creep has a little bit of energy, use the last of it
    //                 if creep.store_used_capacity(Some(ResourceType::Energy)) > 0 {
    //                     creep.move_to(&construction);
    //                 }
    //             }
    //             x => warn!("Failed to upgrade controller: {:?}", x),
    //         }
    //     }
    //     x => warn!("Failed to harvest: {:?}", x),
    // }
}
