use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;

mod scranner;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Packet Sniffer",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    packets: Arc<Mutex<Vec<String>>>,
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
                    if let Ok(captured_packets) = scranner::sniff("enp7s0".to_string()) {
                        for packet in captured_packets {
                            println!("packet captured");
                            if !*is_scanning.lock().unwrap() {
                                break;
                            }
                            let packet_info = format!(
                                "Src: {}:{} -> Dst: {}:{}",
                                packet.src_ip, packet.src_port, packet.dst_ip, packet.dst_port
                            );
                            println!("Adding a packet {}", packet_info.to_string());
                            packets.lock().unwrap().push(packet_info);
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
                for packet in self.packets.lock().unwrap().iter().rev() {
                    println!("Got a packet");
                    ui.label(packet);
                }
            });
            ui.separator();
        });


        ctx.request_repaint();
    }
}
