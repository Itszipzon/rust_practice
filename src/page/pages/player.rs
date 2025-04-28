use crate::appstate::AppState;
use crate::page::page::Page;
use eframe::egui::{self, Context};

pub struct PlayerPage {}

impl PlayerPage {
  pub fn new() -> Self {
    PlayerPage {  }
  }

  pub fn show(&mut self, ctx: &Context, app_state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading(format!("{}'s Player Page", app_state.player.name));
      ui.label(format!("Player Name: {}", app_state.player.name));

      ui.separator();

      let columns = 10;
      let rows = 5;
      let slot_size = 50.0;

      ui.vertical_centered(|ui| {
        egui::Grid::new("inventory_grid")
          .spacing([10.0, 10.0]) // spacing between cells
          .show(ui, |ui| {
            for row in 0..rows {
              for col in 0..columns {
                let index = row * columns + col;
                if let Some(slot) = app_state.player.inventory.get(index) {
                  if let Some(item) = slot {
                    // Get display name
                    let display_name = item
                      .as_equipment()
                      .map(|eq| eq.dispaly_name())
                      .unwrap_or_else(|| item.name());

                    let truncated_name = if display_name.len() > 6 {
                      format!("{}...", &display_name[..3])
                    } else {
                      display_name
                    };

                    let response =
                      ui.add_sized([slot_size, slot_size], egui::Button::new(truncated_name).min_size([slot_size, slot_size].into()));

                    if response.hovered() {
                      // Tooltip on hover
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
                  } else {
                    ui.add_sized([slot_size, slot_size], egui::Button::new(""));
                  }
                } else {
                  ui.add_sized([slot_size, slot_size], egui::Button::new(""));
                }
              }
              ui.end_row();
            }
          });
      });

      ui.separator();

      if ui.button("Go to Settings").clicked() {
        app_state.current_page = Page::Settings;
      }
      if ui.button("Go to Home").clicked() {
        app_state.current_page = Page::Home;
      }
    });
  }
}
