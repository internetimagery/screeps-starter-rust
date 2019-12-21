use crate::units::Unit;
use log::*;
use screeps::objects::Creep;

pub trait UnitCreep {
    fn think(&self, creep: &Creep);
}

pub fn run_creep(creep: Creep) {
    debug!("Running creep {}", creep.name());
    let unit = Unit::from(&creep);
    unit.think(&creep);
}
