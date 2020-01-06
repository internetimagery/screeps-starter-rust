use crate::strategies::Strategy;
use crate::units::Unit;
use screeps::{Creep, StructureSpawn};

pub trait StrategySpawn {
    fn recruit(&self, spawn: &StructureSpawn);
}

pub trait UnitCreep {
    fn think(&self, creep: &Creep);
}

// struct Busy {
//     busy: bool,
//     creep: Creep,
// }

pub fn manage_forces(spawns: Vec<StructureSpawn>, creeps: Vec<Creep>) {
    // let mut creeps: Vec<_> = creeps
    //     .into_iter()
    //     .map(|c| Busy {
    //         busy: false,
    //         creep: c,
    //     })
    //     .collect();
    // TODO: Refactor into basic base building logic
    // Refactor unit type logic. Make it more centralized. So different units can take on different tasks
    // depending on what the rest of the team needs. Allowing more complex / complete AI

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
