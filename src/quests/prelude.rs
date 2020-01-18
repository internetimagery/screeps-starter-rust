// Higher level quests
use screeps::{prelude::*, Creep, Room};
use super::{BulletinBoard, BULLETIN};


pub trait RoomBulletin {
    fn get_bulletin(&self) -> BulletinBoard;
    fn set_bulletin(&self, board: &BulletinBoard);
}

// Attach a bulletin board to a room
impl RoomBulletin for Room {
    fn get_bulletin(&self) -> BulletinBoard {
        if let Ok(Some(data)) = self.memory().string(BULLETIN) {
            if let Ok(board) = serde_json::from_str(&data) {
                return board
            }
        }
        BulletinBoard::new()
    }
    fn set_bulletin(&self, board: &BulletinBoard) {
        self.memory().set(BULLETIN, serde_json::to_string(board).unwrap());
    }
}

pub trait CreepBulletin {
    fn pick_up_quest(&self) -> bool;
}

// Pick up a quest suited to the creep. Return true if a quest was picked up
impl CreepBulletin for Creep {
    fn pick_up_quest(&self) -> bool {
        let board = self.room().get_bulletin();
        // todo: loop through quests and pick best one if one exists
        // if so, assign the quest to the creep.
        false
    }
}




// // Helper methods exposed on the creep
// pub trait CreepQuests {
//     fn execute_quest(&self) -> bool;
//     fn set_quest(&self, quest: Quest);
// }
//
// impl CreepQuests for Creep {
//     fn execute_quest(&self) -> bool {
//         if let Ok(action) = Quest::try_from(self) {
//             if action.execute(self) {
//                 return true;
//             }
//             self.memory().set(QUEST, "lazy");
//         }
//         false
//     }
//     fn set_quest(&self, quest: Quest) {
//         self.memory().set(QUEST, String::from(&quest));
//         quest.save(self);
//     }
// }
