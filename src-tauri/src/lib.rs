use tauri::{LogicalSize, Manager, Size};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate turbonone;

#[macro_use]
mod logging;
pub use logging::*;

#[macro_use]
pub mod globals;
pub use globals::*;

#[macro_use]
pub mod util;
pub use util::*;

#[macro_use]
pub mod transactions;
pub use transactions::Transaction;

pub mod base64_image;
pub use base64_image::Base64Image;

pub mod appdata;
pub use appdata::AppData;

pub mod game_addons;
pub use game_addons::GameAddons;

pub mod addon_size_analyzer;
pub use addon_size_analyzer::AddonSizeAnalyzer;

pub mod gma;
pub use gma::{GMAError, GMAFile, GMAMetadata};

pub mod steam;
pub use steam::workshop::WorkshopItem;

pub mod octopus;
pub use octopus::*;

pub mod content_generator;
pub mod search;
pub mod webview;

mod cli;
mod commands;

#[cfg(debug_assertions)]
fn deadlock_watchdog() {
	std::thread::spawn(move || loop {
		sleep!(10);

		let deadlocks = parking_lot::deadlock::check_deadlock();
		if deadlocks.is_empty() {
			continue;
		}

		println!("{} deadlocks detected", deadlocks.len());
		for (i, threads) in deadlocks.iter().enumerate() {
			println!("Deadlock #{}", i);
			for t in threads {
				println!("Thread Id {:#?}", t.thread_id());
				println!("{:#?}", t.backtrace());
			}
		}
	});
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	// https://github.com/WilliamVenner/gmpublisher/issues/210
	if cfg!(target_os = "linux") {
		std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
	}

	std::panic::set_hook(Box::new(logging::panic));

	rayon::ThreadPoolBuilder::new().num_threads(*crate::NUM_THREADS).build_global().unwrap();

	if cli::stdin() {
		return;
	}

	println!("gmpublisher v{}", env!("CARGO_PKG_VERSION"));

	#[cfg(debug_assertions)]
	deadlock_watchdog();

	globals::init_globals();

	println!("Starting GUI...");

	tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		.setup(|app| {
			let settings = APP_DATA.settings.read();

			let window = app.get_webview_window("gmpublisher").unwrap();

			window.set_title(&format!("gmpublisher v{}", env!("CARGO_PKG_VERSION"))).ok();

			window
				.set_size(Size::Logical(LogicalSize {
					width: settings.window_size.0.max(800.),
					height: settings.window_size.1.max(600.),
				}))
				.ok();

			if !cfg!(debug_assertions) && settings.window_maximized {
				window.maximize().ok();
			}

			webview!().init(window);

			Ok(())
		})
		.plugin(webview::error_reporter_plugin())
		.plugin(appdata::appdata_plugin())
		.invoke_handler(tauri::generate_handler![
			commands::check_dir,
			commands::check_file,
			commands::open,
			commands::open_file_location,
			commands::file_size,
			webview::reloaded,
			webview::js_error,
			webview::error,
			webview::info,
			webview::warn,
			transactions::websocket,
			transactions::cancel_transaction,
			appdata::update_settings,
			appdata::validate_gmod,
			appdata::window_resized,
			game_addons::browse_installed_addons,
			game_addons::get_installed_addon,
			game_addons::downloader_extract_gmas,
			steam::is_steam_connected,
			steam::get_current_user,
			steam::users::get_steam_user,
			steam::workshop::fetch_workshop_items,
			steam::workshop::fetch_workshop_item,
			steam::workshop::browse_my_workshop,
			steam::workshop::workshop_item_channel,
			steam::downloads::workshop_download,
			steam::publishing::verify_whitelist,
			steam::publishing::publish,
			steam::publishing::verify_icon,
			steam::publishing::publish_icon,
			steam::subscriptions::browse_subscribed_addons,
			addon_size_analyzer::addon_size_analyzer,
			content_generator::get_content_generator_manifests,
			content_generator::update_content_generator_manifest,
			gma::preview::preview_gma,
			gma::preview::extract_preview_entry,
			gma::preview::extract_preview_gma,
			gma::extract::extract_gma,
			search::search,
			search::search_channel,
			search::full_search,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");

	println!("Goodbye!");
}
