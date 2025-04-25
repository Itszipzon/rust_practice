use eframe::egui::{self, Context};
use crate::entity::player::Player;
use crate::settings::Settings;

pub struct SettingsPage {
}

impl SettingsPage {
    pub fn new() -> Self {
      SettingsPage {  }
    }

    pub fn show(&mut self, ctx: &Context, player: &mut Player, settings: &mut Settings) {
        egui::CentralPanel::default().show(ctx, |ui| {
          ui.heading(format!("{}'s Settings", player.name));
          ui.label("Adjust your preferences here.");
        });
    }
}