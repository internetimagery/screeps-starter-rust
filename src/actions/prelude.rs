// Simple repeditive actions
use super::ActionProvider;
use screeps::Creep;

pub trait ActionExecute {
    fn execute(&self, creep: &Creep) -> bool;
}

pub trait ActionProviderTrait<T> {
    fn actions(&self) -> ActionProvider<'_, T>;
}

impl ActionProviderTrait<Creep> for Creep {
    fn actions(&self) -> ActionProvider<'_, Creep> {
        ActionProvider { source: self }
    }
}
