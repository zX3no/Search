#![allow(unused_imports)]
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::process::Command;
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

//todo
type MS = Arc<RwLock<String>>;

pub struct App {
    search_query: String,
    last_search: String,
    search_result: Arc<RwLock<VecDeque<Index>>>,
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

    fn search(&self, query: String) {
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

            ui.columns(2, |columns| {
                columns[0].label("Name");
                columns[1].label("Path");
            });

            ui.add_space(4.0);

            let row_height = ui.fonts()[TextStyle::Body].row_height();
            let num_rows = search_result.read().unwrap().len();

            ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
                let file = search_result.read().unwrap();
                for row in row_range {
                    let file = file.get(row).unwrap();
                    ui.columns(2, |columns| {
                        //todo line wrapping does not work
                        //text does not stay in it's column
                        if columns[0].button(file.file_name.clone()).clicked() {
                            Command::new("explorer").arg(&file.path).spawn().unwrap();
                        };
                        if columns[1].button(file.path.clone()).clicked() {
                            Command::new("explorer").arg(&file.path).spawn().unwrap();
                        };
                    });
                }
            });
        });

        if search_query != last_search {
            let query = search_query.to_string().to_lowercase();
            self.last_search = query.clone();
            self.search(query);
        }
    }
}
