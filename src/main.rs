#![allow(warnings)]

use std::{
    default,
    time::{Duration, Instant},
};

use eframe::{egui, wgpu::rwh::AppKitDisplayHandle};
use egui::{CentralPanel, Context, RichText};

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
#[derive(Clone, Copy, PartialEq)]
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
impl Applicazione {
    fn aggiorna_timer(&mut self, contesto: &Context) {
        if !self.in_esecuzione {
            return;
        }

        let adesso = Instant::now();

        if let Some(precedente) = self.ultimo_aggiornamento {
            let trascorso = adesso.duration_since(precedente);
            self.tempo_rimanente = self.tempo_rimanente.saturating_sub(trascorso);
            if self.tempo_rimanente.is_zero() {
                self.in_esecuzione = false;
            }
        }

        self.ultimo_aggiornamento = Some(adesso);

        contesto.request_repaint_after(Duration::from_millis(500));
    }

    // cambia modalita e azzera il timer
    fn cambia_modalita(&mut self, nuova: Modalita) {
        self.modalita = nuova;
        self.tempo_rimanente = nuova.durata();
        self.in_esecuzione = false;
        self.ultimo_aggiornamento = None;
    }

    //riporta il timer al valore iniziale
    fn azzera(&mut self) {
        self.tempo_rimanente = self.modalita.durata();
        self.in_esecuzione = false;
        self.ultimo_aggiornamento = None;
    }
    //avvia o mette in pausa il timer
    fn avvia_o_pausa(&mut self) {
        self.in_esecuzione = !self.in_esecuzione;
        if self.in_esecuzione {
            self.ultimo_aggiornamento = Some(Instant::now());
        }
    }

    // formatta il tempo timanente come mm:ss
    fn tempo_formattato(&self) -> String {
        let secondi_totali = self.tempo_rimanente.as_secs();
        let minuti = secondi_totali / 60;
        let secondi = secondi_totali % 60;
        format!("{:02}:{:02}", minuti, secondi)
    }
}

// punto di ingesso del interfaccia grafica
impl eframe::App for Applicazione {
    fn ui(&mut self, pannello: &mut egui::Ui, _finestra: &mut eframe::Frame) {
        // senza Context
        if self.in_esecuzione {
            let adesso = Instant::now();
            if let Some(precedente) = self.ultimo_aggiornamento {
                let trascorso = adesso.duration_since(precedente);
                self.tempo_rimanente = self.tempo_rimanente.saturating_sub(trascorso);
                if self.tempo_rimanente.is_zero() {
                    self.in_esecuzione = false;
                }
            }
            self.ultimo_aggiornamento = Some(adesso);
            pannello
                .ctx()
                .request_repaint_after(Duration::from_millis(500));
        }

        pannello.add_space(10.0);

        //selezione modalita
        pannello.horizontal(|riga| {
            for modalita in [Modalita::Concentrazione, Modalita::Pausa] {
                let selezionata = self.modalita == modalita;
                if riga
                    .selectable_label(selezionata, RichText::new(modalita.etichetta()).size(18.0))
                    .clicked()
                    && !selezionata
                {
                    self.cambia_modalita(modalita);
                }
            }
        });
        pannello.add_space(8.0);
        pannello.separator();
        pannello.add_space(8.0);

        // timer grande
        let testo_timer = self.tempo_formattato();
        pannello.vertical_centered(|centro| {
            centro.label(
                // settimo il font size
                RichText::new(testo_timer).size(48.0).strong(),
            );
        });
        pannello.add_space(10.0);

        //pulsanti
        pannello.vertical_centered(|centro| {
            // etichetta del pulsante settata a seconda dello stato se in esecuzione o meno
            let etichetta_pulsante = if self.in_esecuzione {
                "⏸️ PAUSA"
            } else {
                "▶️ AVVIA"
            };

            if centro
                .button(RichText::new(etichetta_pulsante).size(16.0))
                .clicked()
            {
                self.avvia_o_pausa();
            }
            centro.add_space(4.0);
            if centro.small_button("↺ Reset").clicked() {
                self.azzera();
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Pomodoro")
            .with_inner_size([420.0, 580.0])
            .with_min_inner_size([360.0, 500.0])
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "Test di applicazione",
        options,
        Box::new(|_contestocreazione| Ok(Box::new(Applicazione::default()))),
    )
}
