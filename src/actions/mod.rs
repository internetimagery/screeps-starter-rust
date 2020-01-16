// Simple repeditive actions

use prelude::*;
use screeps::Creep;
use serde::{Deserialize, Serialize};

mod build;
pub mod prelude;
mod transport;

const ACTION: &'static str = "action";

#[derive(Serialize, Deserialize)]
pub enum Action {
    HarvestEnergy(transport::HarvestEnergy),
    StoreEnergy(transport::StoreEnergy),
    BuildSite(build::BuildSite),
    RepairStructure(build::RepairStructure),
}

impl Action {
    fn execute(&self, creep: &Creep) -> bool {
        let result = match self {
            Self::HarvestEnergy(x) => x.execute(creep),
            Self::StoreEnergy(x) => x.execute(creep),
            Self::BuildSite(x) => x.execute(creep),
            Self::RepairStructure(x) => x.execute(creep),
        };
        if !result {
            creep.memory().set(ACTION, "{}");
        }
        result
    }
}

// Handle action execution and initializing
pub struct ActionProvider<'a, T> {
    source: &'a T,
}

impl ActionProvider<'_, Creep> {
    fn set_action(&self, action: Action) {
        self.source
            .memory()
            .set(ACTION, serde_json::to_string(&action).unwrap());
    }
    pub fn execute(&self) -> bool {
        if let Ok(Some(data)) = self.source.memory().string(ACTION) {
            if let Ok(action) = serde_json::from_str::<Action>(&data) {
                return action.execute(self.source);
            }
        }
        false
    }
}
