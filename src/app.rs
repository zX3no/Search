use std::collections::VecDeque;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use eframe::egui::{color::*, *};
use eframe::{egui, epi};
use jwalk::WalkDir;

pub struct App {
    //async drive scan
    in_progress: Option<Receiver<VecDeque<PathBuf>>>,
    result: Option<VecDeque<PathBuf>>,
    trigger_fetch: bool,
    //search
    search: String,
    last_search: String,
    search_result: VecDeque<String>,
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
        return out;
    }
}
fn test() -> usize {
    thread::sleep(Duration::from_secs(1));
    return 5;
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
            search,
            last_search,
            search_result,
        } = self;
        if let Some(receiver) = in_progress {
            // Are we there yet?
            if let Ok(r) = receiver.try_recv() {
                *in_progress = None;
                *result = Some(r);
            }
        }

        if *trigger_fetch {
            *trigger_fetch = false;
            let (sender, receiver) = std::sync::mpsc::channel();
            *in_progress = Some(receiver);
            thread::spawn(move || {
                println!("waiting");
                sender.send(App::scan_drive()).ok();
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

        if search != last_search && result.is_some() {
            println!("starting search");
            *search_result = VecDeque::new();
            *last_search = search.clone();

            for file in result.as_ref().unwrap() {
                let data = file.to_string_lossy();
                if data.contains(&*search) || search == "" {
                    search_result.push_back(data.to_string());
                }
            }

            println!("done search");
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            //TODO make this bigger
            ui.text_edit_singleline(search);

            egui::warn_if_debug_build(ui);

            ui.add_space(4.0);

            let row_height = ui.fonts()[TextStyle::Body].row_height();
            let num_rows = search_result.len();

            ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
                if result.is_none() {
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

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            in_progress: Default::default(),
            result: Default::default(),
            trigger_fetch: true,
            search: String::new(),
            last_search: String::from(" "),
            search_result: Default::default(),
        }
    }
}
