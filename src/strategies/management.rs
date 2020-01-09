use crate::actions::prelude::*;
use crate::actions::Action;
use crate::prelude::*;
use crate::strategies::Strategy;
use crate::units::Unit;
use screeps::{Creep, ResourceType, StructureSpawn};

pub trait StrategySpawn {
    fn recruit(&self, spawn: &StructureSpawn);
}

pub trait UnitCreep {
    fn think(&self, creep: &Creep);
}

pub fn manage_forces(spawns: Vec<StructureSpawn>, mut creeps: Vec<Creep>) {
    // Remove creeps from Vec as they get jobs assigned
    creeps.retain(pending_actions); // Run pending actions
    creeps.retain(needs_energy); // Empty creeps go get energy

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

fn pending_actions(creep: &Creep) -> bool {
    !creep.execute_action()
}

// If empty, go get some energy
fn needs_energy(creep: &Creep) -> bool {
    if !creep.is_empty(ResourceType::Energy) {
        return true;
    }
    // TODO, add more complex logic to get the optimal source.
    // This is run infrequently, so can afford a cpu spike
    creep.set_action(Action::harvest_energy(creep.nearest_source()));
    false
}
