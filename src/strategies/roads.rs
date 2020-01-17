// Lay out some roads!

use log::*;
use screeps::pathfinder::{search, SearchOptions};
use screeps::{find, prelude::*, StructureSpawn, StructureType};

// Lay out construction sites to spawns, extensions and sources
pub fn supply_infrustructure(spawn: &StructureSpawn) {
    info!("Upgrading supply infruastructure");
    let room = spawn.room();
    let sources = room.find(find::SOURCES);
    let stores: Vec<_> = room
        .find(find::STRUCTURES)
        .into_iter()
        .filter(|s| s.as_has_energy_for_spawn().is_some())
        .collect();
    for source in sources {
        for store in &stores {
            let options = SearchOptions::new().swamp_cost(1);
            for pos in search(&source, store, 2, options).load_local_path() {
                room.create_construction_site(&pos, StructureType::Road);
            }
        }
    }
}
