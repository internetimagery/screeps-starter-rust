// Simple repeditive actions

use crate::exception::Res;
use log::*;
use screeps::*;
use serde::{Deserialize, Serialize};

mod build;
pub mod prelude;
mod transport;

const ACTION: &'static str = "action";

// Resolve objectids
macro_rules! ok {
    ($name:expr) => {
        $name
            .try_resolve()?
            .ok_or_else(|| format!("Failed to resolve: {:?}\n{}:{}", $name, file!(), line!()))?
    };
}

pub enum ActionResult {
    Done,
    Continue,
    Abort,
}

// Register our actions
#[derive(Serialize, Deserialize)]
pub enum Action {
    // GoTo { location: Position },
    HarvestEnergy { source: ObjectId<Source> },
    StoreEnergy { store: ObjectId<Structure> },
    RenewLife { spawn: ObjectId<StructureSpawn> },
    BuildSite { site: ObjectId<ConstructionSite> },
    RepairStructure { target: ObjectId<Structure> },
}

impl Action {
    // Run by action provider
    fn execute(&self, creep: &Creep) -> Res<ActionResult> {
        use Action::*;
        match self {
            // GoTo { location } => transport::go_to(creep, location),
            HarvestEnergy { source } => transport::harvest_energy(creep, &ok!(source)),
            StoreEnergy { store } => transport::store_energy(creep, &ok!(store)),
            RenewLife { spawn } => transport::renew_life(creep, &ok!(spawn)),
            // BuildSite { site } => build::build_site(creep, &ok!(site)),
            BuildSite { site } => {
                // Temporary till more reliable construction logic, and assigned tasks are added
                match site.try_resolve() {
                    Ok(Some(site)) => build::build_site(creep, &site),
                    _ => Ok(ActionResult::Abort),
                }
            },
            RepairStructure { target } => build::repair_structure(creep, &ok!(target)),
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
            .set(ACTION, &serde_json::to_string(&action).unwrap());
    }
    // Use in game loop to progress actions another tick
    pub fn execute(&self) -> bool {
        if let Ok(Some(data)) = self.source.memory().string(ACTION) {
            if let Ok(action) = serde_json::from_str::<Action>(&data) {
                match action.execute(self.source) {
                    Ok(ActionResult::Continue) => return true,
                    Err(x) => warn!("Action failed: {:?}", x),
                    Ok(_) => (),
                }
            }
        }
        self.source.memory().set(ACTION, "{}");
        false
    }
}
