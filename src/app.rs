use eframe::egui;
use serde::{Deserialize, Serialize};

const APP_KEY: &str = "egui_cross_platform_starter_kit";

#[derive(Serialize, Deserialize)]
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

impl MyApp {
	/// Restore states from the persistant storage, or use the default values.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		if let Some(storage) = cc.storage {
			if let Some(state) = eframe::get_value::<Self>(storage, APP_KEY) {
				return state;
			}
		}
		Self::default()
	}
}

impl eframe::App for MyApp {

	/// Called automatically by eframe before closing or at regular intervals.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, APP_KEY, self);
	}

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

			let symbol = if self.dark_mode { "🌙 Dark" } else { "🌞 Light" };
			ui.horizontal(|ui| {
				ui.label("Theme Preference:");
				ui.checkbox(&mut self.dark_mode, symbol);
			});

			ui.separator();

			if !self.name.is_empty() {
				ui.label(format!("Hello, {}! You are {} years old.", self.name, self.age));
			}
		});
	}
}
