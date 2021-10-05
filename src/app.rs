#![allow(unused_imports)]
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::thread::{self};
use std::time::{Duration, Instant};

use eframe::egui::util::undoer::Settings;
use eframe::egui::*;
use eframe::{egui, epi};
use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::indexer::Indexer;

pub struct App {
    search_query: String,
    last_search: String,
    search_result: VecDeque<String>,
    index: Indexer,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            search_query: String::new(),
            last_search: String::from(" "),
            search_result: Default::default(),
            index: Indexer::new(),
        }
    }
}

impl App {
    pub fn scan_drive() -> VecDeque<PathBuf> {
        let mut out = VecDeque::new();
        for file in WalkDir::new(Path::new(r"C:\"))
            .sort(true)
            .skip_hidden(false)
        {
            if let Ok(f) = file {
                out.push_back(f.path())
            };
        }
        for file in WalkDir::new(Path::new(r"D:\"))
            .sort(true)
            .skip_hidden(false)
        {
            if let Ok(f) = file {
                out.push_back(f.path())
            };
        }
        return out;
    }
    pub fn search_files(files: VecDeque<PathBuf>, s: String) -> VecDeque<PathBuf> {
        let out: VecDeque<_> = files
            .par_iter()
            .filter_map(|f| {
                if f.to_string_lossy().contains(&s) {
                    return Some(f.clone());
                }
                return None;
            })
            .collect();
        return out;
    }
}
impl epi::App for App {
    fn name(&self) -> &str {
        "Search!"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        if self.index.is_empty() {
            thread::spawn(|| {
                Indexer::create();
            });
        }
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            search_query,
            last_search,
            search_result,
            index,
        } = self;

        if search_query != last_search && !index.is_empty() {
            //update the last search
            *last_search = search_query.clone();

            *search_result = index.search(search_query);
        }

        if index.is_empty() {
            // index.update();
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                //the height does not matter
                Vec2::new(ui.available_width(), 0.0),
                egui::TextEdit::singleline(search_query),
            );

            egui::warn_if_debug_build(ui);

            ui.add_space(10.0);

            //broken
            // ui.style_mut().spacing.scroll_bar_width = 13.0;
            let row_height = ui.fonts()[TextStyle::Body].row_height();
            let num_rows = search_result.len();

            ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
                if index.is_empty() {
                    ui.label("Please wait...");
                } else {
                    for row in row_range {
                        ui.label(search_result.get(row).unwrap());
                    }
                }
            });
        });
    }
}
