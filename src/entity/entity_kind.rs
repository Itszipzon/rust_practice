use super::{entities::{chicken::Chicken, cow::Cow, player::Player, skeleton::Skeleton, zombie::Zombie}, entity::Entity};

macro_rules! entity_kind {
    ( $( $variant:ident => $type:path ),* $(,)? ) => {
        #[derive(Debug, Clone)]
        pub enum EntityKind {
            $( $variant($type), )*
        }

        impl Entity for EntityKind {
            fn description(&self) -> String {
                match self {
                    $( EntityKind::$variant(inner) => inner.description(), )*
                }
            }

            fn id(&self) -> u32 {
                match self {
                    $( EntityKind::$variant(inner) => inner.id(), )*
                }
            }

            fn hostile(&self) -> bool {
                match self {
                    $( EntityKind::$variant(inner) => inner.hostile(), )*
                }
            }

            fn health(&self) -> u32 {
                match self {
                    $( EntityKind::$variant(inner) => inner.health(), )*
                }
            }

            fn damage(&self) -> u32 {
                match self {
                    $( EntityKind::$variant(inner) => inner.damage(), )*
                }
            }

            fn entity_kind(&self) -> EntityKind {
                match self {
                    $( EntityKind::$variant(inner) => EntityKind::$variant(inner.clone()), )*
                }
            }
        }
    };
}

entity_kind! {
    Player => Player,
    Chicken => Chicken,
    Cow => Cow,
    Skeleton => Skeleton,
    Zombie => Zombie,
}