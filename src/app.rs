use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use crate::scanner;

pub struct WebEnumApp {
    pub target_url: String,
    pub wordlist_path: Option<PathBuf>,
    pub wordlist_status: String,
    pub status_message: String,
    
    pub status_200_results: Vec<String>,
    pub status_403_results: Vec<String>,
    pub status_others: Vec<String>,
    
    pub is_scanning: bool,
    pub rx: Option<Receiver<(u16, String)>>,
    pub tx: Sender<(u16, String)>,
}

impl Default for WebEnumApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            target_url: "https://example.com".to_string(),
            wordlist_path: None,
            wordlist_status: "No wordlist selected (Using default paths)".to_string(),
            status_message: "Ready to scan...".to_string(),
            status_200_results: Vec::new(),
            status_403_results: Vec::new(),
            status_others: Vec::new(),
            is_scanning: false,
            rx: Some(rx),
            tx,
        }
    }
}

impl eframe::App for WebEnumApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ref rx) = self.rx {
            while let Ok((status_code, url)) = rx.try_recv() {
                if status_code == 0 && url == "DONE" {
                    self.is_scanning = false;
                    self.status_message = "Scan completed successfully!".to_string();
                } else {
                    let log = format!("[{}] {}", status_code, url);
                    match status_code {
                        200 => self.status_200_results.push(log),
                        403 => self.status_403_results.push(log),
                        _ => self.status_others.push(log),
                    }
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌐 WebEnum - Categorized Security Scanner Dashboard");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Target URL:");
                ui.text_edit_singleline(&mut self.target_url);
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Wordlist File:");
                if ui.button("📁 Browse .txt File").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Text Files", &["txt"]).pick_file() {
                        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                        self.wordlist_status = format!("Loaded: {}", file_name);
                        self.wordlist_path = Some(path);
                    }
                }
                ui.label(&self.wordlist_status);
            });

            ui.add_space(15.0);

            if self.is_scanning {
                ui.spinner();
                ui.label(&self.status_message);
            } else {
                if ui.button("🚀 Start Categorized Scan").clicked() {
                    self.status_200_results.clear();
                    self.status_403_results.clear();
                    self.status_others.clear();
                    
                    self.is_scanning = true;
                    self.status_message = "Scanning and categorizing paths...".to_string();
                    
                    let target = self.target_url.clone();
                    let tx = self.tx.clone();
                    let custom_path = self.wordlist_path.clone();
                    
                    scanner::start_scan_task(target, custom_path, tx);
                }
            }

            ui.add_space(15.0);
            ui.separator();
            ui.heading("📊 Scan Results Dashboard (Categorized)");
            ui.add_space(5.0);

            ui.columns(3, |columns| {
                columns[0].vertical(|ui| {
                    ui.colored_label(egui::Color32::GREEN, format!("🟢 [200 OK] ({})", self.status_200_results.len()));
                    ui.separator();
                    egui::ScrollArea::vertical().id_source("col_200").max_height(250.0).show(ui, |ui| {
                        for res in &self.status_200_results {
                            ui.monospace(res);
                        }
                    });
                });

                columns[1].vertical(|ui| {
                    ui.colored_label(egui::Color32::YELLOW, format!("🟡 [403 Forbidden] ({})", self.status_403_results.len()));
                    ui.separator();
                    egui::ScrollArea::vertical().id_source("col_403").max_height(250.0).show(ui, |ui| {
                        for res in &self.status_403_results {
                            ui.monospace(res);
                        }
                    });
                });

                columns[2].vertical(|ui| {
                    ui.colored_label(egui::Color32::LIGHT_BLUE, format!("🔵 [Others / 3xx, 5xx] ({})", self.status_others.len()));
                    ui.separator();
                    egui::ScrollArea::vertical().id_source("col_others").max_height(250.0).show(ui, |ui| {
                        for res in &self.status_others {
                            ui.monospace(res);
                        }
                    });
                });
            });
        });
        
        ctx.request_repaint();
    }
}