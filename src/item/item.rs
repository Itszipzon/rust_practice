use std::fmt::Debug;

use crate::items::list::ItemList;

use super::{equipment::equipment::Equipment, item_type::ItemType, placeable::placeable::Placeable};

pub trait Item: Debug {
  fn name(&self) -> String;
  fn description(&self) -> String;
  fn id(&self) -> u32;
  fn set_name(&mut self, name: String);
  fn item(&self) -> ItemList;

  fn as_equipment(&self) -> Option<&dyn Equipment>;
  fn as_equipment_mut(&mut self) -> Option<&mut dyn Equipment>;

  fn as_placeable(&self) -> Option<&dyn Placeable>;
  fn as_placeable_mut(&mut self) -> Option<&mut dyn Placeable>;

  fn clone_box(&self) -> Box<dyn Item>;
  fn item_type(&self) -> ItemType;
}

impl Clone for Box<dyn Item> {
  fn clone(&self) -> Box<dyn Item> {
    self.clone_box()
  }
}
