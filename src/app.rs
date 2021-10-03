use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

use eframe::egui::{color::*, *};
use eframe::{egui, epi};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use jwalk::WalkDir;
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    search: String,
    last_search: String,
    files: VecDeque<String>,
    drive: VecDeque<Box<PathBuf>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            search: String::new(),
            last_search: String::new(),
            files: VecDeque::new(),
            drive: VecDeque::new(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "egui template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        if self.drive.is_empty() {
            println!("scaning drive");
            for file in WalkDir::new(Path::new(r"C:\")) {
                if let Ok(f) = file {
                    self.drive.push_back(Box::new(f.path()))
                };
            }
        }

        if self.search != self.last_search {
            println!("starting search");
            self.files = VecDeque::new();
            self.last_search = self.search.clone();

            let matcher = SkimMatcherV2::default();
            for file in &self.drive {
                if let Some(score) = matcher.fuzzy_match(file.to_str().unwrap(), &self.search) {
                    if score > 50 {
                        //TODO crossbeam sort
                        self.files.push_back(file.to_string_lossy().to_string());
                    }
                }
            }

            println!("done search");
            // let file = File::open("C.db").expect("no such file");
            // let buf = BufReader::new(file);
            // for line in buf.lines() {
            //     if line.as_ref().unwrap().contains(&search) {
            //         if let Err(e) = send.send(line.unwrap().to_string()) {
            //             println!("{}", e);
            //         }
            //     }
            // }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.search);

            egui::warn_if_debug_build(ui);

            ui.add_space(4.0);

            let row_height = ui.fonts()[TextStyle::Body].row_height();
            let num_rows = self.files.len();

            ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
                // let safe_ui = Arc::new(Mutex::new(ui));
                // let matcher = SkimMatcherV2::default();
                for row in row_range {
                    ui.label(self.files.get(row).unwrap());
                }
                // self.files.par_iter().for_each(|file| {});
                // let iter = self.file_name.par_iter().for_each(|path| {
                //     if let Some(hit) = matcher.fuzzy_match(path, "among us") {
                //         if hit > 100 {
                //             let data = *path.clone();
                //             safe_ui.lock().unwrap().label(data);
                //         }
                //     };
            });
        });
    }
}
