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

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Game", options, Box::new(|_cc| Box::new(App::default())))
}

struct App {
    current_page: Page,
    home_page: HomePage,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_page: Page::Loading,
            home_page: HomePage::new(Player::new("Rune".to_string(), "Hero".to_string(), 10, 1, 100, 50)),
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
                Page::Loading => {
                    ui.heading("Welcome to the Loading Page");
                    ui.label("Some home page content.");
                }
                Page::Home => {self.home_page.show(ctx);}
                Page::Settings => {
                    ui.heading("Settings");
                    ui.label("Adjust your preferences here.");
                }
            }
        });
    }
}
