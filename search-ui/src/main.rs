mod app;
use app::App;
use eframe::NativeOptions;
fn main() {
    let app = App::default();
    eframe::run_native(Box::new(app), NativeOptions::default());
}
