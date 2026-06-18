use std::ops::Not;

use eframe::{
    APP_KEY, App, CreationContext,
    egui::{
        Align, Button, CentralPanel, Color32, Key, KeyboardShortcut, Layout, Modal, Modifiers,
        Panel, RichText, TextEdit, TextWrapMode, Ui, gui_zoom::zoom_menu_buttons,
    },
    get_value, set_value,
};
use serde::{Deserialize, Serialize};

use crate::{
    components::{console_pane::ConsolePane, explorer_pane::ExplorerPane},
    utilities::collections::Collections,
};

#[derive(Default)]
pub struct EphemeralState {
    pub new_collection_id: String,
}

#[derive(Default, Deserialize, Serialize)]
struct State {
    explorer_pane: ExplorerPane,
    console_pane: ConsolePane,
    collections: Collections,
    #[serde(skip)]
    ephemeral_state: EphemeralState,
}

#[derive(Default)]
pub struct Purl {
    state: State,
}

impl Purl {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        cc.storage
            .and_then(|s| get_value(s, APP_KEY))
            .map(|s| Self { state: s })
            .unwrap_or_default()
    }

    fn view_menu_button(&mut self, ui: &mut Ui) {
        let explorer_shortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::E);
        let console_shortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::Backtick);

        if ui.input_mut(|i| i.consume_shortcut(&explorer_shortcut)) {
            self.state.explorer_pane.open = !self.state.explorer_pane.open;
        }

        if ui.input_mut(|i| i.consume_shortcut(&console_shortcut)) {
            self.state.console_pane.open = !self.state.console_pane.open;
        }

        ui.menu_button("View", |ui| {
            ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);

            zoom_menu_buttons(ui);
            ui.weak(format!("Zoom: {:.0}%", 100.0 * ui.ctx().zoom_factor()))
                .on_hover_text("The UI zoom level");

            ui.separator();

            if ui
                .add(
                    Button::new("Explorer Pane")
                        .shortcut_text(ui.ctx().format_shortcut(&explorer_shortcut))
                        .selected(self.state.explorer_pane.open),
                )
                .clicked()
            {
                self.state.explorer_pane.open = !self.state.explorer_pane.open;
            }

            if ui
                .add(
                    Button::new("Console Pane")
                        .shortcut_text(ui.ctx().format_shortcut(&console_shortcut))
                        .selected(self.state.console_pane.open),
                )
                .clicked()
            {
                self.state.console_pane.open = !self.state.console_pane.open;
            }

            ui.separator();

            if ui.button("Reset Layout").clicked() {
                ui.memory_mut(|mem| *mem = Default::default());
            }
        });
    }

    fn new_collection_dialog(&mut self, ui: &mut Ui) {
        Modal::new("add_collection_dialog".into()).show(ui.ctx(), |ui| {
            ui.vertical(|ui| {
                ui.set_max_width(240.0);

                ui.heading("Add Collection");

                ui.separator();

                ui.horizontal(|ui| {
                    // let label = ui.label("Name");
                    let name_edit =
                        TextEdit::singleline(&mut self.state.ephemeral_state.new_collection_id)
                            .hint_text("New Collection");
                    name_edit.show(ui);
                });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui
                        .add(Button::new("Cancel").fill(Color32::DARK_RED))
                        .clicked()
                    {
                        self.state.explorer_pane.ephemeral_state.active_add = false;
                    }

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_enabled_ui(
                            self.state
                                .ephemeral_state
                                .new_collection_id
                                .is_empty()
                                .not(),
                            |ui| {
                                if ui
                                    .add(Button::new("Add").fill(Color32::DARK_GREEN))
                                    .clicked()
                                {
                                    self.state.explorer_pane.ephemeral_state.active_add = false;
                                    self.state.ephemeral_state.new_collection_id.clear();
                                }
                            },
                        )
                    });
                });
            });
        });
    }
}

fn main_pane(ui: &mut Ui) {
    ui.heading("pURL by mralfarrakhan");
}

impl App for Purl {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        set_value(storage, APP_KEY, &self.state);
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        let style = ui.style_mut();
        style.visuals.override_text_color.replace(Color32::WHITE);

        Panel::top("menu_bar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                self.view_menu_button(ui);

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.toggle_value(&mut self.state.explorer_pane.open, "Explorer");
                    ui.toggle_value(&mut self.state.console_pane.open, "Console");
                });
            });
        });

        Panel::bottom("status_bar").show_inside(ui, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(RichText::new("pURL").weak());
            });
        });

        if self.state.explorer_pane.open {
            Panel::left("explorer_pane")
                .resizable(true)
                .default_size(200.0)
                .size_range(180.0..=300.0)
                .show_inside(ui, |ui| {
                    ui.take_available_space();

                    self.state.explorer_pane.ui(ui);
                });
        }

        if self.state.console_pane.open {
            Panel::bottom("console_pane")
                .resizable(true)
                .default_size(200.0)
                .size_range(200.0..=400.0)
                .show_inside(ui, |ui| {
                    ui.take_available_space();

                    self.state.console_pane.ui(ui);
                });
        }

        if self.state.explorer_pane.ephemeral_state.active_add {
            self.new_collection_dialog(ui);
        }

        CentralPanel::default().show_inside(ui, |ui| {
            main_pane(ui);
        });
    }
}
