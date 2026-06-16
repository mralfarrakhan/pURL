use eframe::egui::{Button, ScrollArea, Ui, Widget};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct ExplorerPane {
    pub open: bool,
}

impl ExplorerPane {
    pub fn ui(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                if Button::new("Add")
                    .min_size([ui.available_width(), 0.0].into())
                    .ui(ui)
                    .clicked()
                {}
            });
        });
    }
}
