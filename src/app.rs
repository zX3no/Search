#![allow(unused_imports)]
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::thread::{self};
use std::time::{Duration, Instant};

use eframe::egui::util::undoer::Settings;
use eframe::egui::*;
use eframe::{egui, epi};
use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::index::{self, Index};
use crate::indexer::Indexer;

pub struct App {
    search_query: String,
    last_search: String,
    search_result: VecDeque<String>,
    indexer: Indexer,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            search_query: String::new(),
            last_search: String::from(" "),
            search_result: Default::default(),
            indexer: Indexer::new(),
        }
    }
}

impl App {}
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
        // if self.index.is_empty() {
        //     thread::spawn(|| {
        //         Indexer::create();
        //     });
        // }
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            search_query,
            last_search,
            search_result,
            indexer,
        } = self;

        if search_query != last_search && !indexer.is_empty() {
            //update the last search
            *last_search = search_query.clone();

            *search_result = indexer.search(search_query);
        }

        if indexer.is_empty() {
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
                [ui.available_width(), 0.0],
                egui::TextEdit::singleline(search_query),
            );

            egui::warn_if_debug_build(ui);

            ui.add_space(10.0);

            if let Some(index) = &indexer.index {
                let row_height = ui.fonts()[TextStyle::Body].row_height();
                //todo remove this
                let vec_queue: VecDeque<&Index> = VecDeque::from_iter(index);
                let num_rows = vec_queue.len();

                ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        for row in row_range {
                            let file = vec_queue.get(row).unwrap();
                            ui.label(file.file_name.clone());
                            ui.label(file.path.clone());
                            ui.end_row();
                        }
                    });
                });
            }
            //broken
            // ui.style_mut().spacing.scroll_bar_width = 13.0;
            // let row_height = ui.fonts()[TextStyle::Body].row_height();
            // let num_rows = search_result.len();

            // ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
            //     if index.is_empty() {
            //         ui.label("Please wait...");
            //     } else {
            //         for row in row_range {
            //             ui.label(search_result.get(row).unwrap());
            //         }
            //     }
            // });
        });
    }
}
