// Common traits for useful abstractions
use crate::units::{UnitTypes, ROLE, SPAWN};
use screeps::objects::{Creep, StructureSpawn};
use screeps::{game, prelude::*, ObjectId, ResourceType};
use std::str::FromStr;

pub trait CreepExtras {
    fn get_spawn(&self) -> Option<StructureSpawn>;
    fn get_role(&self) -> UnitTypes;
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
}

impl CreepExtras for Creep {
    // Get associated spawn point from stored ID
    fn get_spawn(&self) -> Option<StructureSpawn> {
        if let Ok(Some(id)) = self.memory().string(SPAWN) {
            if let Ok(object_id) = ObjectId::from_str(&id) {
                if let Ok(spawn) = object_id.try_resolve() {
                    return spawn;
                }
            }
        }
        None
    }
    // Get role of creep.
    // TODO: How can this conversion be automatic?
    fn get_role(&self) -> UnitTypes {
        match self.memory().i32(ROLE) {
            Ok(Some(c)) if c == UnitTypes::Upgrader as i32 => UnitTypes::Upgrader,
            Ok(Some(c)) if c == UnitTypes::Gatherer as i32 => UnitTypes::Gatherer,
            Ok(Some(c)) if c == UnitTypes::Builder as i32 => UnitTypes::Builder,
            Ok(Some(c)) if c == UnitTypes::Miner as i32 => UnitTypes::Miner,
            _ => UnitTypes::Zombie,
        }
    }
    // Return true if creep is full of energy
    fn is_full(&self) -> bool {
        self.store_free_capacity(Some(ResourceType::Energy)) == 0
    }
    // Return true if creep has no energy
    fn is_empty(&self) -> bool {
        self.store_used_capacity(Some(ResourceType::Energy)) == 0
    }
}

pub trait SpawnExtras {
    fn get_team(&self) -> Vec<Creep>;
}

impl SpawnExtras for StructureSpawn {
    // Get creeps associated with this spawn
    fn get_team(&self) -> Vec<Creep> {
        let team_id = self.id().to_string();
        let mut creeps = vec![];
        for creep in game::creeps::values() {
            if let Ok(Some(check_id)) = creep.memory().string(SPAWN) {
                if check_id == team_id {
                    creeps.push(creep);
                }
            }
        }
        creeps
    }
}
