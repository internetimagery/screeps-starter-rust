// Higher level quests
use screeps::{prelude::*, Creep, Room};
use super::{BulletinBoard, BULLETIN};

pub trait QuestTrait {
    // Number indicating if the creep should pick up this quest or not
    fn achievable(&self, creep: &Creep) -> Option<u32>;
}

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
        let mut board = self.room().get_bulletin();
        if let Some(quest) = board.iter().max_by_key(|q| q.1.achievable(self)) {
            self.memory().set("TEST", &quest.0);
            return true
        }
        false
    }
}
