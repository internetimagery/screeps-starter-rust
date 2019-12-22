// Starting from nothing. Build some basic units to gather materials and upgrade equally
use crate::strategies::{StrategyController, UnitSpawn};
use crate::units::{SpawnTeam, Unit, UnitTypes::*, ROLE};
use log::*;
use screeps::objects::StructureSpawn;
use screeps::prelude::*;

pub struct Caveman {
    pub next: Option<Box<dyn StrategyController>>,
}

impl StrategyController for Caveman {
    fn recruit(&self, spawn: &StructureSpawn) {
        // If we have built some dudes we can move onto something else
        let team = spawn.get_team();
        if self.next.is_some() && team.len() > 5 {
            self.next.as_ref().unwrap().recruit(spawn);
            return;
        }

        let mut upgraders = 0;
        let mut gatherers = 0;
        for creep in team {
            match creep.memory().i32(ROLE) {
                Ok(Some(c)) if c == Upgrader as i32 => upgraders += 1,
                Ok(Some(c)) if c == Gatherer as i32 => gatherers += 1,
                _ => (),
            }
        }
        let mut unit = None;
        // Make sure we build a couple gatherers first
        if gatherers < 2 {
            unit = Some(Unit::from(Gatherer));
        // Once we have some gatherers lets get some upgraders
        } else if upgraders < 2 {
            unit = Some(Unit::from(Upgrader));
        // If we have some upgraders, get some more gatherers
        } else if gatherers < 5 {
            unit = Some(Unit::from(Gatherer));
        }

        if unit.is_some() && spawn.energy() >= unit.as_ref().unwrap().cost() {
            if let Some(unit_id) = unit.unwrap().create(&spawn) {
                info!("Created {}", unit_id);
            }
        }
    }
}
