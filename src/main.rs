mod item;
mod items;
mod entity;
mod page;

use eframe::egui;
use entity::player::Player;
use item::equipment::equipment::Equipment;
use items::{list::ItemList, sword::Sword};
use page::page::Page;
use page::pages::home::HomePage;
use page::pages::loading::LoadingPage;
use page::pages::settings::SettingsPage;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Game", options, Box::new(|_cc| Box::new(App::default())))
}

struct App {
    player: Player,
    current_page: Page,
    loading_page: LoadingPage,
    home_page: HomePage,
    settings_page: SettingsPage,
}

impl Default for App {
    fn default() -> Self {
        let player = Player::new("Rune".to_string(), "Hero".to_string(), 10, 1, 100, 50);
        Self {
            current_page: Page::Loading,
            player,
            loading_page: LoadingPage::new(),
            home_page: HomePage::new(),
            settings_page: SettingsPage::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Loading").clicked() {
                    self.current_page = Page::Loading;
                }
                if ui.button("Home").clicked() {
                    self.current_page = Page::Home;
                }
                if ui.button("About").clicked() {
                    self.current_page = Page::Settings;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Loading => {self.loading_page.show(ctx, &mut self.player);}
                Page::Home => {self.home_page.show(ctx, &mut self.player);}
                Page::Settings => {self.settings_page.show(ctx, &mut self.player);}
            }
        });
    }
}
