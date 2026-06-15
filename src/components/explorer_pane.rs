use eframe::egui::{ScrollArea, Ui};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct ExplorerPane {
    pub open: bool,
}

impl ExplorerPane {
    pub fn ui(&mut self, ui: &mut Ui) {
        ScrollArea::horizontal().show(ui, |ui| {
            ui.label("meh");
            if ui.button("Add").clicked() {}
        });
    }
}
