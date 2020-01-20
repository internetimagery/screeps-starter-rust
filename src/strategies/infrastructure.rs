// Lay out some roads!

use log::*;
use screeps::pathfinder::{search, SearchOptions};
use screeps::{find, prelude::*, Structure, StructureSpawn, StructureType};

// TODO: make roads that lead in a single direction to and from locations, using locations id as name
// When discovering roads, use a custom heristic which heavily penalizes other roads
// so we get two roads, a lane each.
// If possible... see if road direction can be engrained in the road itself, for later pathfinding
// to correctly choose a lane.

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
