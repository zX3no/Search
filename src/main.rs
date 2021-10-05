use eframe::NativeOptions;

use crate::app::App;

mod app;

fn main() {
    let app = App::default();

    eframe::run_native(Box::new(app), NativeOptions::default());
    //Currently self.result cannot be shared across threads
    //this is bad because the memory needs to be copied
    //every time I want to do a search
}
