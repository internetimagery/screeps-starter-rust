use crate::strategies::Strategy;
use screeps::objects::StructureSpawn;

pub trait StrategySpawn {
    fn recruit(&self, spawn: &StructureSpawn);
}

pub fn run_spawn(spawn: StructureSpawn) {
    let strategy = Strategy::from(&spawn);
    strategy.recruit(&spawn);
}
