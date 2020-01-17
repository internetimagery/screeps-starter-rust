// General utilities

use screeps::objects::Attackable;
use screeps::objects::{Creep, Source, Structure};
use screeps::{find, prelude::*, ResourceType};

pub trait StoreUtility: HasStore {
    // Return true if unit is full
    fn is_full(&self, res: ResourceType) -> bool {
        self.store_free_capacity(Some(res)) == 0
    }
    // Return true if unit is empty
    fn is_empty(&self, res: ResourceType) -> bool {
        self.store_used_capacity(Some(res)) == 0
    }
}

impl<T: HasStore> StoreUtility for T {}
impl StoreUtility for dyn HasStore {}

pub trait AttackableUtility: Attackable {
    // How much HP is lost?
    fn hits_lost(&self) -> u32 {
        self.hits_max() - self.hits()
    }
}

impl<T: Attackable> AttackableUtility for T {}
impl AttackableUtility for dyn Attackable {}

/////////// Refactor out below

pub trait CreepExtras {
    fn nearest_source(&self) -> Source;
}

impl CreepExtras for Creep {
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
        match self.as_attackable() {
            Some(structure) => structure.hits_max() > structure.hits(),
            None => false,
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

pub trait Registerable {
    fn save(&self, _: &Creep) {}
    fn execute(&self, creep: &Creep) -> bool;
}
