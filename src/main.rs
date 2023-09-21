#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread};

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
        min_window_size: Some(egui::vec2(320.0, 240.0)),
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        always_on_top: true,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        icon_data: Some(eframe::IconData::try_from_png_bytes(
            include_bytes!("icon.png")
        ).unwrap()),
        ..Default::default()
    };

    let selecting = Arc::new(AtomicBool::new(false));
    let selecting1 = selecting.clone();

    let saved = Arc::new(Mutex::new(Vec::new()));
    let saved1 = saved.clone();
    let saved2 = saved.clone();

    inputbot::MouseButton::LeftButton.bind(move || {
        if selecting1.fetch_and(false, Ordering::SeqCst) {
            println!("selected");
            saved2.lock().unwrap()
                .push(SensitivePixel::from_mouse_position().unwrap());
        }
    });
    thread::spawn(|| {
        inputbot::handle_input_events();
    });

    eframe::run_simple_native("clonk's basic tool", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.monospace("
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
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    let mut inner = saved1.lock().unwrap();
                    inner.retain(|t| {
                        ui.horizontal(|ui| {
                            let mut delete = false;
                            if ui.button("delete").clicked() {
                                delete = true;
                            }
                            ui.monospace(format!("({:4}, {:4}) #{:02x}{:02x}{:02x}", t.x, t.y, t.r, t.g, t.b));
                            delete
                        }).inner
                    });
                    if selecting.load(Ordering::Relaxed) {
                        ui.monospace("click anywhere...");
                    } else {
                        if ui.button("select").clicked() {
                            println!("selecting");
                            selecting.store(true, Ordering::SeqCst);
                        }
                    }
                });
            });
        });
        ctx.request_repaint();
    })
}
