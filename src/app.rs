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

pub struct App {
    //async drive scan
    in_progress: Option<Receiver<VecDeque<PathBuf>>>,
    result: Option<VecDeque<PathBuf>>,
    trigger_fetch: bool,
    //search
    search_in_progress: Option<Receiver<VecDeque<PathBuf>>>,
    search: String,
    last_search: String,
    search_result: VecDeque<PathBuf>,
    //benchmarks
    now: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            in_progress: Default::default(),
            result: Default::default(),
            trigger_fetch: true,
            search_in_progress: Default::default(),
            search: String::new(),
            last_search: String::from(" "),
            search_result: Default::default(),
            now: Instant::now(),
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
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            in_progress,
            result,
            trigger_fetch,
            search_in_progress,
            search,
            last_search,
            search_result,
            now,
        } = self;
        if let Some(receiver) = in_progress {
            // Are we there yet?
            if let Ok(r) = receiver.try_recv() {
                println!("done scanning");
                *in_progress = None;
                *result = Some(r);
            }
        }

        if *trigger_fetch {
            println!("scanning");
            *trigger_fetch = false;
            let (sender, receiver) = std::sync::mpsc::channel();
            *in_progress = Some(receiver);
            thread::spawn(move || {
                sender.send(App::scan_drive()).ok();
            });
        }

        if let Some(receiver) = search_in_progress {
            // Are we there yet?
            if let Ok(r) = receiver.try_recv() {
                println!(
                    "done searching, {} items, took {:?}",
                    r.len(),
                    now.elapsed()
                );

                *search_in_progress = None;
                *search_result = r;
            }
        }

        if search != last_search && result.is_some() {
            self.now = Instant::now();
            //TODO if the current search is longer than the last search
            //it should not delete the last search_result.
            //instead it should search through that.
            println!("searching");
            let (sender, receiver) = std::sync::mpsc::channel();

            //add the receiver
            *search_in_progress = Some(receiver);
            //wipe the last search
            *search_result = VecDeque::new();
            //update the last search
            *last_search = search.clone();

            let r = result.clone();
            // let r = result.clone();
            let s = search.clone();

            thread::spawn(move || {
                sender.send(App::search_files(r.unwrap(), s)).ok();
            });
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
                egui::TextEdit::singleline(search),
            );

            egui::warn_if_debug_build(ui);

            ui.add_space(10.0);

            //broken
            // ui.style_mut().spacing.scroll_bar_width = 13.0;
            let row_height = ui.fonts()[TextStyle::Body].row_height();
            let num_rows = search_result.len();

            ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
                if result.is_none() {
                    ui.label("Please wait...");
                } else {
                    for row in row_range {
                        ui.label(
                            search_result
                                .get(row)
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                        );
                    }
                }
            });
        });
    }
}
