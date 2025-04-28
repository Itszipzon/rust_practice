use std::fmt::Debug;

use crate::items::list::ItemList;

use super::equipment::equipment::Equipment;

pub trait Item: Debug {
  fn name(&self) -> String;
  fn description(&self) -> String;
  fn id(&self) -> u32;
  fn set_name(&mut self, name: String);
  fn item_type(&self) -> ItemList;

  fn as_equipment(&self) -> Option<&dyn Equipment>;
  fn as_equipment_mut(&mut self) -> Option<&mut dyn Equipment>;

  fn clone_box(&self) -> Box<dyn Item>;
}

impl Clone for Box<dyn Item> {
  fn clone(&self) -> Box<dyn Item> {
    self.clone_box()
  }
}
