use crate::entity::{entity::Entity, entity_kind::EntityKind};

#[derive(Debug, Clone)]
pub struct Skeleton {
  description: String,
  is_alive: bool,
  health: u32,
  damage: u32,
  hostile: bool,
}

impl Skeleton {
  pub fn new(description: String, health: u32, damage: u32, is_alive: bool, hostile: bool) -> Self {
    Skeleton {
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

  pub fn attack(&self, mut target: impl Entity) -> u32 {
    target.take_damage(self.damage);
    self.damage
  }
}

impl Entity for Skeleton {
  fn description(&self) -> String {
    self.description.clone()
  }

  fn id(&self) -> u32 {
    4
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
    EntityKind::Skeleton(self.clone())
  }
}

impl Default for Skeleton {
  fn default() -> Self {
    Skeleton::new("Skeleton".to_string(), 6, 1, true, true)
  }
}