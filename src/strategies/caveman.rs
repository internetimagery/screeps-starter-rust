// Starting from nothing. Build some basic units to gather materials and upgrade equally
use crate::strategies::{StrategyController, UnitSpawn};
use crate::units::{prelude::*, Unit, UnitTypes::*};
use log::*;
use screeps::objects::StructureSpawn;
use screeps::{find, prelude::*};

pub struct Caveman {
    pub next: Option<Box<dyn StrategyController>>,
}

impl StrategyController for Caveman {
    fn recruit(&self, spawn: &StructureSpawn) {
        // If we have built some dudes we can move onto something else
        let team = spawn.get_team();

        let mut upgraders = 0;
        let mut gatherers = 0;
        let mut builders = 0;

        for creep in &team {
            match creep.get_role() {
                Upgrader => upgraders += 1,
                Gatherer => gatherers += 1,
                Builder => builders += 1,
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
        // If we have some upgraders, get some builders
        } else if builders < spawn.room().find(find::CONSTRUCTION_SITES).len() + 2 {
            unit = Some(Unit::from(Builder));
        }

        if unit.is_some() && spawn.energy() >= unit.as_ref().unwrap().cost() {
            if let Some(unit_id) = unit.unwrap().create(&spawn) {
                info!("Created {}", unit_id);
            }
        }

        if team.len() > 4 && self.next.is_some() {
            self.next.as_ref().unwrap().recruit(spawn);
        }
    }
}
