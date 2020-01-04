use crate::actions::prelude::*;
use crate::strategies::manage_forces;
use log::*;
use screeps::{game, memory};
use std::collections::HashSet;

pub fn game_loop() {
    let starting_cpu = game::cpu::get_used();

    // Run all creep actions if any are pending
    let creeps: Vec<_> = game::creeps::values()
        .into_iter()
        .filter(|c| !c.execute_action())
        .collect();

    // TODO: manage a spawn queue, for spawns to save up for units
    let spawns = game::spawns::values();

    // Control our forces
    manage_forces(spawns, creeps);

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
