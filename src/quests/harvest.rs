// Harvest energy!
use crate::prelude::*;
use screeps::Creep;

pub struct HarvestEnergy {}

impl Registerable for HarvestEnergy {
    fn execute(&self, _: &Creep) -> bool {
        false
    }
}

impl From<&Creep> for HarvestEnergy {
    fn from(_: &Creep) -> Self {
        Self {}
    }
}
