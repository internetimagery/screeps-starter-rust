
use serde::{Serialize, Deserialize};
use std::collections::hash_map::{HashMap, Iter};

mod harvest;
pub mod prelude;

const QUEST: &'static str = "quest";
const BULLETIN: &'static str = "bulletin";

#[derive(Serialize, Deserialize)]
enum Quest {
    FillSpawn(harvest::FillSpawn),
}

// Hold all our quests!
#[derive(Serialize, Deserialize)]
pub struct BulletinBoard {
    id: u32, // Unique ID
    quests: HashMap<u32, Quest>, // Map ID to quest
}

impl BulletinBoard {
    fn new() -> Self {
        Self {id: 0, quests: HashMap::new()}
    }
    fn add_quest(&mut self, quest: Quest) -> u32 {
        self.id += 1;
        self.quests.insert(self.id, quest);
        self.id
    }
    fn get_quest(&self, id: u32) -> Option<&Quest> {
        self.quests.get(&id)
    }
    fn iter(&mut self) -> Iter<'_, u32, Quest> {
        self.quests.iter()
    }
}
