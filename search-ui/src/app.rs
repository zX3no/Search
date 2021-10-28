use eframe::egui::*;
use eframe::{egui, epi};

mod table;
use table::Table;
pub struct App {
    search: String,
    selected_row: Option<usize>,
    data: Vec<(String, String)>,
}
impl Default for App {
    fn default() -> Self {
        Self {
            search: String::new(),
            selected_row: None,
            data: vec![
                (String::from("D"), String::from("D:\\")),
                (String::from("Git"), String::from("D:\\Git")),
            ],
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Search!"
    }

    fn setup(
        &mut self,
        _ctx: &CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &CtxRef, frame: &mut epi::Frame<'_>) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            let string = TextEdit::singleline(&mut self.search);
            ui.add_sized([ui.available_width(), 0.0], string);

            warn_if_debug_build(ui);

            ui.add_space(4.0);

            let table = Table::new_selectable("file_list", &mut self.data, &mut self.selected_row)
                .column("File", |(f, _)| f.to_string())
                .column("Path", |(_, p)| p.to_string());

            ui.add(table);
        });
    }
}
