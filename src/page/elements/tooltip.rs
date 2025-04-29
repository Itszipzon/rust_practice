use eframe::egui::{self, Context};

use crate::item::item::Item;

pub struct ToolTip {
}

impl ToolTip {

  pub fn new() -> Self {
    ToolTip {}
  }

  pub fn show(&mut self, ctx: &Context, item: &Box<dyn Item>, index: usize) {
    egui::show_tooltip_at_pointer(
      ctx,
      egui::Id::new(format!("tooltip_{}", index)),
      |ui| {
        ui.vertical(|ui| {
          if let Some(equip) = item.as_equipment() {
            ui.heading(equip.dispaly_name());
            ui.label(format!("Description: {}", equip.description()));
            ui.label(format!(
              "Durability: {}/{}",
              equip.durability(),
              equip.max_durability()
            ));
            if let Some(weapon) = equip.as_weapon() {
              ui.label(format!("Damage: {}", weapon.damage()));
              ui.label(format!("Range: {}", weapon.range()));
            }
          }
          ui.label(format!("Type: {:?}", item.item_type()));
        });
      },
    );
  }
  
}