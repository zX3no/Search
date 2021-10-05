#![allow(unused_imports)]
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self};
use std::time::{Duration, Instant};

use eframe::egui::util::undoer::Settings;
use eframe::egui::*;
use eframe::{egui, epi};
use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::index::{self, Index};
use crate::indexer::Indexer;

type MS = Arc<RwLock<String>>;

pub struct App {
    search_query: String,
    last_search: String,
    search_result: Arc<RwLock<VecDeque<String>>>,
    indexer: Arc<RwLock<Indexer>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            search_query: String::new(),
            last_search: String::from(" "),
            search_result: Default::default(),
            indexer: Arc::new(RwLock::new(Indexer::new())),
        }
    }
}

impl App {
    fn run_indexer(&self) {
        //this should be empty so no harm done
        let indexer = self.indexer.clone();

        thread::spawn(move || {
            indexer.write().unwrap().update();
        });
    }

    fn search(&mut self, query: String) {
        self.last_search = self.search_query.clone();

        let search_result = self.search_result.clone();
        //oof chrome moment
        let indexer = self.indexer.clone();

        thread::spawn(move || {
            //should this event exist shouldn't we just show the results?
            //so much data :/
            *search_result.write().unwrap() = indexer.read().unwrap().search(&query);
        });
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
        self.run_indexer();
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            search_query,
            last_search,
            search_result,
            indexer,
        } = self;

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
                [ui.available_width(), 0.0],
                egui::TextEdit::singleline(search_query),
            );

            egui::warn_if_debug_build(ui);

            ui.add_space(4.0);

            if let Some(index) = &indexer.read().unwrap().index {
                let row_height = ui.fonts()[TextStyle::Body].row_height();
                //todo remove this
                let vec_queue: VecDeque<&Index> = VecDeque::from_iter(index);
                let num_rows = vec_queue.len() + 1;

                //todo swap to search results
                egui::Grid::new("heading").show(ui, |ui| {
                    ui.heading("Name");
                    ui.heading("Path");
                    ui.end_row();
                });
                ui.add_space(4.0);
                ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
                    egui::Grid::new("files").show(ui, |ui| {
                        for row in row_range {
                            let file = vec_queue.get(row).unwrap();
                            ui.label(file.file_name.clone());
                            ui.label(file.path.clone());
                            ui.end_row();
                        }
                    });
                });
            }
        });

        if search_query != last_search {
            let query = search_query.to_string();
            self.search(query);
        }
    }
}
