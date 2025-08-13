#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

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

    fn display_duplicate_groups(&mut self, ui: &mut egui::Ui) {
        let separator = "==="; // TODO: This is sus. Ideally the vector shouldn't have the separator.
        let mut groups = Vec::new();
        let mut current_group = Vec::new();
        
        for line in &self.output {
            if line == separator {
                if !current_group.is_empty() {
                    groups.push(current_group.clone());
                    current_group.clear();
                }
            } else if !line.trim().is_empty() {
                current_group.push(line.clone());
            }
        }
        
        if !current_group.is_empty() {  // Edge case for last group.
            groups.push(current_group);
        }
        
        // Display files as groups in a collapsible header.
        for (group_idx, group) in groups.iter().enumerate() {
            let header_text = if group.len() > 1 {
                format!("Group {} ({} files)", group_idx + 1, group.len())
            } else {
                format!("Group {} (1 file)", group_idx + 1)
            };
            
            egui::CollapsingHeader::new(header_text)
                .default_open(true)
                .show(ui, |ui| {
                    ui.indent("group_indent", |ui| {
                        for file_path in group.iter() {
                            ui.horizontal(|ui| {
                                // Add a bullet point
                                ui.label("•");
                                // Make the path selectable and clickable
                                let response = ui.add(
                                    egui::Label::new(file_path)
                                        .selectable(true)
                                        .sense(egui::Sense::click())
                                );
                                
                                if response.hovered() {
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                }
                                
                                if response.clicked() {
                                    // Open the file/directory in the default system application
                                    if let Err(e) = open::that(file_path) {
                                        println!("Failed to open {}: {}", file_path, e);
                                    }
                                }
                            });
                        }
                    });
                });
            
            // Add some spacing between groups
            ui.add_space(5.0);
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
            ui.add_space(10.0);
            
            // Directory picker section
            ui.horizontal(|ui| {
                ui.label("Directory:");
                if self.directory.is_empty() {
                    ui.label("(No directory selected)");
                } else {
                    ui.label(&self.directory);
                }
            });
            ui.add_space(5.0);
            
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
            });
            ui.add_space(15.0);

            // Options section
            ui.checkbox(&mut self.all, "Include hidden '.' files");
            ui.add_space(3.0);

            ui.checkbox(&mut self.omit, "Omit unique files");
            ui.add_space(3.0);

            ui.horizontal(|ui| {
                ui.label("Search Depth:");
                ui.add(egui::DragValue::new(&mut self.depth).range(1..=32));
            });
            ui.add_space(15.0);

            // Run button
            if ui.button("Run").clicked() {
                // Here you would call the actual logic of dupels with the parameters
                // For now, we just print the parameters to the console
                self.run();
                
                // You can replace this with actual logic to run dupels and display results
            }
            ui.add_space(10.0);

            // Display results in a better format
            ui.separator();
            ui.label("Results:");
            
            if self.output.is_empty() {
                ui.label("No results yet. Click 'Run' to find duplicates.");
            } else {
                // Create a scrollable area that uses remaining available height
                let available_height = ui.available_height() - 20.0; // Leave some margin
                egui::ScrollArea::vertical()
                    .max_height(available_height)
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        self.display_duplicate_groups(ui);
                    });
            }
        });
    }
}


fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])  // Larger initial size
            .with_min_inner_size([400.0, 300.0])  // Minimum window size
            .with_resizable(true)  // Allow resizing
            .with_maximize_button(true), // Show maximize button
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