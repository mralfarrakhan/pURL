#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod purl;

use crate::purl::Purl;
use eframe::{NativeOptions, run_native};

fn main() -> eframe::Result {
    env_logger::init();

    let options = NativeOptions {
        ..Default::default()
    };

    run_native("pURL", options, Box::new(|cc| Ok(Box::new(Purl::new(cc)))))
}
