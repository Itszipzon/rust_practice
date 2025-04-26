use crate::{entity::player::Player, item::item::Item, page::page::Page, settings::Settings};

use std::vec;

use crate::items::sword::Sword;

pub struct AppState {
  pub player: Player,
  pub settings: Settings,
  pub current_page: Page,
}

impl AppState {
  pub fn new() -> Self {
    let mut player = Player::new("Hero".to_string(), "A brave hero.".to_string(), 10, 1, 100, 100);
    let wooden_sword = Sword::wooden_sword(Some(100));
    let stone_sword = Sword::stone_sword(Some(250));
    let iron_sword = Sword::iron_sword(Some(500));
    
    let swords: Vec<Box<dyn Item>> = vec![
      Box::new(wooden_sword),
      Box::new(stone_sword),
      Box::new(iron_sword),
    ];
    
    player.add_items(swords);

    println!("{:#?}", player.inventory);

    player.move_item(0, 5);
    println!("{:#?}", player.inventory);

    player.move_item(1, 5);
    println!("{:#?}", player.inventory);
    AppState {
      player,
      settings: Settings::load(),
      current_page: Page::Loading,
    }
  }
}