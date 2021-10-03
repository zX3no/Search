use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use eframe::egui::{color::*, *};
use eframe::{egui, epi};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use jwalk::WalkDir;
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

pub struct TemplateApp {
    // Example stuff:
    search: String,
    last_search: String,
    files: VecDeque<String>,
    drive: VecDeque<PathBuf>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            search: String::new(),
            last_search: String::from(" "),
            files: VecDeque::new(),
            drive: VecDeque::new(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Search!"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if self.drive.is_empty() {
            let (tx, rx) = mpsc::channel();
            println!("created channel");

            thread::spawn(move || {
                println!("scaning drive");
                let mut data = VecDeque::new();
                for file in WalkDir::new(Path::new(r"C:\"))
                    .sort(true)
                    .skip_hidden(false)
                {
                    if let Ok(f) = file {
                        data.push_back(f.path())
                    };
                }
                tx.send(data).unwrap();
            });
            println!("checking");
            self.drive = rx.recv().unwrap();
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        if self.search != self.last_search {
            println!("starting search");
            self.files = VecDeque::new();
            self.last_search = self.search.clone();

            for file in &self.drive {
                let data = file.to_string_lossy();
                if data.contains(&self.search) || self.search == "" {
                    self.files.push_back(data.to_string());
                }
            }

            println!("done search");
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            //TODO make this bigger
            ui.text_edit_singleline(&mut self.search);

            egui::warn_if_debug_build(ui);

            ui.add_space(4.0);

            let row_height = ui.fonts()[TextStyle::Body].row_height();
            let num_rows = self.files.len();

            ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
                for row in row_range {
                    ui.label(self.files.get(row).unwrap());
                }
            });
        });
    }
}
