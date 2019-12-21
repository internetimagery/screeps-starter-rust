// Poor little guy.

use screeps::objects::Creep;
use screeps::Part;

use crate::units::UnitController;

pub struct Zombie {}

impl UnitController for Zombie {
    fn get_name(&self) -> &'static str {
        "Zombie"
    }
    fn get_body(&self) -> &'static [Part] {
        &[Part::Move]
    }
    fn control_creep(&self, creep: &Creep) {
        creep.say("Goodbye cruel world!", true);
        creep.suicide();
    }
}
