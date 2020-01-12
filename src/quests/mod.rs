
use screeps::Creep;
use crate::{prelude::*, register_for_creep};

mod prelude;
mod harvest;

const QUEST: &'static str = "quest";

register_for_creep! {
    (field=QUEST, name=Quest),
    HarvestEnergy(harvest::HarvestEnergy) = "harvest_energy",
}
