use crate::entity::{entity::Entity, entity_kind::EntityKind};

#[derive(Debug, Clone)]
pub struct Cow {
  description: String,
  is_alive: bool,
  health: u32,
  damage: u32,
  hostile: bool,
}

impl Cow {
  pub fn new() -> Self {
    Cow {
      description: "Cow".to_string(),
      health: 6,
      damage: 0,
      is_alive: true,
      hostile: false,
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
    self.description.clone()
  }

  fn id(&self) -> u32 {
    2
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
    EntityKind::Cow(self.clone())
  }
}
