use crate::item::{
  equipment::{
    equipment::Equipment, equipment_type::equipment_type::{Armor, Shield, Tool, Weapon},
  },
  item::Item, item_type::ItemType,
};

use super::list::ItemList;

#[derive(Debug, Clone)]
pub struct Sword {
  id: u32,
  name: String,
  display_name: String,
  description: String,
  damage: u32,
  range: u32,
  max_durability: u32,
  durability: u32,
  item_type: ItemList,
}

impl Sword {
  pub fn new(
    description: String,
    damage: u32,
    range: u32,
    max_durability: u32,
    durability: u32,
    item_type: ItemList,
  ) -> Self {
    Sword {
      display_name: item_type.name().to_string(),
      name: item_type.name().to_string(),
      description,
      damage,
      range,
      max_durability,
      durability,
      item_type,
      id: item_type.id(),
    }
  }

  /// Creates a new Wooden Sword instance with the given durability.
  /// If no durability is provided, it defaults to 100.
  /// # Arguments
  /// * `durability` - An optional u32 representing the durability of the sword. None means default durability.
  /// # Values
  /// * `damage`: 10
  /// * `range`: 1
  /// * `max_durability`: 100
  /// * `durability`: 100 (default)
  /// # Returns
  /// A new Sword instance with the specified durability.
  /// # Examples
  /// ```
  /// let sword = Sword::wooden_sword(None); // Defaults to 100 durability
  /// let sword = Sword::wooden_sword(Some(75)); // 75 durability
  /// ```
  pub fn wooden_sword(durability: Option<u32>) -> Self {
    let max_durability = 100;
    let mut durability = durability.unwrap_or(max_durability);
    if durability > max_durability {
      durability = max_durability;
    }
    Sword::new(
      "A sturdy wooden sword.".to_string(),
      10,
      1,
      max_durability,
      durability,
      ItemList::WoodenSword,
    )
  }

  /// Creates a new Stone Sword instance with the given durability.
  /// If no durability is provided, it defaults to 150.
  /// # Arguments
  /// * `durability` - An optional u32 representing the durability of the sword. None means default durability.
  /// # Values
  /// * `damage`: 12
  /// * `range`: 1
  /// * `max_durability`: 150
  /// * `durability`: 150 (default)
  /// # Returns
  /// A new Sword instance with the specified durability.
  /// # Examples
  /// ```
  /// let sword = Sword::stone_sword(None); // Defaults to 150 durability
  /// let sword = Sword::stone_sword(Some(100)); // 100 durability
  /// ```
  pub fn stone_sword(durability: Option<u32>) -> Self {
    let max_durability = 150;
    let mut durability = durability.unwrap_or(max_durability);
    if durability > max_durability {
      durability = max_durability;
    }
    Sword::new(
      "A sturdy stone sword.".to_string(),
      12,
      1,
      max_durability,
      durability,
      ItemList::StoneSword,
    )
  }

  /// Creates a new Iron Sword instance with the given durability.
  /// If no durability is provided, it defaults to 250.
  /// # Arguments
  /// * `durability` - An optional u32 representing the durability of the sword. None means default durability.
  /// # Values
  /// * `damage`: 15
  /// * `range`: 1
  /// * `max_durability`: 250
  /// * `durability`: 250 (default)
  /// # Returns
  /// A new Sword instance with the specified durability.
  /// # Examples
  /// ```
  /// let sword = Sword::iron_sword(None); // Defaults to 100 durability
  /// let sword = Sword::iron_sword(Some(150)); // 150 durability
  /// ```
  pub fn iron_sword(durability: Option<u32>) -> Self {
    let mut durability = durability.unwrap_or(250);
    let max_durability = 250;
    if durability > max_durability {
      durability = max_durability;
    }
    Sword::new(
      "A sturdy iron sword.".to_string(),
      15,
      1,
      max_durability,
      durability,
      ItemList::IronSword,
    )
  }
}

impl Weapon for Sword {
  fn damage(&self) -> u32 {
    self.damage
  }

  fn range(&self) -> u32 {
    self.range
  }

  fn as_equipment(&self) -> Option<&dyn Equipment> {
    Some(self as &dyn Equipment)
  }

  fn as_equipment_mut(&mut self) -> Option<&mut dyn Equipment> {
    Some(self as &mut dyn Equipment)
  }

  fn on_use(&mut self) -> u32 {
    if self.durability > 0 {
      self.durability -= 1;
    }
    self.durability
  }
}

impl Equipment for Sword {
  fn display_name(&self) -> String {
    self.name.clone()
  }

  fn equipment_type(&self) -> String {
    "Sword".to_string()
  }

  fn durability(&self) -> u32 {
    self.durability
  }

  fn as_item(&self) -> Option<&dyn Item> {
    Some(self as &dyn Item)
  }

  fn as_armor(&self) -> Option<&dyn Armor> {
    None
  }

  fn as_weapon(&self) -> Option<&dyn Weapon> {
    Some(self as &dyn Weapon)
  }

  fn as_shield(&self) -> Option<&dyn Shield> {
    None
  }

  fn as_tool(&self) -> Option<&dyn Tool> {
    None
  }

  fn as_item_mut(&mut self) -> Option<&mut dyn Item> {
    Some(self as &mut dyn Item)
  }

  fn as_armor_mut(&mut self) -> Option<&mut dyn Armor> {
    None
  }

  fn as_weapon_mut(&mut self) -> Option<&mut dyn Weapon> {
    Some(self as &mut dyn Weapon)
  }

  fn as_shield_mut(&mut self) -> Option<&mut dyn Shield> {
    None
  }

  fn as_tool_mut(&mut self) -> Option<&mut dyn Tool> {
    None
  }

  fn max_durability(&self) -> u32 {
    self.max_durability
  }
}

impl Item for Sword {
  fn name(&self) -> String {
    self.display_name.clone()
  }

  fn description(&self) -> String {
    self.description.clone()
  }

  fn id(&self) -> u32 {
    self.id
  }

  fn set_name(&mut self, name: String) {
    self.display_name = name;
  }

  fn as_equipment(&self) -> Option<&dyn Equipment> {
    Some(self as &dyn Equipment)
  }

  fn as_equipment_mut(&mut self) -> Option<&mut dyn Equipment> {
    Some(self as &mut dyn Equipment)
  }

  fn item(&self) -> ItemList {
    self.item_type.clone()
  }

  fn item_type(&self) -> ItemType {
    ItemType::Equipment
  }

  fn clone_box(&self) -> Box<dyn Item> {
    Box::new(self.clone())
  }
  
  fn as_placeable(&self) -> Option<&dyn crate::item::placeable::placeable::Placeable> {
        None
    }
  
  fn as_placeable_mut(&mut self) -> Option<&mut dyn crate::item::placeable::placeable::Placeable> {
        None
    }
}
