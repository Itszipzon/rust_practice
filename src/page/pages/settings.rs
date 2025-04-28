use crate::appstate::AppState;
use crate::page::page::Page;
use eframe::egui::{self, Context};

pub struct SettingsPage {}

impl SettingsPage {
  pub fn new() -> Self {
    SettingsPage {}
  }

  pub fn show(&mut self, ctx: &Context, app_state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading(format!("{}'s Settings", app_state.player.name));
      ui.label("Adjust your preferences here.");
      ui.button("Go to Home").clicked().then(|| {
        app_state.current_page = Page::Home;
      });
      ui.button("Go to Player").clicked().then(|| {
        app_state.current_page = Page::Player;
      });
    });
  }
}
