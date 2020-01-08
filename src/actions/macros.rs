// An action that specifies a target at creation time
// eg: source to harvest / building to build etc
#[macro_export]
macro_rules! action_target {
    (fn $func:ident(target: $contains:ty) -> $name:ident; fn execute(&$self:ident, $creep:ident: &Creep) -> bool $execute:block) => {
        pub struct $name {
            target: Option<$contains>
        }

        impl crate::actions::Action {
            pub fn $func(target: $contains) -> crate::actions::Action {
                crate::actions::Action::$name($name {
                    target: Some(target),
                })
            }
        }

        impl std::convert::From<&Creep> for $name {
            fn from(creep: &Creep) -> Self {
                Self {
                    target: get_id!(creep, crate::actions::TARGET),
                }
            }
        }

        impl crate::actions::Actionable for $name {
            fn save(&self, creep: &Creep) {
                if let Some(target) = &self.target {
                    set_id!(creep, crate::actions::TARGET, target);
                }
            }
            fn execute(&$self, $creep: &Creep) -> bool $execute
        }

    }
}
