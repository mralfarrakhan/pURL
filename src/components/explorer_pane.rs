use eframe::egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct ExplorerPane {
    pub open: bool,
}

impl ExplorerPane {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Explorer Pane");
        });
    }
}
