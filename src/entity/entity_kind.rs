use super::{entities::{chicken::Chicken, cow::Cow, player::Player, skeleton::Skeleton, zombie::Zombie}, entity::Entity};

#[derive(Debug)]
pub enum EntityKind {
    Zombie(Zombie),
    Player(Player),
    Cow(Cow),
    Chicken(Chicken),
    Skeleton(Skeleton)
}

impl Entity for EntityKind {
    fn description(&self) -> String {
        match self {
            EntityKind::Zombie(z) => z.description(),
            EntityKind::Player(p) => p.description(),
            EntityKind::Cow(c) => c.description(),
            EntityKind::Chicken(ch) => ch.description(),
            EntityKind::Skeleton(s) => s.description(),
        }
    }

    fn id(&self) -> u32 {
        match self {
            EntityKind::Zombie(z) => z.id(),
            EntityKind::Player(p) => p.id(),
            EntityKind::Cow(c) => c.id(),
            EntityKind::Chicken(ch) => ch.id(),
            EntityKind::Skeleton(s) => s.id(),
        }
    }

    fn hostile(&self) -> bool {
        match self {
            EntityKind::Zombie(z) => z.hostile(),
            EntityKind::Player(p) => p.hostile(),
            EntityKind::Cow(c) => c.hostile(),
            EntityKind::Chicken(ch) => ch.hostile(),
            EntityKind::Skeleton(s) => s.hostile(),
        }
    }

    fn health(&self) -> u32 {
        match self {
            EntityKind::Zombie(z) => z.health(),
            EntityKind::Player(p) => p.health(),
            EntityKind::Cow(c) => c.health(),
            EntityKind::Chicken(ch) => ch.health(),
            EntityKind::Skeleton(s) => s.health(),
        }
    }

    fn damage(&self) -> u32 {
        match self {
            EntityKind::Zombie(z) => z.damage(),
            EntityKind::Player(p) => p.damage(),
            EntityKind::Cow(c) => c.damage(),
            EntityKind::Chicken(ch) => ch.damage(),
            EntityKind::Skeleton(s) => s.damage(),
        }
    }

    fn entity_kind(&self) -> EntityKind {
        match self {
            EntityKind::Zombie(z) => EntityKind::Zombie(z.clone()),
            EntityKind::Player(p) => EntityKind::Player(p.clone()),
            EntityKind::Cow(c) => EntityKind::Cow(c.clone()),
            EntityKind::Chicken(ch) => EntityKind::Chicken(ch.clone()),
            EntityKind::Skeleton(s) => EntityKind::Skeleton(s.clone()),
        }
    }
}