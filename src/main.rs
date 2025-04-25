mod entity;
mod item;
mod items;
mod page;
mod settings;

use std::vec;

use eframe::egui;
use entity::player::Player;
use items::sword::Sword;
use page::page::Page;
use page::pages::home::HomePage;
use page::pages::loading::LoadingPage;
use page::pages::settings::SettingsPage;
use settings::Settings;

fn main() -> Result<(), eframe::Error> {
    let wooden_sword = Sword::wooden_sword(Some(100));
    let stone_sword = Sword::stone_sword(Some(250));
    let iron_sword = Sword::iron_sword(Some(500));
    let swords = vec![wooden_sword, stone_sword, iron_sword];
    println!("Sword: {:#?}", swords);
    let options = eframe::NativeOptions::default();
    eframe::run_native("Game", options, Box::new(|_cc| Box::new(App::default())))
}

struct App {
    settings: Settings,
    player: Player,
    current_page: Page,
    loading_page: LoadingPage,
    home_page: HomePage,
    settings_page: SettingsPage,
}

impl Default for App {
    fn default() -> Self {
        let settings = Settings::load();
        let player = Player::new("Isaac".to_string(), "A crybaby".to_string(), 10, 1, 100, 50);
        Self {
            current_page: Page::Loading,
            player,
            settings,
            loading_page: LoadingPage::new(),
            home_page: HomePage::new(),
            settings_page: SettingsPage::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style: egui::Style = (*ctx.style()).clone();

        style.text_styles = [
            (
                egui::TextStyle::Heading,
                egui::FontId::new(30.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(15.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();

        ctx.set_style(style);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.current_page = Page::Home;
                }
                if ui.button("About").clicked() {
                    self.current_page = Page::Settings;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| match self.current_page {
            Page::Loading => {
                self.loading_page.show(
                    ctx,
                    &mut self.player,
                    &mut self.current_page,
                    &mut self.settings,
                );
            }

            Page::Home => {
                self.home_page.show(ctx, &mut self.player, &mut self.settings);
            }
            Page::Settings => {
                self.settings_page.show(ctx, &mut self.player, &mut self.settings);
            }
        });
    }
}
