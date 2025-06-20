#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_file_dialog::{self, FileDialog};
use rfd;



struct DupeLsApp {
    file_dialog: FileDialog,
    directory: String,
    all: bool,
    depth: u32,
    omit: bool,
}

impl Default for DupeLsApp {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            directory: String::new(),
            all: false,
            omit: false,
            depth: 2,
        }
    }
}

impl DupeLsApp {
    fn run(&mut self) {
        println!(
            "Running dupels with directory: {}, all: {}, depth: {}, omit: {}",
            self.directory, self.all, self.depth, self.omit
        );        
    }
}

impl eframe::App for DupeLsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // Title
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.heading("dupels");
                },
            );
            
            // Directory picker
            ui.horizontal(|ui| {
                ui.label("Directory:");
                ui.text_edit_singleline(&mut self.directory);
                if ui.button("Browse...").clicked() {
                    println!("Opening file dialog...");
                    self.file_dialog.pick_directory();
                }
            });

            // -a checkbox
            ui.checkbox(&mut self.all, "Include hidden '.' files");

            // -o checkbox
            ui.checkbox(&mut self.omit, "Omit unique files");

            // -d input
            ui.horizontal(|ui| {
                ui.label("Search Depth:");
                ui.add(egui::DragValue::new(&mut self.depth).clamp_range(1..=32));
            });

            if ui.button("Run").clicked() {
                // Here you would call the actual logic of dupels with the parameters
                // For now, we just print the parameters to the console
                println!(
                    "Running dupels with directory: {}, all: {}, depth: {}, omit: {}",
                    self.directory, self.all, self.depth, self.omit
                );
                
                // You can replace this with actual logic to run dupels and display results
            }

            ui.label("Output:");
            /*
            ui.add(
                egui::TextEdit::multiline(&mut self.output)
                    .desired_rows(10)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            ); */
        });
    }
}


fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "dupels",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<DupeLsApp>::default())
        }),
    )
}