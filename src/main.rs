mod app;
mod scanner;
mod reporter;

use app::WebEnumApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([850.0, 600.0])
            .with_title("WebEnum - Categorized URL Enumeration Scanner"),
        ..Default::default()
    };
    
    eframe::run_native(
        "WebEnum GUI",
        options,
        Box::new(|_cc| Ok(Box::new(WebEnumApp::default()))),
    )
}