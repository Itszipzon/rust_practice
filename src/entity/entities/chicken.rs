use crate::entity::{entity::Entity, entity_kind::EntityKind};

#[derive(Debug, Clone)]
pub struct Chicken {
  description: String,
  is_alive: bool,
  health: u32,
  damage: u32,
  hostile: bool,
}

impl Chicken {
  pub fn new(description: String, health: u32, damage: u32, is_alive: bool, hostile: bool) -> Self {
    Chicken {
      description,
      health,
      damage,
      is_alive,
      hostile,
    }
  }

  pub fn is_alive(&self) -> bool {
    self.is_alive
  }

  pub fn get_health(&self) -> u32 {
    self.health
  }
}

impl Entity for Chicken {
  fn description(&self) -> String {
    self.description.clone()
  }

  fn id(&self) -> u32 {
    3
  }

  fn hostile(&self) -> bool {
    self.hostile
  }

  fn health(&self) -> u32 {
    self.health
  }

  fn take_damage(&mut self, damage: u32) {
    if damage >= self.health {
      self.health = 0;
      self.is_alive = false;
    } else {
      self.health -= damage;
    }
  }

  fn damage(&self) -> u32 {
    self.damage
  }

  fn entity_kind(&self) -> crate::entity::entity_kind::EntityKind {
    EntityKind::Chicken(self.clone())
  }
}

impl Default for Chicken {
  fn default() -> Self {
    Chicken {
      description: "Chicken".to_string(),
      is_alive: true,
      health: 4,
      damage: 0,
      hostile: false,
    }
  }
}
