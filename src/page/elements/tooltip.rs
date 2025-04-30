use eframe::egui::{self, Context};

use crate::item::{item::Item, item_type::ItemType};

pub struct ToolTip {}

impl ToolTip {
  pub fn new() -> Self {
    ToolTip {}
  }

  pub fn show(&mut self, ctx: &Context, item: &Box<dyn Item>, index: usize) {
    let layer_id = egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("tooltip_layer"));
    egui::show_tooltip_at_pointer(ctx, layer_id, egui::Id::new(format!("tooltip_{}", index)), |ui| {
      ui.vertical(|ui| {
        let display_name = match item.item_type() {
          ItemType::Equipment => item.as_equipment().unwrap().display_name(),
          ItemType::Placeable => item.as_placeable().unwrap().display_name(),
        };

        ui.heading(display_name);
        ui.label(format!("Description: {}", item.description()));
        if item.item_type() == ItemType::Equipment {

          ui.label(format!(
            "Durability: {}/{}",
            item.as_equipment().unwrap().durability(),
            item.as_equipment().unwrap().max_durability()
          ));

          if let Some(weapon) = item.as_equipment().unwrap().as_weapon() {
            ui.label(format!("Damage: {}", weapon.damage()));
            ui.label(format!("Range: {}", weapon.range()));
          }
        }

        ui.label(format!("Type: {:?}", item.item()));
      });
    });
  }
}
