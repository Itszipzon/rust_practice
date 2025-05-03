use eframe::egui;
use page::page::Page;
use page::pages::home::HomePage;
use page::pages::loading::LoadingPage;
use page::pages::settings::SettingsPage;
use page::pages::player_page::PlayerPage;

use crate::{appstate, page};

pub struct App {
  app_state: appstate::AppState,
  loading_page: LoadingPage,
  home_page: HomePage,
  settings_page: SettingsPage,
  player_page: PlayerPage,
}

impl App {
  pub fn new(app_state: appstate::AppState) -> Self {
    Self {
      app_state,
      loading_page: LoadingPage::new(),
      home_page: HomePage::new(),
      settings_page: SettingsPage::new(),
      player_page: PlayerPage::new(),
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
        egui::FontId::new(14.0, egui::FontFamily::Proportional),
      ),
    ]
    .into();

    ctx.set_style(style);

    /*         egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Home").clicked() {
                self.current_page = Page::Home;
            }
            if ui.button("About").clicked() {
                self.current_page = Page::Settings;
            }
        });
    }); */

    egui::CentralPanel::default().show(ctx, |_ui| match self.app_state.current_page {
      Page::Loading => {
        self.loading_page.show(ctx, &mut self.app_state);
      }

      Page::Home => {
        self.home_page.show(ctx, &mut self.app_state);
      }
      Page::Settings => {
        self.settings_page.show(ctx, &mut self.app_state);
      }
      Page::Player => {
        self.player_page.show(ctx, &mut self.app_state);
      }
    });
  }
  fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
    self.app_state.settings.save();
  }
}
