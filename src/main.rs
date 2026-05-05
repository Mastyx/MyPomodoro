#![allow(warnings)]

use std::default;

use eframe::egui;

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
//

struct NomeApplicazione {
    // la nostra struttura non contiene nulla per il momento\
}

// una volta importato eframe con il modulo egui
// implementiamolo nella struttura
impl eframe::App for NomeApplicazione {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
}

fn main() -> Result<(), eframe::Error> {
    let option = eframe::NativeOptions::default();
    let app = NomeApplicazione {};
    eframe::run_native("test_applicazione", option, Box::new(|cc| Box::new(app)))
}
