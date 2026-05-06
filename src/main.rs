#![allow(warnings)]

use std::default;

use eframe::{egui, wgpu::rwh::AppKitDisplayHandle};

mod timer;
//
//struct PomodoroApp {
//    // ancora nulla
//}
//// implementiamo il comportamento del PomodoroApp
//impl eframe::App for PomodoroApp {
//    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//        // pannello centrale
//        egui::CentralPanel::default().show(ctx, |ui| {
//            ui.label("Ciao! Gestisci il tuo tempo con un pomodoro !");
//            if ui.button("click").clicked() {
//                println!("Botton clicked");
//            }
//        });
//    }
//}
//
//fn main() -> Result<(), eframe::Error> {
//    // Configurazione della finestra
//    let options = eframe::NativeOptions {
//        viewport: egui::ViewportBuilder::default()
//            .with_inner_size([500.0, 300.0]) // Larghezza 400, altezza 300
//            .with_title("Pomodoro Widget"),
//        ..Default::default()
//    };
//
//    // Crea l'app (vuota per ora) e avvia la finestra
//    let app = PomodoroApp {};
//    eframe::run_native("Pomodoro Widget", options, Box::new(|_cc| Box::new(app)))
//}
//

// struttura dell' applicazione
fn main() {
    let native_option = eframe::NativeOptions::default();
    eframe::run_native(
        "Nome del Applicazione",
        native_option,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    );
}

// struttira della nostra applicazione
#[derive(Default)]
struct Application {
    //
}
impl Application {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
impl eframe::App for Application {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let contenitore = ui.ctx();
        egui::TopBottomPanel::top("Top panel")
            .min_height(50.0)
            .show(contenitore, |ui| {
                // pulsante per la chiusura
                if ui.button("Esci").clicked() {};
            });
        egui::SidePanel::left("SidePanel").show(contenitore, |ui| {
            ui.label("Elemento 1");
            ui.label("Elemento 2");
        });
        egui::CentralPanel::default().show(contenitore, |ui| {
            ui.heading("Testo del central panel ");
        });
    }
}
