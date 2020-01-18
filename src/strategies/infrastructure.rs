// Lay out some roads!

use log::*;
use screeps::pathfinder::{search, SearchOptions};
use screeps::{find, prelude::*, Structure, StructureSpawn, StructureType};

// Lay out construction sites to spawns, extensions and sources
pub fn supply_roads(spawn: &StructureSpawn) {
    info!("Upgrading supply infruastructure");
    let room = spawn.room();
    let sources = room.find(find::SOURCES);
    let mut stores: Vec<_> = room
        .find(find::STRUCTURES)
        .into_iter()
        .filter(|s| s.as_has_energy_for_spawn().is_some())
        .collect();
    if let Some(controller) = room.controller() {
        stores.push(Structure::Controller(controller));
    }
    for source in sources {
        for store in &stores {
            let options = SearchOptions::new().swamp_cost(1);
            for pos in search(&source, store, 1, options).load_local_path() {
                room.create_construction_site(&pos, StructureType::Road);
            }
        }
    }
}
