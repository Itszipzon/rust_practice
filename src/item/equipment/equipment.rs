use crate::item::item::Item;
use super::equipment_type::equipment_type::Armor;
use super::equipment_type::equipment_type::Weapon;
use super::equipment_type::equipment_type::Shield;
use super::equipment_type::equipment_type::Tool;


pub trait Equipment: Item {
    fn name(&self) -> String;
    fn equipment_type(&self) -> String;
    fn durability(&self) -> u32;

    fn as_item(&self) -> Option<&dyn Item>;
    fn as_item_mut(&mut self) -> Option<&mut dyn Item>;

    fn as_armor(&self) -> Option<&dyn Armor>;
    fn as_armor_mut(&mut self) -> Option<&mut dyn Armor>;

    fn as_weapon(&self) -> Option<&dyn Weapon>;
    fn as_weapon_mut(&mut self) -> Option<&mut dyn Weapon>;

    fn as_shield(&self) -> Option<&dyn Shield>;
    fn as_shield_mut(&mut self) -> Option<&mut dyn Shield>;

    fn as_tool(&self) -> Option<&dyn Tool>;
    fn as_tool_mut(&mut self) -> Option<&mut dyn Tool>;
}