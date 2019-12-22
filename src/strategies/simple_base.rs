// Build some basic defenses. Turrets, etc

use crate::strategies::{StrategyController, UnitSpawn};
use crate::units::{CreepRole, SpawnTeam, Unit, UnitTypes::*};
use log::*;
use screeps::objects::StructureSpawn;
use screeps::{find, prelude::*};

pub struct SimpleBase {}

impl StrategyController for SimpleBase {
    fn recruit(&self, spawn: &StructureSpawn) {
        let mut miners = 0;

        for creep in spawn.get_team() {
            match creep.get_role() {
                Miner => miners += 1,
                _ => (),
            }
        }

        // TODO: Make some construction sites

        let mut unit = None;
        // At least one miner per energy source
        if miners < spawn.room().find(find::SOURCES).len() * 2 {
            unit = Some(Unit::from(Miner));
        }

        if unit.is_some() && spawn.energy() >= unit.as_ref().unwrap().cost() {
            if let Some(unit_id) = unit.unwrap().create(&spawn) {
                info!("Created {}", unit_id);
            }
        }
    }
}
