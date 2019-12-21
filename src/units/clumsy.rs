// Poor little guy.

use screeps::objects::Creep;
use screeps::Part;

use crate::units::UnitController;

pub struct Clumsy {}

impl UnitController for Clumsy {
    fn get_name(&self) -> &'static str {
        "Clumsy"
    }
    fn get_body(&self) -> &'static [Part] {
        &[Part::Move]
    }
    fn control_creep(&self, creep: &Creep) {
        creep.say("Oh I seem to have tripped on a bannana peel!", true);
        creep.suicide();
    }
}
