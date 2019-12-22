// Units

use log::*;
use screeps::memory::MemoryReference;
use screeps::objects::{Creep, SpawnOptions, StructureSpawn};
use screeps::{game, prelude::*, Part, ReturnCode};

use crate::game_loop::UnitCreep;
use crate::strategies::UnitSpawn;

// Units
mod clumsy;
mod gatherer;
mod upgrader;

// Common fields
const ROLE: &'static str = "role";
const SPAWN: &'static str = "spawn";
const STATE: &'static str = "state";

// Unit type ID's
#[derive(Copy, Clone)]
pub enum UnitTypes {
    Gatherer = 0,
    Upgrader = 1,
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

// Create a new creep
impl UnitSpawn for Unit {
    fn create(&self, spawn: &StructureSpawn) -> Option<String> {
        let name = self.controller.get_name();
        let body = self.controller.get_body();
        let memory = self.controller.get_memory();
        memory.set(ROLE, self.unit_type);
        memory.set(SPAWN, &*spawn.id().to_array_string());
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
