#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod tasks;
mod files;

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			tasks::new_task,
			files::get_filenames,
			files::load_file,
			files::write_file
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
