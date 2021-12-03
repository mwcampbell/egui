use eframe::{egui, epi};

struct MyApp {
    name: String,
    age: u32,
    checked: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            checked: false,
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "My egui App"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { name, age, checked } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let label = ui.label("Your name: ");
                ui.text_edit_singleline(name).labelled_by(label.id);
            });
            ui.add(egui::Slider::new(age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                *age += 1;
            }
            ui.label(format!("Hello '{}', age {}", name, age));
            ui.checkbox(checked, "Check me");
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);
}
