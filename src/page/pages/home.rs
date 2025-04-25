use eframe::egui::{self, Context};
use crate::entity::player::Player;

pub struct HomePage {
    pub player: Player,
}

impl HomePage {
    pub fn new(player: Player) -> Self {
        Self { player }
    }

    pub fn show(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to the Home Page!");
            ui.label(format!("Player Name: {}", self.player.name));
            ui.label(format!("Player Description: {}", self.player.description));
            ui.label(format!("Player Level: {}", self.player.level));
            ui.label(format!("Player Health: {}", self.player.health));
            ui.label(format!("Player Default Damage: {}", self.player.default_damage));
        });
    }
}

