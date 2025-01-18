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
  results: Arc<Mutex<Vec<String>>>,
  scanning: Arc<Mutex<bool>>,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      results: Arc::new(Mutex::new(Vec::new())),
      scanning: Arc::new(Mutex::new(false)),
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      let scanning = *self.scanning.lock().unwrap();

      if !scanning && ui.button("Start").clicked() {
        let results = Arc::clone(&self.results);
        let scanning_flag = Arc::clone(&self.scanning);
        *scanning_flag.lock().unwrap() = true;

        thread::spawn(move || {
          if let Ok(packets) = scranner::sniff("enp7s0".to_string()) {
            for packet in packets {
              if !*scanning_flag.lock().unwrap() {
                break;
              }
              let packet_info = format!(
                "Src: {}:{} -> Dst: {}:{}",
                packet.src_ip, packet.src_port, packet.dst_ip, packet.dst_port
              );
              results.lock().unwrap().push(packet_info);
            }
          }
        });
      }

      if scanning && ui.button("Stop").clicked() {
        *self.scanning.lock().unwrap() = false;
      }

      ui.separator();
      egui::ScrollArea::vertical().show(ui, |ui| {
        for packet in self.results.lock().unwrap().iter().rev() {
          ui.label(packet);
        }
      });
    });

    ctx.request_repaint();
  }
}

