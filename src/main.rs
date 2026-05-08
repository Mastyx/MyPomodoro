#![allow(warnings)]

use std::{
    default,
    time::{Duration, Instant},
};

use eframe::{egui, wgpu::rwh::AppKitDisplayHandle};
use egui::{CentralPanel, Context};

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

//// struttura dell' applicazione
//fn main() {
//    let native_option = eframe::NativeOptions::default();
//
//    eframe::run_native(
//        "Nome del Applicazione",
//        native_option,
//        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
//    );
//}
//
//// struttura  della nostra applicazione
//#[derive(Default)]
//struct Application {
//    testo: String,
//}
//
//impl Application {
//    fn new(cc: &eframe::CreationContext<'_>) -> Self {
//        Self::default()
//    }
//}
//impl eframe::App for Application {
//    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
//        // all'interno di questa funzione inseriamo la parte grafica
//
//        // impostiamo il contenitore principale
//        let contenitore = ui.ctx();
//        barra_in_alto(&contenitore);
//        // pannello a sinistra sidebar
//        egui::SidePanel::left("SidePanel").show(contenitore, |ui| {
//            ui.label("Elemento 1");
//            ui.label("Elemento 2");
//        });
//        // pannello centrale
//        pannello_centrale(&contenitore); // abbiamo creato una funzione a parte
//        //egui::CentralPanel::default().show(contenitore, |ui| {
//        //    ui.heading("Testo del central panel ");
//        //});
//    }
//}
//// funzione che
//fn pannello_centrale(contesto: &Context) {
//    let mut testo = String::new();
//    testo = "Testo della pagina centrale della nostra applicazione".to_string();
//    CentralPanel::default().show(contesto, |ui| {
//        ui.heading("Testo contenuto nel pannello centrale ");
//        ui.add(egui::TextEdit::multiline(&mut testo));
//    });
//}
//fn barra_in_alto(contesto: &Context) {
//    // pannello in alto
//    egui::TopBottomPanel::top("Top panel")
//        .min_height(50.0)
//        .show(contesto, |ui| {
//            ui.vertical_centered(|ui| {
//                // pulsante per la chiusura
//                if ui
//                    .button(egui::RichText::new("Esci").size(20.0).strong())
//                    .clicked()
//                {
//                    contesto.send_viewport_cmd(egui::ViewportCommand::Close);
//                };
//            });
//        });
//}

// ---------------------------------------------------------------------

enum Modalita {
    Concentrazione,
    Pausa,
}
impl Modalita {
    //etichetta mostrata nel pulsante di selezione
    fn etichetta(self) -> &'static str {
        match self {
            Modalita::Pausa => "⏸️ Pausa 5:00",
            Modalita::Concentrazione => "🍅 Focus 25:00",
        }
    }
    //durata associata alla modalita
    fn durata(self) -> Duration {
        match self {
            Modalita::Concentrazione => Duration::from_secs(25 * 60),
            Modalita::Pausa => Duration::from_secs(5 * 60),
        }
    }
}
// struttura del applicazione
struct Applicazione {
    modalita: Modalita,
    tempo_rimanente: Duration,
    in_esecuzione: bool,
    ultimo_aggiornamento: Option<Instant>,
}
impl Default for Applicazione {
    fn default() -> Self {
        Self {
            modalita: Modalita::Concentrazione,
            tempo_rimanente: Modalita::Concentrazione.durata(),
            in_esecuzione: false,
            ultimo_aggiornamento: None,
        }
    }
}

fn main() {}
