use std::sync::{Arc, RwLock};

use eframe::NativeOptions;

use crate::app::App;
use std::thread;

mod app;

// fn create_db(drive: &str) {
//     let drive = Path::new(drive);

//     let db_name: String = drive.to_str().unwrap()[..1].to_string() + ".db";

//     let mut buffer = File::create(Path::new(db_name.as_str())).unwrap();

//     let newline: &[u8] = &['\n' as u8];

//     for entry in WalkDir::new(drive).sort(true).skip_hidden(false) {
//         buffer
//             .write_all(
//                 &[
//                     entry.as_ref().unwrap().path().to_string_lossy().as_bytes(),
//                     newline,
//                 ]
//                 .concat(),
//             )
//             .unwrap();
//     }

//     return;
// }

fn main() {
    //On Start
    //Start the UI
    //Start the drive scan in a new thread
    //Once the thread has finished put the drive into memory
    //Display the drive using the search query
    let app = App::default();

    eframe::run_native(Box::new(app), NativeOptions::default());
}
