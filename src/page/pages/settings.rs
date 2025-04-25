use eframe::egui::{self, Context};
use crate::entity::player::Player;

pub struct SettingsPage {
}

impl SettingsPage {
    pub fn new() -> Self {
      SettingsPage {  }
    }

    pub fn show(&mut self, ctx: &Context, player: &mut Player) {
        egui::CentralPanel::default().show(ctx, |ui| {
          ui.heading("Settings");
          ui.label("Adjust your preferences here.");
        });
    }
}

