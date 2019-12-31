// General utilities

use screeps::objects::{Creep, Source, Structure};
use screeps::{find, prelude::*, ResourceType};

// Enum that includes the ability to get it back by value
#[macro_export]
macro_rules! reversable_enum {
    ($enum_name: ident, $enum_type: ty, {$($name: ident = $value: expr,)+}) => {
        #[derive(Copy, Clone)]
        pub enum $enum_name {
            $($name = $value,)+
        }
        impl std::convert::TryFrom<$enum_type> for $enum_name {
            type Error = String;
            fn try_from(value: $enum_type) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($enum_name::$name), )+
                    _ => Err(format!("Unrecognized value {}", value)),
                }
            }
        }
    }
}

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

pub trait StructureExtras {
    fn needs_repair(&self) -> bool;
}

impl StructureExtras for Structure {
    fn needs_repair(&self) -> bool {
        if let Some(structure) = self.as_attackable() {
            structure.hits_max() > structure.hits()
        } else {
            false
        }
    }
}
