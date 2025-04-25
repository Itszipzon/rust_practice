use eframe::egui::{self, Context};
use crate::entity::player::Player;

pub struct LoadingPage {
}

impl LoadingPage {
    pub fn new() -> Self{
        LoadingPage { }
    }

    pub fn show(&mut self, ctx: &Context, player: &mut Player) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to the Loading Page");
            ui.label(format!("Loading game for player: {}", player.name));
        });
    }
}

