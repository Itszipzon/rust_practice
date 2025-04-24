use crate::item::equipment::equipment::Equipment;

pub trait Armor: Equipment {
    fn defence(&self) -> u32;
    fn on_use(&self) -> u32;

    fn as_equipment(&self) -> Option<&dyn Equipment>;
    fn as_equipment_mut(&mut self) -> Option<&mut dyn Equipment>;
}

pub trait Weapon: Equipment {
    fn damage(&mut self) -> u32;
    fn range(&self) -> u32;
    fn on_use(&mut self) -> u32;

    fn as_equipment(&self) -> Option<&dyn Equipment>;
    fn as_equipment_mut(&mut self) -> Option<&mut dyn Equipment>;
}

pub trait Shield: Equipment {
    fn defence(&self) -> u32;
    fn on_use(&mut self) -> u32;

    fn as_equipment(&self) -> Option<&dyn Equipment>;
    fn as_equipment_mut(&mut self) -> Option<&mut dyn Equipment>;
}

pub trait Tool: Equipment {
    fn uses(&self) -> u32;
    fn durability(&self) -> u32;
    fn on_use(&mut self) -> u32;

    fn as_equipment(&self) -> Option<&dyn Equipment>;
    fn as_equipment_mut(&mut self) -> Option<&mut dyn Equipment>;
}