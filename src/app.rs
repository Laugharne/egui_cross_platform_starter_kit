use eframe::egui;

// #[derive(Default)]
pub struct MyApp {
	name     : String,
	age      : u32,
	counter  : i32,
	dark_mode: bool,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			name     : String::new(),
			age      : 18,
			counter  : 42,
			dark_mode: true,
		}
	}
}

impl eframe::App for MyApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

		let mut _visuals = if self.dark_mode {
			ui.ctx().set_visuals(egui::Visuals::dark());
		} else {
			ui.ctx().set_visuals(egui::Visuals::light());
		};


		egui::CentralPanel::default().show_inside(ui, |ui| {
			ui.heading("Welcome to egui!");

			ui.separator();

			ui.horizontal(|ui| {
				ui.label("Your name: ");
				ui.text_edit_singleline(&mut self.name);
			});

			ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

			if ui.button("Increment Counter").clicked() {
				self.counter += 1;
			}

			ui.label(format!("Counter: {}", self.counter));

			ui.separator();

			ui.heading("Theme Preference");
			ui.horizontal(|ui| {
				ui.label("Appearance:");
				ui.checkbox(&mut self.dark_mode, "Dark mode");
			});

			ui.separator();

			if !self.name.is_empty() {
				ui.label(format!("Hello, {}! You are {} years old.", self.name, self.age));
			}
		});

	}
}
