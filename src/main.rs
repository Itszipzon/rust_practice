mod item;
mod items;
mod entity;
mod page;

use eframe::egui;
use entity::player::Player;
use items::sword::Sword;
use page::page::Page;
use page::pages::home::HomePage;
use page::pages::loading::LoadingPage;
use page::pages::settings::SettingsPage;

fn main() -> Result<(), eframe::Error> {
    let sword = Sword::stone_sword(Some(250));
    println!("Sword: {:?}", sword);
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
        
        
        let mut style: egui::Style = (*ctx.style()).clone();

        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(30.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(15.0, egui::FontFamily::Proportional)),
        ]
        .into();

        ctx.set_style(style);

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

        egui::CentralPanel::default().show(ctx, |_ui| {
            match self.current_page {
                Page::Loading => {self.loading_page.show(ctx, &mut self.player);}
                Page::Home => {self.home_page.show(ctx, &mut self.player);}
                Page::Settings => {self.settings_page.show(ctx, &mut self.player);}
            }
        });
    }
}
