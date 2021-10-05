use eframe::NativeOptions;

use crate::app::App;
use crate::indexer::Indexer;
mod app;
mod indexer;

fn main() {
    let buf = &[0x41u8, 0x41u8, 0x42u8];

    let s = match std::str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);

    // Indexer::create();
    // Indexer::read();

    // let app = App::default();

    // eframe::run_native(Box::new(app), NativeOptions::default());
    //Currently self.result cannot be shared across threads
    //this is bad because the memory needs to be copied
    //every time I want to do a search

    //Bugs
    //Scroll bar in really smol
    //If the scroll bar width is increased, it out grows the clickable area
    //Scroll speed is way to slow
    //hangs on printing search results

    //Data that needs to be cached
    //File path
    //File name, can extrapolate from file path because we have .is_dir?
    //File size
    //date modified
}
