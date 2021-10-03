#![allow(unused_imports)]
#![allow(dead_code)]
use std::{collections::VecDeque, fs::File, path::Path, sync::Arc};

use app::TemplateApp;
use crossbeam::thread;
use eframe::NativeOptions;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use jwalk::{DirEntry, WalkDir};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::Write;

mod app;

fn create_db(drive: &str) {
    let drive = Path::new(drive);

    let db_name: String = drive.to_str().unwrap()[..1].to_string() + ".db";

    let mut buffer = File::create(Path::new(db_name.as_str())).unwrap();

    let newline: &[u8] = &['\n' as u8];

    for entry in WalkDir::new(drive).sort(true).skip_hidden(false) {
        buffer
            .write_all(
                &[
                    entry.as_ref().unwrap().path().to_string_lossy().as_bytes(),
                    newline,
                ]
                .concat(),
            )
            .unwrap();
    }

    return;
}
fn main() {
    let app = TemplateApp::default();
    eframe::run_native(Box::new(app), NativeOptions::default());
}
