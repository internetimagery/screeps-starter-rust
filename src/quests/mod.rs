use crate::{prelude::*, register_for_creep};
use screeps::Creep;

mod harvest;
mod prelude;

const QUEST: &'static str = "quest";

register_for_creep! {
    (field=QUEST, name=Quest),
    HarvestEnergy(harvest::HarvestEnergy) = "harvest_energy",
}
