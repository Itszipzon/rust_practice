use crate::appstate::AppState;
use crate::page::page::Page;
use eframe::egui::{self, Context};

pub struct HomePage {}

impl HomePage {
  pub fn new() -> Self {
    HomePage {}
  }

  pub fn show(&mut self, ctx: &Context, app_state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Welcome to the Home Page!");
      ui.label(format!("Player Name: {}", app_state.player.name));
      ui.label(format!(
        "Player Description: {}",
        app_state.player.description
      ));
      ui.label(format!("Player Level: {}", app_state.player.level));
      ui.label(format!("Player Health: {}", app_state.player.health));
      ui.label(format!(
        "Player Default Damage: {}",
        app_state.player.default_damage
      ));
      ui.button("Go to Settings").clicked().then(|| {
        app_state.current_page = Page::Settings;
      });
      ui.button("Go to Player").clicked().then(|| {
        app_state.current_page = Page::Player;
      });
    });
  }
}
