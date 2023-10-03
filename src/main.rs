#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod objects;
mod game;
mod brains;

use eframe::egui;
use egui::{containers::*, *};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Ping Pong",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // the game box
            Box::<game::PingPong>::new(game::PingPong::new(Vec2 { x: 800.0, y: 600.0 }))
        }),
    )
}

