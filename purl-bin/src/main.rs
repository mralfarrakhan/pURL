#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod purl;

use std::error::Error;

use crate::purl::Purl;
use eframe::{NativeOptions, run_native};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let options = NativeOptions {
        ..Default::default()
    };

    run_native("pURL", options, Box::new(|cc| Ok(Box::new(Purl::new(cc)))))?;

    Ok(())
}
