// Simple repeditive actions

use prelude::*;
use screeps::{Creep};
use serde::{Deserialize, Serialize};

mod build;
pub mod prelude;
mod transport;

const ACTION: &'static str = "action";

// Register our actions
#[derive(Serialize, Deserialize)]
pub enum Action {
    HarvestEnergy(transport::HarvestEnergy),
    StoreEnergy(transport::StoreEnergy),
    BuildSite(build::BuildSite),
    RepairStructure(build::RepairStructure),
    RenewLife(transport::RenewLife),
}

impl Action {
    // Run by action provider
    fn execute(&self, creep: &Creep) -> bool {
        match self {
            Self::HarvestEnergy(x) => x.execute(creep),
            Self::StoreEnergy(x) => x.execute(creep),
            Self::BuildSite(x) => x.execute(creep),
            Self::RepairStructure(x) => x.execute(creep),
            Self::RenewLife(x) => x.execute(creep),
        }
    }
}

// Handle action execution and initializing
pub struct ActionProvider<'a, T> {
    source: &'a T,
}

impl ActionProvider<'_, Creep> {
    // Use internally by initialization functions
    fn set_action(&self, action: Action) {
        self.source
            .memory()
            .set(ACTION, serde_json::to_string(&action).unwrap());
    }
    // Use in game loop to progress actions another tick
    pub fn execute(&self) -> bool {
        if let Ok(Some(data)) = self.source.memory().string(ACTION) {
            if let Ok(action) = serde_json::from_str::<Action>(&data) {
                if action.execute(self.source) {
                    return true;
                }
            }
        }
        self.source.memory().set(ACTION, "{}");
        false
    }
}
