use eframe::egui::{self, Context};
use crate::entity::player::Player;
use crate::settings::Settings;

pub struct HomePage {
}

impl HomePage {
    pub fn new() -> Self {
        HomePage { }
    }

    pub fn show(&mut self, ctx: &Context, player: &mut Player, settings: &mut Settings) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to the Home Page!");
            ui.label(format!("Player Name: {}", player.name));
            ui.label(format!("Player Description: {}", player.description));
            ui.label(format!("Player Level: {}", player.level));
            ui.label(format!("Player Health: {}", player.health));
            ui.label(format!("Player Default Damage: {}", player.default_damage));
        });
    }
}

