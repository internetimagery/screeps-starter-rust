// Build some basic defenses. Turrets, etc

use crate::strategies::{UnitSpawn, StrategyController};
use crate::units::{SpawnTeam, CreepRole, Unit, UnitTypes::Builder};
use screeps::objects::StructureSpawn;
use screeps::prelude::*;
use log::*;

pub struct SimpleBase {}

impl StrategyController for SimpleBase {
    fn recruit(&self, spawn: &StructureSpawn) {
        let mut builders = 0;

        for creep in spawn.get_team() {
            match creep.get_role() {
                Builder => builders += 1,
                _ => (),
            }
        }

        // TODO: Make some construction sites

        let mut unit = None;
        // Make sure we have some builders
        // TODO: Number of builders should increase when number of structures does
        if builders < 2 {
            unit = Some(Unit::from(Builder));
        }

        if unit.is_some() && spawn.energy() >= unit.as_ref().unwrap().cost() {
            if let Some(unit_id) = unit.unwrap().create(&spawn) {
                info!("Created {}", unit_id);
            }
        }
    }
}
