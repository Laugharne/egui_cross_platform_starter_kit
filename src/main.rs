#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
use egui::IconData;

mod app;
use app::MyApp;

// desktop version
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {

	let options: eframe::NativeOptions = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([400.0, 300.0])
			.with_icon(load_icon()),
		..Default::default()
	};

	eframe::run_native(
		"egui (Desktop App)",
		options,
		Box::new(|cc| {
			cc.egui_ctx.set_visuals(egui::Visuals::dark());
			Ok(Box::new(MyApp::default()))
		}),
	)
}


// icon loading function (for desktop)
#[cfg(not(target_arch = "wasm32"))]
fn load_icon() -> IconData {

	let (icon_rgba, icon_width, icon_height) = {
		let icon: &[u8] = include_bytes!("../assets/icon.png");
		let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::load_from_memory(icon)
			.expect("Failed to load icon")
			.into_rgba8();

		let (width, height) = image.dimensions();
		let rgba: Vec<u8>   = image.into_raw();

		(rgba, width, height)
	};

	IconData {
		rgba  : icon_rgba,
		width : icon_width,
		height: icon_height,
	}
}


// web version
#[cfg(target_arch = "wasm32")]
fn main() {

	// Redirect Rust `panic` to console.error into web browser
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	// put trace on console log
	web_sys::console::log_1(&"Start `egui` application...".into());

	use eframe::wasm_bindgen::JsCast as _;

	let web_options = eframe::WebOptions::default();

	wasm_bindgen_futures::spawn_local(async {
		let document = web_sys::window()
			.expect("No window")
			.document()
			.expect("No document");

		let canvas = document
			.get_element_by_id("the_canvas_id")
			.expect("Failed to find the_canvas_id")
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.expect("the_canvas_id was not a HtmlCanvasElement");

		let start_result = eframe::WebRunner::new()
			.start(
				canvas,
				web_options,
				Box::new(|cc| {
					cc.egui_ctx.set_visuals(egui::Visuals::dark());
					Ok(Box::new(MyApp::default()))
				}),
			)
			.await;

		let loading_text = web_sys::window()
			.and_then(|w| w.document())
			.and_then(|d| d.get_element_by_id("loading_text"));
		if let Some(loading_text) = loading_text {
			match start_result {
				Ok(_) => {
					loading_text.remove();
				}
				Err(e) => {
					loading_text.set_inner_html(
						"<p> The app has crashed. See the developer console for details. </p>",
					);
					panic!("Failed to start eframe: {e:?}");
				}
			}
		}
	});
}

