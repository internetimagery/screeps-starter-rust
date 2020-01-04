use crate::strategies::Strategy;
use crate::units::Unit;
use screeps::{Creep, StructureSpawn};

pub trait StrategySpawn {
    fn recruit(&self, spawn: &StructureSpawn);
}

pub trait UnitCreep {
    fn think(&self, creep: &Creep);
}

pub fn manage_forces(spawns: Vec<StructureSpawn>, creeps: Vec<Creep>) {
    // Spawn some units (maybe)
    for spawn in spawns {
        let strategy = Strategy::from(&spawn);
        strategy.recruit(&spawn);
    }

    // Run our creeps AI
    for creep in creeps {
        let unit = Unit::from(&creep);
        unit.think(&creep);
    }
}
