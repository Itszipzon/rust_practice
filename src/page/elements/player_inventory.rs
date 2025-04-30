use eframe::egui::{self, Context};

use crate::{appstate::AppState, item::item_type::ItemType};

use super::tooltip::ToolTip;

pub struct PlayerInventory {}

impl PlayerInventory {
  pub fn new() -> Self {
    PlayerInventory {}
  }

  pub fn show(&mut self, ctx: &Context, ui: &mut egui::Ui, app_state: &mut AppState) {
    let input = ctx.input(|i| i.clone());
    let columns = 10;
    let rows = 5;
    let slot_size = 50.0;

    // Handle number key shortcuts BEFORE rendering
    if app_state.player.has_cursor_item() {
      for (i, key) in [
        egui::Key::Num1,
        egui::Key::Num2,
        egui::Key::Num3,
        egui::Key::Num4,
        egui::Key::Num5,
        egui::Key::Num6,
        egui::Key::Num7,
        egui::Key::Num8,
        egui::Key::Num9,
        egui::Key::Num0,
      ]
      .iter()
      .enumerate()
      {
        if input.key_pressed(*key) {
          app_state.player.move_cursor_item_to_inventory(i);
        }
      }
    }

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
              .map(Clone::clone);

            let response = if let Some(item) = maybe_item {
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
                ToolTip::new().show(ctx, &item, index);
              }

              response
            } else {
              ui.add_sized([slot_size, slot_size], egui::Button::new(""))
            };

            if response.clicked() {
              if app_state.player.has_cursor_item() {
                app_state.player.move_cursor_item_to_inventory(index);
              } else {
                app_state.player.move_inventory_item_to_cursor(index);
              }
            }
          }
          ui.end_row();
        }
      });
  }
}
