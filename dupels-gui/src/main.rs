#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs::File;

use eframe::egui;

use dupels_lib::{Gui, run_gui};

struct DupeLsApp {
    output: Vec<String>,
    directory: String,
    all: bool,
    depth: u32,
    omit: bool,
}

impl Default for DupeLsApp {
    fn default() -> Self {
        Self {
            output: Vec::new(),
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
        self.output = run_gui(&Gui {
            directory: self.directory.clone(),
            all: self.all,
            depth: self.depth as usize,
            omit: self.omit,
        });
        if self.output.is_empty() {
            println!("No duplicates found.");
        } else {
            for line in &self.output {
                println!("{}", line);
            }
        }
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
                if self.directory.is_empty() {
                    ui.label("(No directory selected)");
                } else {
                    ui.label(&self.directory);
                }
            });
            
            ui.horizontal(|ui| {
                if ui.button("Browse...").clicked() {
                    println!("Browse button clicked - opening file dialog...");
                    
                    // Try direct synchronous call first
                    match rfd::FileDialog::new()
                        .set_title("Select Directory for Duplicate Search")
                        .pick_folder() 
                    {
                        Some(path) => {
                            self.directory = path.display().to_string();
                            println!("Directory selected: {}", self.directory);
                        }
                        None => {
                            println!("No directory selected - dialog may have been cancelled");
                        }
                    }
                }
                
                ui.label("Or type path:");
                ui.text_edit_singleline(&mut self.directory);
            });

            // -a checkbox
            ui.checkbox(&mut self.all, "Include hidden '.' files");

            // -o checkbox
            ui.checkbox(&mut self.omit, "Omit unique files");

            // -d input
            ui.horizontal(|ui| {
                ui.label("Search Depth:");
                ui.add(egui::DragValue::new(&mut self.depth).range(1..=32));
            });

            if ui.button("Run").clicked() {
                // Here you would call the actual logic of dupels with the parameters
                // For now, we just print the parameters to the console
                self.run();
                
                // You can replace this with actual logic to run dupels and display results
            }

            ui.label("Output:");
            ui.add(
                egui::TextEdit::multiline(&mut self.output.join("\n"))
                    .desired_rows(10)
                    .desired_width(f32::INFINITY)
                    .code_editor(),
            );
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