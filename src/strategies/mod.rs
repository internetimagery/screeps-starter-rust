// Strategy encapsulation

// Should change strategy automatically?
// Should be possible to change it in the console?
// strategy should handle spawning of troops and perhaps rally points etc
// perhaps strategy could also handle some creeps by setting memory data on them

use log::*;
pub use management::{manage_forces, StrategySpawn, UnitCreep};
use screeps::objects::StructureSpawn;

mod caveman;
mod management;
mod simple_base;

// Allow units to spawn
pub trait UnitSpawn {
    fn create(&self, spawn: &StructureSpawn) -> Option<String>;
}

// Requirements for controllers
pub trait StrategyController {
    fn recruit(&self, spawn: &StructureSpawn);
}

// Wrap different strategies
pub struct Strategy {
    controller: Box<dyn StrategyController>,
}

// Get a Strategy from a spawn
impl From<&StructureSpawn> for Strategy {
    fn from(_: &StructureSpawn) -> Self {
        let strategy_chain = caveman::Caveman {
            next: Some(Box::new(simple_base::SimpleBase {})),
        };
        Strategy {
            controller: Box::new(strategy_chain),
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
