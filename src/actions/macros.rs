// Create an enum to "register" actions, and associate them with related structs (for logic)
// Include an "id" that can be converted to and from
// for use in serialization

macro_rules! register_actions {
    ($($name: ident($logic:ty) = $value: expr,)+) => {
        pub enum Action {
            $($name($logic),)+
        }

        impl Action {
            fn save(&self, creep: &Creep) {
                match self {
                    $(Action::$name(x) => x.save(creep),)+
                }
            }
            fn execute(&self, creep: &Creep) -> bool {
                match self {
                    $(Action::$name(x) => x.execute(creep),)+
                }
            }
        }

        impl std::convert::From<&Action> for i32 {
            fn from(value: &Action) -> Self {
                match value {
                    $(Action::$name(_) => $value,)+
                }
            }
        }

        impl std::convert::TryFrom<&Creep> for Action {
            type Error = String;
            fn try_from(creep: &Creep) -> Result<Self, Self::Error> {
                if let Ok(Some(id)) = creep.memory().i32(ACTION) {
                    return match id {
                        $($value => Ok(Action::$name(<$logic>::load(creep))),)+
                        x => Err(format!("Unknown Action {}", x)),
                    }
                }
                Err("No action available".to_string())
            }
        }
    }
}
