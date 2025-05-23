use crate::entity::{entity::Entity, entity_kind::EntityKind};

#[derive(Debug, Clone)]
pub struct Zombie {
  description: String,
  is_alive: bool,
  health: u32,
  damage: u32,
  hostile: bool,
}

impl Zombie {
  pub fn new(description: String, health: u32, damage: u32, is_alive: bool, hostile: bool) -> Self {
    Zombie {
      description,
      health,
      damage,
      is_alive,
      hostile,
    }
  }

  pub fn attack(&self, mut target: impl Entity) -> u32 {
    target.take_damage(self.damage);
    self.damage
  }

  pub fn is_alive(&self) -> bool {
    self.is_alive
  }

  pub fn get_health(&self) -> u32 {
    self.health
  }
}

impl Entity for Zombie {
  fn description(&self) -> String {
    format!("Zombie: {}", self.description)
  }

  fn id(&self) -> u32 {
    1
  }

  fn hostile(&self) -> bool {
    self.hostile
  }

  fn health(&self) -> u32 {
    self.health
  }

  fn damage(&self) -> u32 {
    self.damage
  }

  fn take_damage(&mut self, damage: u32) {
    if damage >= self.health {
      self.health = 0;
      self.is_alive = false;
    } else {
      self.health -= damage;
    }
  }

  fn entity_kind(&self) -> EntityKind {
    EntityKind::Zombie(self.clone())
  }
}

impl Default for Zombie {
  fn default() -> Self {
    Zombie::new("Zombie".to_string(), 6, 2, true, true)
  }
}
