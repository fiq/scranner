use eframe::egui;
use egui::{RichText, Color32};
use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;

mod scranner;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Packet Sniffer",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    packets: Arc<Mutex<Vec<scranner::PacketInfo>>>,
    is_scanning: Arc<Mutex<bool>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            packets: Arc::new(Mutex::new(Vec::new())),
            is_scanning: Arc::new(Mutex::new(false)),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let is_scanning = *self.is_scanning.lock().unwrap();

            // Start button
            if !is_scanning && ui.button("Start").clicked() {
                println!("cloning packets");
                let packets = Arc::clone(&self.packets);
                let is_scanning = Arc::clone(&self.is_scanning);

                // Start scanning in a separate thread
                *is_scanning.lock().unwrap() = true;
                thread::spawn(move || {
                    println!("Spawned");
                    if let Ok(captured_packets) = scranner::sniff("enp7s0".to_string(), 4) {
                        println!("post scanning");
                        for packet in captured_packets {
                            println!("packet captured");
                            if !*is_scanning.lock().unwrap() {
                                break;
                            }
                            packets.lock().unwrap().push(packet);
                        }
                    }
                });
            }

            // Stop button
            if is_scanning && ui.button("Stop").clicked() {
                *self.is_scanning.lock().unwrap() = false;
            }

            // Separator and scroll area for packets
            ui.separator();
            ui.label("packets");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("packet area");
                display_packets(self, ui);  
            });
            ui.separator();
        });


        ctx.request_repaint();
    }
}

//, packet_info: & scranner::PacketInfo) {
fn display_packets(app: &mut MyApp, ui: &mut egui::Ui) {
    egui::Grid::new("Packets").show(ui, |ui| {
        for packet in app.packets.lock().unwrap().iter().rev() {
            ui.label(RichText::new("date:").color(Color32::LIGHT_GRAY));
            ui.label(RichText::new(format!("{}", packet.date)).color(Color32::DARK_GRAY));
            ui.label(RichText::new("src:").color(Color32::LIGHT_GRAY));
            ui.label(RichText::new(format!("{}", packet.src_mac)).color(Color32::DARK_GRAY));
            ui.label(RichText::new("dst:").color(Color32::LIGHT_GRAY));
            ui.label(RichText::new(format!("{}", packet.dst_mac)).color(Color32::DARK_GRAY));
            ui.label(RichText::new("src port:").color(Color32::LIGHT_GRAY));
            ui.label(RichText::new(format!("{}", packet.src_port)).color(Color32::DARK_GRAY));
            ui.label(RichText::new("dst port:").color(Color32::LIGHT_GRAY));
            ui.label(RichText::new(format!("{}", packet.dst_port)).color(Color32::DARK_GRAY));
            ui.end_row();
        }
    });
}


