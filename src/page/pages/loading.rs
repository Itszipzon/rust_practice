use eframe::egui::{self, Context};
use crate::{entity::player::Player, page::page::Page, settings::Settings};

use super::settings;

pub struct LoadingPage {
    loaded: bool,
}

impl LoadingPage {
    pub fn new() -> Self {
        LoadingPage { loaded: false }
    }

    pub fn show(&mut self, ctx: &Context, player: &mut Player, page: &mut Page, settings: &mut Settings) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Loading...");
            ui.label(format!("Loading game for player: {}", player.name));
            ui.label(format!("Applying settings: {:?}", settings));
        });

        if !self.loaded {
            // You can apply settings here (e.g., adjust player, theme, etc.)
            println!("Settings loaded: {:?}", settings);

            // Once loading is "done", switch to Home
            *page = Page::Home;
            self.loaded = true;
        }
    }
}

