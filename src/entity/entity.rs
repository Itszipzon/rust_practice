use std::fmt::Debug;

use super::entity_kind::EntityKind;

pub trait Entity: Debug {
  
  fn description(&self) -> String;
  fn id(&self) -> u32;
  fn hostile(&self) -> bool;
  fn health(&self) -> u32;
  fn damage(&self) -> u32;
  fn take_damage(&mut self, damage: u32);

  fn entity_kind(&self) -> EntityKind;
}