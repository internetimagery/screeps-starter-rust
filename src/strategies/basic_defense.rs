// Build some basic defenses. Turrets, etc

use crate::strategies::StrategyController;
use screeps::objects::StructureSpawn;

pub struct BasicDefense {}

impl StrategyController for BasicDefense {
    fn recruit(&self, _: &StructureSpawn) {}
}
