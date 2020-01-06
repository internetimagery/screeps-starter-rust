// Create an enum to "register" types, associate them with related structs and serialize / deserialize on creeps
// Include an "id" that can be converted to and from
// for use in serialization

#[macro_export]
macro_rules! register_for_creep {
    ((field=$field:expr, name=$enum_name:ident), $($name: ident($logic:ty) = $value: expr,)+) => {
        pub enum $enum_name {
            $($name($logic),)+
        }

        impl $enum_name {
            fn save(&self, creep: &Creep) {
                match self {
                    $($enum_name::$name(x) => x.save(creep),)+
                }
            }
            fn execute(&self, creep: &Creep) -> bool {
                match self {
                    $($enum_name::$name(x) => x.execute(creep),)+
                }
            }
        }

        impl std::convert::From<&$enum_name> for String {
            fn from(value: &$enum_name) -> Self {
                match value {
                    $($enum_name::$name(_) => $value.to_string(),)+
                }
            }
        }

        impl std::convert::TryFrom<&Creep> for $enum_name {
            type Error = String;
            fn try_from(creep: &Creep) -> Result<Self, Self::Error> {
                use std::convert::From;
                if let Ok(Some(id)) = creep.memory().string($field) {
                    return match id.as_ref() {
                        $($value => Ok($enum_name::$name(<$logic>::from(creep))),)+
                        x => Err(format!("Unknown $enum_name {}", x)),
                    }
                }
                Err("No action available".to_string())
            }
        }
    }
}

#[macro_export]
macro_rules! get_id {
    ($creep:expr, $key:expr) => {
        {
            use std::str::FromStr;
            match $creep.memory().string($key) {
                Ok(Some(id)) => {
                    match screeps::ObjectId::from_str(&id) {
                        Ok(object_id) => {
                            match object_id.try_resolve() {
                                Ok(Some(obj)) => Some(obj),
                                _ => None,
                            }
                        },
                        _ => None,
                    }
                },
                _ => None,
            }
        }
    }
}

#[macro_export]
macro_rules! set_id {
    ($creep:expr, $key:expr, $obj:expr) => {
        $creep.memory().set($key, $obj.id().to_string())
    }
}

// TODO: Replace this with something cleaner
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
