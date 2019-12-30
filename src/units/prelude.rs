// Common Unit traits for useful abstractions
use crate::units::{UnitTypes, ROLE, SPAWN};
use screeps::objects::{Creep, StructureSpawn};
use screeps::{game, prelude::*, ObjectId};
use std::str::FromStr;

pub trait CreepUnitExtras {
    fn get_spawn(&self) -> Option<StructureSpawn>;
    fn get_role(&self) -> UnitTypes;
}

impl CreepUnitExtras for Creep {
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
}

pub trait SpawnUnitExtras {
    fn get_team(&self) -> Vec<Creep>;
}

impl SpawnUnitExtras for StructureSpawn {
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
