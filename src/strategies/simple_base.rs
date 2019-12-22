// Build some basic defenses. Turrets, etc

use crate::strategies::StrategyController;
use screeps::objects::StructureSpawn;

pub struct SimpleBase {}

impl StrategyController for SimpleBase {
    fn recruit(&self, _: &StructureSpawn) {}
}
