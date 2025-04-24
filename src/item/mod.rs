use std::fmt::Debug;
pub mod equipment;

use equipment::Equipment;

pub trait Item: Debug {
  fn name (&self) -> String;
  fn description (&self) -> String;
  fn id (&self) -> u32;
  fn set_name (&mut self, name: String);

  fn as_equipment(&self) -> Option<&dyn Equipment>;
  fn as_equipment_mut(&mut self) -> Option<&mut dyn Equipment>;
}