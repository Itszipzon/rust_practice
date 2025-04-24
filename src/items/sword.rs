use crate::item::{
    Item,
    equipment::{Equipment, equipment_type::Weapon},
};

use super::list::ItemList;

#[derive(Debug)]
pub struct Sword {
    id: u32,
    name: String,
    display_name: String,
    description: String,
    damage: u32,
    range: u32,
    durability: u32,
    item_type: ItemList,
}

impl Sword {
    pub fn new(description: String, damage: u32, range: u32, durability: u32, item_type: ItemList) -> Self {
        Sword {
            display_name: item_type.name().to_owned(),
            name: item_type.name().to_owned(),
            description,
            damage,
            range,
            durability,
            item_type,
            id: item_type.id(),
        }
    }
}

impl Weapon for Sword {
    fn damage(&mut self) -> u32 {
        self.damage
    }

    fn range(&self) -> u32 {
        self.range
    }

    fn as_equipment(&self) -> Option<&dyn crate::item::equipment::Equipment> {
        Some(self as &dyn crate::item::equipment::Equipment)
    }

    fn as_equipment_mut(&mut self) -> Option<&mut dyn crate::item::equipment::Equipment> {
        Some(self as &mut dyn crate::item::equipment::Equipment)
    }

    fn on_use(&mut self) -> u32 {
      if self.durability > 0 {
          self.durability -= 1;
      }
      self.durability
    }
}

impl Equipment for Sword {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn equipment_type(&self) -> String {
        "Sword".to_string()
    }

    fn durability(&self) -> u32 {
        self.durability
    }

    fn as_item(&self) -> Option<&dyn crate::item::Item> {
        Some(self as &dyn crate::item::Item)
    }

    fn as_armor(&self) -> Option<&dyn crate::item::equipment::equipment_type::Armor> {
        None
    }

    fn as_weapon(&self) -> Option<&dyn Weapon> {
        Some(self as &dyn Weapon)
    }

    fn as_shield(&self) -> Option<&dyn crate::item::equipment::equipment_type::Shield> {
        None
    }

    fn as_tool(&self) -> Option<&dyn crate::item::equipment::equipment_type::Tool> {
        None
    }
    
    fn as_item_mut(&mut self) -> Option<&mut dyn Item> {
        Some(self as &mut dyn Item)
    }
    
    fn as_armor_mut(&mut self) -> Option<&mut dyn crate::item::equipment::equipment_type::Armor> {
        None
    }
    
    fn as_weapon_mut(&mut self) -> Option<&mut dyn Weapon> {
        Some(self as &mut dyn Weapon)
    }
    
    fn as_shield_mut(&mut self) -> Option<&mut dyn crate::item::equipment::equipment_type::Shield> {
        None
    }
    
    fn as_tool_mut(&mut self) -> Option<&mut dyn crate::item::equipment::equipment_type::Tool> {
        None
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
}
