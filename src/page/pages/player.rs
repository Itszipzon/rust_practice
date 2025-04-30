use crate::item::item_type::ItemType;
use crate::page::page::Page;
use crate::{appstate::AppState, page::elements::tooltip::ToolTip};
use eframe::egui::{self, Context};

pub struct PlayerPage {}

impl PlayerPage {
  pub fn new() -> Self {
    PlayerPage {}
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
          .spacing([10.0, 10.0])
          .show(ui, |ui| {
            for row in 0..rows {
              for col in 0..columns {
                let index = row * columns + col;

                let maybe_item = app_state
                  .player
                  .inventory
                  .get(index)
                  .and_then(|slot| slot.as_ref())
                  .map(|item| item.clone());

                if let Some(item) = maybe_item {
                  let display_name = match item.item_type() {
                    ItemType::Equipment => item.as_equipment().unwrap().display_name(),
                    ItemType::Placeable => item.as_placeable().unwrap().display_name(),
                  };

                  let truncated_name = if display_name.len() > 6 {
                    format!("{}...", &display_name[..3])
                  } else {
                    display_name
                  };

                  let response = ui.add_sized(
                    [slot_size, slot_size],
                    egui::Button::new(truncated_name).min_size([slot_size, slot_size].into()),
                  );

                  if response.hovered() {
                    let mut tooltip = ToolTip::new();
                    tooltip.show(ctx, &item, index);
                  }

                  if response.clicked() {
                    if app_state.player.has_cursor_item() {
                      app_state.player.move_cursor_item_to_inventory(index);
                    } else {
                      app_state.player.move_inventory_item_to_cursor(index);
                    }
                  }
                } else {
                  let response = ui.add_sized([slot_size, slot_size], egui::Button::new(""));
                  if response.clicked() {
                    if app_state.player.has_cursor_item() {
                      app_state.player.move_cursor_item_to_inventory(index);
                    }
                  }
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
