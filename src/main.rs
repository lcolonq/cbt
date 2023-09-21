#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::{Arc, Mutex}, thread};

use enigo::*;
use eframe::egui;

#[derive(Clone)]
struct SensitivePixel {
    x: i32,
    y: i32,
    r: u8,
    g: u8,
    b: u8,
}

impl SensitivePixel {
    fn from_mouse_position() -> Option<Self> {
        let enigo = Enigo::new();
        let (x, y) = enigo.mouse_location();
        let screen = screenshots::Screen::from_point(x, y).ok()?;
        let cap = screen.capture_area(
            x - screen.display_info.x,
            y - screen.display_info.y,
            1,
            1,
        ).ok()?;
        let mut iter = cap.into_iter();
        let r = *iter.next()?;
        let g = *iter.next()?;
        let b = *iter.next()?;
        Some(Self {
            x,
            y,
            r,
            g,
            b,
        })
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let saved = Arc::new(Mutex::new(SensitivePixel::from_mouse_position().unwrap()));
    let saved1 = saved.clone();

    eframe::run_simple_native("Clonk's Basic Tool", options, move |ctx, _frame| {
        let saved2 = saved.clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.code("
  /----\\     /$$$$$$  /$$$$$$$  /$$$$$$$$ 
 / x  - \\   /$$__  $$| $$__  $$|__  $$__/ 
 \\  ww  /  | $$  \\__/| $$  \\ $$   | $$    
  +----+   | $$      | $$$$$$$    | $$    
           | $$      | $$__  $$   | $$    
 twitch.tv | $$    $$| $$  \\ $$   | $$    
 /LCOLONQ  |  $$$$$$/| $$$$$$$/   | $$    
    :3      \\______/ |_______/    |__/    
");
            let s = SensitivePixel::from_mouse_position().unwrap();
            ui.label(format!("current: ({}, {}) #{:02x}{:02x}{:02x}", s.x, s.y, s.r, s.g, s.b));
            let inner = saved1.lock().unwrap().clone();
            ui.label(format!("saved: ({}, {}) #{:02x}{:02x}{:02x}", inner.x, inner.y, inner.r, inner.g, inner.b));
            if ui.button("select").clicked() {
                println!("selecting");
                inputbot::MouseButton::LeftButton.bind(move || {
                    println!("selected");
                    *saved2.lock().unwrap() = SensitivePixel::from_mouse_position().unwrap();
                });
                thread::spawn(|| {
                    println!("listening");
                    inputbot::handle_input_events();
                    println!("done");
                });
            }
        });
        ctx.request_repaint();
    })
}
