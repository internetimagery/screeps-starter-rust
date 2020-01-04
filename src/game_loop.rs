use crate::actions::prelude::*;
use crate::strategies::Strategy;
use crate::units::Unit;
use log::*;
use screeps::objects::{Creep, StructureSpawn};
use screeps::{game, memory};
use std::collections::HashSet;

pub trait StrategySpawn {
    fn recruit(&self, spawn: &StructureSpawn);
}

pub trait UnitCreep {
    fn think(&self, creep: &Creep);
}

pub fn game_loop() {
    let starting_cpu = game::cpu::get_used();

    // Spawn some units (maybe)
    for spawn in game::spawns::values() {
        let strategy = Strategy::from(&spawn);
        strategy.recruit(&spawn);
    }

    // Run our creeps AI
    for creep in game::creeps::values() {
        debug!("Running creep: {}", creep.name());
        if creep.execute_action() {
            continue;
        }
        let unit = Unit::from(&creep);
        unit.think(&creep);
    }

    // Be a good citizen
    run_cleanup();

    info!("Tick took cpu: {}", game::cpu::get_used() - starting_cpu);
}

pub fn run_cleanup() {
    if game::time() % 32 != 3 {
        return;
    }
    debug!("Running memory cleanup...");

    let alive_creeps: HashSet<String> = game::creeps::keys().into_iter().collect();
    if let Ok(Some(screeps_memory)) = memory::root().dict("creeps") {
        for mem_name in screeps_memory.keys() {
            if !alive_creeps.contains(&mem_name) {
                debug!("Cleaning up creep memory of dead creep {}", mem_name);
                screeps_memory.del(&mem_name);
            }
        }
    } else {
        warn!("Failed to clean up creeps memory. No 'Memory.creeps' dict");
    }
}
