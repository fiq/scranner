use eframe::egui;

fn main() -> Result<(), eframe::Error> {
  return eframe::run_native(
    "scranner",
    eframe::NativeOptions::default(),
    Box::new(|_| Ok(Box::new(MyApp::default()))),
  )
}

struct MyApp {
  name: String,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      name: String::from("User"),
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _:&mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      if ui.button("Scan").clicked() {
        println!("presssed {}", self.name);
      };
    });
    return;
  }
}
