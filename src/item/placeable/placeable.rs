use crate::item::item::Item;

pub trait Placeable: Item {
  fn display_name(&self) -> String;
  fn amount(&self) -> u32;

  fn as_item(&self) -> Option<&dyn Item>;
  fn as_item_mut(&mut self) -> Option<&mut dyn Item>;
}