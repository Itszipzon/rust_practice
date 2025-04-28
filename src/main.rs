mod app;
mod appstate;
mod entity;
mod item;
mod items;
mod page;
mod settings;
use app::App;

fn main() -> Result<(), eframe::Error> {
  let options = eframe::NativeOptions::default();
  let app_state = appstate::AppState::new();

  eframe::run_native(
    "Game",
    options,
    Box::new(|_cc| Box::new(App::new(app_state))),
  )
}
