// General utilities

use screeps::objects::{Creep, Source, Structure};
use screeps::{find, prelude::*, ResourceType};

pub trait CreepExtras {
    fn is_full(&self, res: ResourceType) -> bool;
    fn is_empty(&self, res: ResourceType) -> bool;
    fn nearest_source(&self) -> Source;
}

impl CreepExtras for Creep {
    // Return true if creep is full
    fn is_full(&self, res: ResourceType) -> bool {
        self.store_free_capacity(Some(res)) == 0
    }
    // Return true if creep is empty
    fn is_empty(&self, res: ResourceType) -> bool {
        self.store_used_capacity(Some(res)) == 0
    }
    // Look for the nearest energy source
    fn nearest_source(&self) -> Source {
        let my_pos = self.pos();
        self.room()
            .find(find::SOURCES)
            .into_iter()
            .min_by_key(|s| s.pos().get_range_to(&my_pos))
            .unwrap()
    }
}

pub trait StructureExtras {
    fn needs_repair(&self) -> bool;
}

impl StructureExtras for Structure {
    fn needs_repair(&self) -> bool {
        if let Some(structure) = self.as_attackable() {
            structure.hits_max() > structure.hits()
        } else {
            false
        }
    }
}

const TEAM: &'static str = "team";

pub trait TeamMates {
    fn get_team(&self) -> String;
    fn set_team(&self, team: &'static str);
}

impl TeamMates for Creep {
    fn set_team(&self, team: &'static str) {
        self.memory().set(TEAM, team);
    }
    fn get_team(&self) -> String {
        match self.memory().string(TEAM) {
            Ok(Some(team)) => team,
            _ => "default".to_string(),
        }
    }
}
