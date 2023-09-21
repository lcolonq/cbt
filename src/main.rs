#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread, collections::HashMap, time::Duration};

use enigo::*;
use eframe::egui;

fn read_pixel(x: i32, y: i32) -> Option<(u8, u8, u8)> {
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
    Some((r, g, b))
}

fn read_pixels(ps: Vec<(i32, i32)>) -> HashMap<(i32, i32), (u8, u8, u8)> {
    let mut screens = HashMap::new();
    let sps: Vec<_> = ps.iter().map(|(x, y)| {
        let screen = screenshots::Screen::from_point(*x, *y).expect("failed to find screen");
        if !screens.contains_key(&screen.display_info.id) {
            let cap = screen.capture().expect("failed to capture screen");
            screens.insert(screen.display_info.id, cap);
        }
        (screen.display_info.id, x - screen.display_info.x, y - screen.display_info.y)
    }).collect();
    HashMap::from_iter(sps.iter().map(|(did, x, y)| {
        let cap = screens.get(did).expect("failed to find capture");
        let pixel = cap.get_pixel(*x as _, *y as _);
        ((*x, *y), (pixel[0], pixel[1], pixel[2]))
    }))
}

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
        let (r, g, b) = read_pixel(x, y)?;
        Some(Self {
            x,
            y,
            r,
            g,
            b,
        })
    }
}

struct Entry {
    id: String,
    pixel: SensitivePixel,
    cooldown: i32,
    current: (u8, u8, u8),
}

impl Entry {
    fn new(id: &str, pixel: SensitivePixel) -> Self {
        Self {
            id: String::from(id),
            pixel,
            cooldown: 0,
            current: (0, 0, 0),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(320.0, 500.0)),
        initial_window_size: Some(egui::vec2(320.0, 500.0)),
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

    let saved = Arc::new(Mutex::new(Vec::<Entry>::new()));
    let saved1 = saved.clone();
    let saved2 = saved.clone();
    let saved3 = saved.clone();

    let identifier = Arc::new(Mutex::new(String::from("foobar")));
    let identifier1 = identifier.clone();

    let pixels = Arc::new(Mutex::new(HashMap::<(i32, i32), (u8, u8, u8)>::new()));
    let pixels1 = pixels.clone();
    let pixels2 = pixels.clone();

    // thread::spawn(move || {
    //     loop {
    //         let inner = saved3.lock().unwrap();
    //         *pixels2.lock().unwrap() = read_pixels(inner.iter().map(|e| (e.pixel.x, e.pixel.y)).collect());
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    inputbot::MouseButton::LeftButton.bind(move || {
        if selecting1.fetch_and(false, Ordering::SeqCst) {
            println!("selected");
            let mut guard = identifier.lock().unwrap();
            saved2.lock().unwrap()
                .push(Entry::new(
                    &guard.clone(),
                    SensitivePixel::from_mouse_position().unwrap(),
                ));
            *guard = String::new();
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
            ui.monospace(
                egui::RichText::new(
                    format!("current: {:4} {:4} #{:02x}{:02x}{:02x}", s.x, s.y, s.r, s.g, s.b)
                ).color(egui::Color32::from_rgb(s.r, s.g, s.b))
            );
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    {
                        let mut inner = saved1.lock().unwrap();
                        inner.retain_mut(|t| {
                            if let Some((r, g, b)) = pixels1.lock().unwrap().get(&(t.pixel.x, t.pixel.y)) {
                                t.current = (*r, *g, *b);
                                let matching = (t.pixel.r, t.pixel.g, t.pixel.b) == t.current;
                                if !matching && t.cooldown <= 0 {
                                    println!("{}", t.id);
                                    t.cooldown += 5;
                                }
                                if matching && t.cooldown > 0 { t.cooldown -= 1; }
                            }
                            ui.horizontal(|ui| {
                                let mut keep = true;
                                if ui.button("delete").clicked() {
                                    keep = false;
                                }
                                let matching = (t.pixel.r, t.pixel.g, t.pixel.b) == t.current;
                                ui.monospace(
                                    egui::RichText::new(
                                        format!(
                                            "{} {:4} {:4} #{:02x}{:02x}{:02x} {}",
                                            if matching { "." } else { "X" },
                                            t.pixel.x, t.pixel.y,
                                            t.pixel.r, t.pixel.g, t.pixel.b,
                                            t.id,
                                        ),
                                    ).color(egui::Color32::from_rgb(t.current.0, t.current.1, t.current.2))
                                );
                                keep
                            }).inner
                        });
                    }
                    if selecting.load(Ordering::Relaxed) {
                        ui.monospace(egui::RichText::new("click anywhere...").color(egui::Color32::RED));
                    } else {
                        ui.horizontal(|ui| {
                            let mut guard = identifier1.lock().unwrap();
                            let mut target = guard.clone();
                            let select_button = ui.button("select");
                            ui.text_edit_singleline(&mut target);
                            if select_button.clicked() && !target.is_empty() {
                                println!("selecting");
                                selecting.store(true, Ordering::SeqCst);
                            }
                            *guard = target;
                        });
                    }
                });
            });
            ui.separator();
            ui.monospace("how to use: enter a string above, click
select, and then click anywhere on screen.
the current color of that pixel will be
stored. whenever that pixel changes color,
the string will be sent via serial.");
        });
        ctx.request_repaint();
    })
}
