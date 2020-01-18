use super::infrastructure::supply_roads;
use crate::actions::prelude::*;
use crate::quests::prelude::*;
use crate::prelude::*;
use crate::strategies::Strategy;
use crate::units::Unit;
use screeps::pathfinder::{search, SearchOptions};
use screeps::{find, game, prelude::*, Creep, ResourceType, Structure, StructureSpawn, StructureTower};

pub trait StrategySpawn {
    fn recruit(&self, spawn: &StructureSpawn);
}

pub trait UnitCreep {
    fn think(&self, creep: &Creep);
}

pub fn manage_forces(spawns: Vec<StructureSpawn>, mut creeps: Vec<Creep>) {
    // Remove creeps from Vec as they get jobs assigned
    creeps.retain(|c| !c.spawning()); // Ignore creeps still spawning
    creeps.retain(prolong_life); // Restore low lifespawn creeps near spawns (eg workers)
    creeps.retain(|c| !c.actions().execute()); // Run pending actions
    creeps.retain(needs_energy); // Empty creeps go get energy
    creeps.retain(|c| !c.pick_up_quest()); // Pick up any new quests

    let time = game::time();

    // Lay out some infrastructure for collecting source energy
    if time % 500 == 0 {
        for spawn in &spawns {
            supply_roads(spawn);
        }
    }

    // Light town defense
    if time % 2 == 0 {
        for structure in game::structures::values() {
            if let Structure::Tower(tower) = structure {
                defend_room(&tower);
            }
        }
    }

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

// If empty, go get some energy
fn needs_energy(creep: &Creep) -> bool {
    if !creep.is_empty(ResourceType::Energy) {
        return true;
    }
    // TODO, add more complex logic to get the optimal source.
    // This is run infrequently, so can afford a cpu spike
    creep.actions().harvest_energy(&creep.nearest_source());
    false
}

fn prolong_life(creep: &Creep) -> bool {
    if creep.ticks_to_live() != 100 {
        return true;
    }
    if let Some(spawn) = creep.pos().find_closest_by_range(find::MY_SPAWNS) {
        let result = search(creep, &spawn, 2, SearchOptions::new());
        if result.opaque_path().len() < 50 {
            // We are close enough and have enough life left it's probably worth renewing!
            creep.actions().renew_life(&spawn);
        }
    }
    true
}

fn defend_room(tower: &StructureTower) {
    let my_pos = tower.pos();
    if let Some(enemy) = tower.room().find(find::HOSTILE_CREEPS).into_iter().min_by_key(|c| c.pos().get_range_to(&my_pos)) {
        tower.attack(&enemy);
    }
}
