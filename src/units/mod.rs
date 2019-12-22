// Units

use log::*;
use screeps::memory::MemoryReference;
use screeps::objects::{Creep, SpawnOptions, StructureSpawn};
use screeps::{game, prelude::*, ObjectId, Part, ReturnCode};
use std::str::FromStr;

use crate::game_loop::UnitCreep;
use crate::strategies::UnitSpawn;

// Basic Units
mod builder;
mod clumsy;
mod gatherer;
mod upgrader;

// Specialized Units
mod miner;

// Common fields
const ROLE: &'static str = "role";
const SPAWN: &'static str = "spawn";
pub const STATE: &'static str = "state";

// Unit type ID's
#[derive(Copy, Clone)]
pub enum UnitTypes {
    Zombie = 0,
    Gatherer = 1,
    Upgrader = 2,
    Builder = 3,
    Miner = 4,
}

// Required functionality of a controller
pub trait UnitController {
    fn get_name(&self) -> &'static str;
    fn get_body(&self) -> &'static [Part];
    fn get_memory(&self) -> MemoryReference {
        MemoryReference::new()
    }
    fn control_creep(&self, creep: &Creep);
}

// Wrapper around a creep control
pub struct Unit {
    unit_type: i32,
    controller: Box<dyn UnitController>,
}

// Get a new controller from a UnitType or Creep
impl From<i32> for Unit {
    fn from(unit_type: i32) -> Self {
        Unit {
            unit_type: unit_type,
            controller: match unit_type {
                x if x == UnitTypes::Upgrader as i32 => Box::new(upgrader::Upgrader {}),
                x if x == UnitTypes::Gatherer as i32 => Box::new(gatherer::Gatherer {}),
                x if x == UnitTypes::Builder as i32 => Box::new(builder::Builder {}),
                x if x == UnitTypes::Miner as i32 => Box::new(miner::Miner {}),
                _ => Box::new(clumsy::Clumsy {}),
            },
        }
    }
}

impl From<UnitTypes> for Unit {
    fn from(unit_type: UnitTypes) -> Self {
        Unit::from(unit_type as i32)
    }
}

impl From<&Creep> for Unit {
    fn from(creep: &Creep) -> Self {
        if let Ok(Some(unit_type)) = creep.memory().i32(ROLE) {
            Unit::from(unit_type)
        } else {
            warn!("Failed to get creep role: {}", creep.name());
            Unit::from(0)
        }
    }
}

pub trait CreepSpawn {
    fn get_spawn(&self) -> Option<StructureSpawn>;
}

impl CreepSpawn for Creep {
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
}

// Create a new creep
impl UnitSpawn for Unit {
    fn create(&self, spawn: &StructureSpawn) -> Option<String> {
        let name = self.controller.get_name();
        let body = self.controller.get_body();
        let memory = self.controller.get_memory();
        memory.set(ROLE, self.unit_type);
        memory.set(SPAWN, spawn.id().to_string());
        memory.set(STATE, 0);

        let mut index = game::time();
        let spawn_options = SpawnOptions::new().memory(memory);
        loop {
            let creep_id = format!("{}-{}", name, index);
            let result = spawn.spawn_creep_with_options(&body, &creep_id, &spawn_options);
            if result == ReturnCode::NameExists {
                index += 1;
                continue;
            }
            if result != ReturnCode::Ok {
                warn!("Failed to spawn {}: {:?}", name, result);
                return None;
            }
            break Some(creep_id);
        }
    }
}

// Control the creep
impl UnitCreep for Unit {
    fn think(&self, creep: &Creep) {
        if creep.spawning() {
            return;
        }
        self.controller.control_creep(&creep);
    }
}

// Utility functions
impl Unit {
    pub fn cost(&self) -> u32 {
        self.controller.get_body().iter().map(|p| p.cost()).sum()
    }
}

// Get the role a creep has
pub trait CreepRole {
    fn get_role(&self) -> UnitTypes;
}

impl CreepRole for Creep {
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

// Get creeps attached to this spawn
pub trait SpawnTeam {
    fn get_team(&self) -> Vec<Creep>;
}

// Get our team!
impl SpawnTeam for StructureSpawn {
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
