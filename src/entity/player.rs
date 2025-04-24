use crate::item::Item;
use super::Entity;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub description: String,
    pub default_damage: u32,
    pub level: u32,
    pub health: u32,
    pub mana: u32,
    pub inventory: Vec<Option<Box<dyn Item>>>,
    pub active_item: u32,
    pub is_alive: bool,
}

impl Player {
    pub fn new(name: String, description: String, default_damage: u32, level: u32, health: u32, mana: u32) -> Self {

      let inventory = (0..50).map(|_| None).collect();

        Player {
            name,
            description,
            default_damage,
            level,
            health,
            mana,
            inventory,
            active_item: 0,
            is_alive: true,
        }
    }

    pub fn add_item(&mut self, item: Box<dyn Item>) {
        if let Some(index) = self.next_non_null_inventory_index() {
            self.inventory[index] = Some(item);
        }
    }

    pub fn remove_item(&mut self, item: &Box<dyn Item>) {
      if (self.inventory.len() > 0) {
        for i in 0..self.inventory.len() {
          if let Some(inventory_item) = &self.inventory[i] {
            if inventory_item.id() == item.id() && inventory_item.name() == item.name() && inventory_item.as_equipment().unwrap().as_item().unwrap().name() == item.as_equipment().unwrap().as_item().unwrap().name() {
              self.inventory[i] = None;
              break;
            }
          }
        }
      }
    }

    pub fn move_item(&mut self, item: &Box<dyn Item>, old_index: usize, new_index: usize) {
      if (new_index < self.inventory.len()) {
        if (self.inventory.get(new_index).is_none()) {
          self.inventory[new_index] = self.inventory[old_index].take();
          self.inventory.remove(old_index);
        } else {
          let positional_item = &self.inventory[new_index];
        }
      }
    }

    pub fn attack(&mut self) -> u32 {
      if (self.active_is_weapon()) {
        let weapon = &mut self.inventory[self.active_item as usize];
        weapon.as_mut().unwrap().as_equipment_mut().unwrap().as_weapon_mut().unwrap().on_use();
          return self.default_damage + weapon.as_mut().unwrap().as_equipment_mut().unwrap().as_weapon_mut().unwrap().damage();
      }

      self.default_damage
        
    }

    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.health {
            self.health = 0;
            self.is_alive = false;
        } else {
            self.health -= damage;
        }
    }

    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn set_mana(&mut self, mana: u32) {
        self.mana = mana;
    }

    pub fn get_mana(&self) -> u32 {
        self.mana
    }

    pub fn set_active_item(&mut self, active_item: u32) {
        if active_item < 10 {
            self.active_item = active_item;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn active_is_weapon(&self) -> bool {
        if self.inventory.len() > 0 {
            let item = &self.inventory[self.active_item as usize];
            if (item.as_ref().unwrap().as_equipment().unwrap().as_weapon().is_some()) {
                return true;
            }
        }
        false
    }

    fn next_non_null_inventory_index(&self) -> Option<usize> {
        for i in 0..self.inventory.len() {
            if self.inventory[i].is_none() {
                return Some(i);
            }
        }
        None
    }


}

impl Entity for Player {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn id(&self) -> u32 {
        0
    }

    fn entity_type(&self) -> String {
        "Player".to_string()
    }
}