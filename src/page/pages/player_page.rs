use crate::page::elements::player_inventory::PlayerInventory;
use crate::page::page::Page;
use crate::appstate::AppState;
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

      PlayerInventory::new().show(ctx, ui, app_state);

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
