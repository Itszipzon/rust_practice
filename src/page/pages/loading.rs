use eframe::egui::{self, Context};
use crate::{appstate::AppState, page::page::Page};

pub struct LoadingPage {
    loaded: bool,
}

impl LoadingPage {
    pub fn new() -> Self {
        LoadingPage { loaded: false }
    }

    pub fn show(&mut self, ctx: &Context, app_state: &mut AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Loading...");
            ui.label(format!("Loading game for player: {}", app_state.player.name));
            ui.label(format!("Applying settings: {:?}", app_state.settings));
        });

        if !self.loaded {
            // You can apply settings here (e.g., adjust player, theme, etc.)
            println!("Settings loaded: {:?}", app_state.settings);

            // Once loading is "done", switch to Home
            app_state.current_page = Page::Home;
            self.loaded = true;
        }
    }
}

