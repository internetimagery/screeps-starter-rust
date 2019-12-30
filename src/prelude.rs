// General utilities

use screeps::objects::{Creep, Source};
use screeps::{find, prelude::*, ResourceType};

pub trait CreepExtras {
    fn is_full(&self, res: ResourceType) -> bool;
    fn is_empty(&self, res: ResourceType) -> bool;
    fn nearest_source(&self) -> Source;
}

impl CreepExtras for Creep {
    // Return true if creep is full
    fn is_full(&self, res: ResourceType) -> bool {
        self.store_free_capacity(Some(res)) == 0
    }
    // Return true if creep is empty
    fn is_empty(&self, res: ResourceType) -> bool {
        self.store_used_capacity(Some(res)) == 0
    }
    // Look for the nearest energy source
    fn nearest_source(&self) -> Source {
        let my_pos = self.pos();
        self.room()
            .find(find::SOURCES)
            .into_iter()
            .min_by_key(|s| s.pos().get_range_to(&my_pos))
            .unwrap()
    }
}
