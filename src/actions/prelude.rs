// Simple repeditive actions
use super::ActionProvider;
use screeps::{Creep, StructureTower};

pub trait ActionExecute<T> {
    fn execute(&self, unit: &T) -> bool;
}

pub trait ActionProviderTrait<T> {
    fn actions(&self) -> ActionProvider<'_, T>;
}

impl ActionProviderTrait<Creep> for Creep {
    fn actions(&self) -> ActionProvider<'_, Creep> {
        ActionProvider { source: self }
    }
}

impl ActionProviderTrait<StructureTower> for StructureTower {
    fn actions(&self) -> ActionProvider<'_, StructureTower> {
        ActionProvider { source: self }
    }
}
