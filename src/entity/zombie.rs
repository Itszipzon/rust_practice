use super::Entity;

#[derive(Debug)]
pub struct Zombie {
    id: u32,
    name: String,
    description: String,
    is_alive: bool,
    health: u32,
    damage: u32,
}

impl Zombie {
    pub fn new(id: u32, name: String, description: String, health: u32, damage: u32) -> Self {
        Zombie {
            id,
            name,
            description,
            health,
            damage,
            is_alive: true,
        }
    }

    pub fn attack(&self) -> u32 {
        self.damage
    }

    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.health {
            self.health = 0;
            self.is_alive = false;
        } else {
            self.health -= damage;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }
}

impl Entity for Zombie {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn entity_type(&self) -> String {
        "Zombie".to_string()
    }
}