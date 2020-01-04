// Simple repeditive actions

use crate::register_for_creep;
use prelude::*;
use screeps::Creep;

pub mod prelude;
mod transport;

const ACTION: &'static str = "action";

// Register actions with their associated logic and serialized IDs
register_for_creep! {
    (field=ACTION, name=Action),
    HarvestEnergy(transport::HarvestEnergy) = 1,
}
