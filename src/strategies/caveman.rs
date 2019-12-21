// Starting from nothing. Build some basic units to gather materials and upgrade equally
use crate::strategies::{StrategyController, UnitSpawn};
use crate::units::{Unit, UnitTypes::Upgrader};
use log::*;
use screeps::objects::StructureSpawn;
use screeps::prelude::*;

pub struct Caveman {}

impl StrategyController for Caveman {
    fn recruit(&self, spawn: &StructureSpawn) {
        // Blindly try to build upgraders! So smart...
        let unit = Unit::from(Upgrader);
        if spawn.energy() < unit.cost() {
            return;
        }
        if let Some(unit_id) = unit.create(&spawn) {
            info!("Created {}", unit_id);
        }
    }
}
