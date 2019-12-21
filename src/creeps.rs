use log::*;
use screeps::objects::Creep;
use crate::units::Unit;

pub trait UnitCreep {
    fn think(&self, creep: &Creep);
}

pub fn run_creep(creep: Creep) {
    if creep.spawning() {
        return;
    }
    debug!("Running creep {}", creep.name());
    let unit = Unit::from(&creep);
    unit.think(&creep);
}
