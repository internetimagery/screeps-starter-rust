// Higher level quests
use super::{Quest, QUEST};
use screeps::Creep;
use std::convert::{From, TryFrom};

// Helper methods exposed on the creep
pub trait CreepQuests {
    fn execute_quest(&self) -> bool;
    fn set_quest(&self, quest: Quest);
}

impl CreepQuests for Creep {
    fn execute_quest(&self) -> bool {
        if let Ok(action) = Quest::try_from(self) {
            if action.execute(self) {
                return true;
            }
            self.memory().set(QUEST, "lazy");
        }
        false
    }
    fn set_quest(&self, quest: Quest) {
        self.memory().set(QUEST, String::from(&quest));
        quest.save(self);
    }
}
