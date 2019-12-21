// Strategy encapsulation

// Should change strategy automatically?
// Should be possible to change it in the console?
// strategy should handle spawning of troops and perhaps rally points etc

use crate::spawn::StrategySpawn;
use log::*;
use screeps::objects::StructureSpawn;

mod caveman;

// Allow units to spawn
pub trait UnitSpawn {
    fn create(&self, spawn: &StructureSpawn) -> Option<String>;
}

// Requirements for controllers
trait StrategyController {
    fn recruit(&self, spawn: &StructureSpawn);
}

// Wrap different strategies
pub struct Strategy {
    controller: Box<dyn StrategyController>,
}

// Get a Strategy from a spawn
impl From<&StructureSpawn> for Strategy {
    fn from(_: &StructureSpawn) -> Self {
        // todo: add some kind of chain-of-responsibility in here
        Strategy {
            controller: Box::new(caveman::Caveman {}),
        }
    }
}

// More troops!
impl StrategySpawn for Strategy {
    fn recruit(&self, spawn: &StructureSpawn) {
        debug!("Running spawn {}", spawn.name());
        self.controller.recruit(spawn)
    }
}
