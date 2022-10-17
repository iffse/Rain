#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod tasks;
mod files;
mod timer;

use tasks::new_task;
use files::{get_filenames, write_file, load_file};
use timer::{timer_update, timer_get_parameters, timer_set, timer_reset, timer_skip};

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			new_task,
			get_filenames, load_file, write_file,
			timer_update, timer_get_parameters, timer_set, timer_reset, timer_skip
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
