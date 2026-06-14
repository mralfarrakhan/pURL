use eframe::{
    APP_KEY, App, CreationContext,
    egui::{
        Align, Button, CentralPanel, Key, KeyboardShortcut, Layout, Modifiers, Panel, TextWrapMode,
        Ui, gui_zoom::zoom_menu_buttons,
    },
    get_value, set_value,
};
use serde::{Deserialize, Serialize};

use crate::components::{console_pane::ConsolePane, explorer_pane::ExplorerPane};

#[derive(Default, Deserialize, Serialize)]
struct State {
    explorer_pane: ExplorerPane,
    console_pane: ConsolePane,
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
        let explorer_shortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::E);
        let console_shortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::R);

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
        });
    }
}

fn main_pane(_ui: &mut Ui) {}

impl App for Purl {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        set_value(storage, APP_KEY, &self.state);
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        Panel::top("menu_bar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                self.view_menu_button(ui);

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.toggle_value(&mut self.state.explorer_pane.open, "Explorer");
                    ui.toggle_value(&mut self.state.console_pane.open, "Console");
                })
            });
        });

        CentralPanel::default().show_inside(ui, |ui| {
            main_pane(ui);
        });
    }
}
