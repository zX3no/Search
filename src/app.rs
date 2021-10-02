use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::channel;
use std::thread;

use eframe::egui::{color::*, *};
use eframe::{egui, epi};
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::search::search;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    search: String,
    last_search: String,
    files: VecDeque<String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            search: String::from("among us"),
            last_search: String::new(),
            files: VecDeque::new(),
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

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });
        if self.search != self.last_search {
            self.files = VecDeque::new();
            let (send, recv) = channel();
            self.last_search = self.search.clone();
            let search = self.search.clone();
            thread::spawn(move || {
                let file = File::open("C.db").expect("no such file");
                let buf = BufReader::new(file);
                for line in buf.lines() {
                    if line.as_ref().unwrap().contains(&search) {
                        if let Err(e) = send.send(line.unwrap().to_string()) {
                            println!("{}", e);
                        }
                    }
                }
            });
            for r in recv.iter() {
                self.files.push_back(r);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.search);

            egui::warn_if_debug_build(ui);

            ui.add_space(4.0);

            let row_height = ui.fonts()[TextStyle::Body].row_height();
            let num_rows = self.files.len();

            ScrollArea::auto_sized().show_rows(ui, row_height, num_rows, |ui, row_range| {
                for row in row_range {
                    println!("printing{}", row);
                    ui.label(self.files.get(row).unwrap());
                }
            });
        });
    }
}
