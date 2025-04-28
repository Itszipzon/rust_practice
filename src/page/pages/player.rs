use crate::appstate::AppState;
use crate::page::page::Page;
use eframe::egui::{self, Context};

pub struct PlayerPage {
  current_index: usize,
}

impl PlayerPage {
  pub fn new() -> Self {
    PlayerPage { current_index: 0 }
  }

  pub fn show(&mut self, ctx: &Context, app_state: &mut AppState) {

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading(format!("{}'s Player Page", app_state.player.name));
      ui.label(format!("Player Name: {}", app_state.player.name));

      ui.separator();

      // Inventory Navigation
      ui.horizontal(|ui| {
        if ui
          .add_enabled(self.current_index > 0, egui::Button::new("< Left"))
          .clicked()
        {
          self.current_index -= 1;
        }

        ui.label(format!(
          "Item {}/{}",
          self.current_index + 1,
          app_state.player.inventory.len()
        ));

        if ui
          .add_enabled(
            self.current_index + 1 < app_state.player.inventory.len(),
            egui::Button::new("Right >"),
          )
          .clicked()
        {
          self.current_index += 1;
        }
      });

      ui.separator();

      // Inventory Item Display
      if let Some(slot) = app_state.player.inventory.get(self.current_index) {
        if let Some(item) = slot {
          // Display the item fields nicely
          ui.group(|ui| {
            ui.heading(&item.as_equipment().unwrap().dispaly_name());
            ui.label(format!("Name: {}", &item.name()));
            ui.label(format!("Description: {}", &item.as_equipment().unwrap().description()));
            ui.label(format!("Damage: {}", &item.as_equipment().unwrap().as_weapon().unwrap().damage()));
            ui.label(format!("Range: {}", &item.as_equipment().unwrap().as_weapon().unwrap().range()));
            ui.label(format!(
              "Durability: {}/{}",
              item.as_equipment().unwrap().durability(), item.as_equipment().unwrap().max_durability()
            ));
            ui.label(format!("Type: {:?}", item.item_type()));
          });
        } else {
          // Slot is None
          ui.label("Empty slot.");
        }
      } else {
        ui.label("Invalid inventory index!");
      }

      ui.separator();

      // Navigation buttons
      if ui.button("Go to Settings").clicked() {
        app_state.current_page = Page::Settings;
      }
      if ui.button("Go to Home").clicked() {
        app_state.current_page = Page::Home;
      }
    });
  }
}
