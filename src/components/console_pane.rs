use eframe::egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct ConsolePane {
    pub open: bool,
}

impl ConsolePane {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Console Pane");
        });
    }
}
