// Common Unit traits for useful abstractions
use crate::units::{UnitTypes, ROLE, SPAWN};
use screeps::objects::{Creep, StructureSpawn};
use screeps::{game, prelude::*, ObjectId};
use std::convert::TryFrom;
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
    fn get_role(&self) -> UnitTypes {
        if let Ok(Some(role)) = self.memory().i32(ROLE) {
            if let Ok(unit_type) = UnitTypes::try_from(role) {
                return unit_type;
            }
        }
        UnitTypes::Zombie
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
