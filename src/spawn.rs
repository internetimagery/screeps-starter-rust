use crate::units::{Unit, UnitTypes::*};
use log::*;
use screeps::objects::StructureSpawn;

pub trait UnitSpawn {
    fn create(&self, spawn: &StructureSpawn) -> Option<String>;
}

pub fn run_spawn(spawn: StructureSpawn) {
    debug!("Running spawn {}", spawn.name());
    // TODO: Dumb AI just creating gatherers as fast as possible
    let unit = Unit::from(Upgrader);
    if let Some(unit_id) = unit.create(&spawn) {
        info!("Created {}", unit_id);
    }
}