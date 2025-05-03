use crate::entity::{entity::Entity, entity_kind::EntityKind};

#[derive(Debug, Clone)]
pub struct Cow {
  description: String,
  is_alive: bool,
  health: u32,
  damage: u32,
}

impl Cow {
  pub fn new(description: String, health: u32) -> Self {
    Cow {
      description,
      health,
      damage: 0,
      is_alive: true,
    }
  }

  pub fn is_alive(&self) -> bool {
    self.is_alive
  }

  pub fn get_health(&self) -> u32 {
    self.health
  }
}

impl Entity for Cow {
  fn description(&self) -> String {
    format!("Chicken")
  }

  fn id(&self) -> u32 {
    2
  }

  fn hostile(&self) -> bool {
    false
  }

  fn health(&self) -> u32 {
    6
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
    0
  }

  fn entity_kind(&self) -> crate::entity::entity_kind::EntityKind {
    EntityKind::Cow(self.clone())
  }
}
