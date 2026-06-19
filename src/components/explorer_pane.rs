use eframe::egui::{Button, ScrollArea, Ui, Widget};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct EphemeralState {
    pub active_add: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ExplorerPane {
    pub open: bool,
    #[serde(skip)]
    pub ephemeral_state: EphemeralState,
}

impl Default for ExplorerPane {
    fn default() -> Self {
        Self {
            open: true,
            ephemeral_state: Default::default(),
        }
    }
}

impl ExplorerPane {
    pub fn ui(&mut self, ui: &mut Ui) -> bool {
        let mut add_clicked = false;
        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                if Button::new("Add")
                    .min_size([ui.available_width(), 0.0].into())
                    .ui(ui)
                    .clicked()
                {
                    self.ephemeral_state.active_add = true;
                    add_clicked = true;
                }
            });
        });
        add_clicked
    }
}
