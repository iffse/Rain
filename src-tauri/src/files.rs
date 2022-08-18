use std::fs::{read_dir, read_to_string, write};
use tauri::command;
use tauri::api::path::data_dir;

const LISTS_DIR: &str = "rain/lists";

#[command]
pub fn get_filenames() -> Vec<String> {
	let mut path = data_dir().unwrap();
	path.push(LISTS_DIR);

	let entries = read_dir(path).unwrap();
	let mut lists: Vec<String> = Vec::new();

	for entry in entries {
		let name = entry.unwrap().file_name().into_string().unwrap();
		lists.push(name);
	};
	lists
}

#[command]
pub fn load_file(name: String) -> String {
	let mut file = data_dir().unwrap();
	file.push(LISTS_DIR);
	file.push(name);

	read_to_string(file).unwrap()
}

#[command]
pub fn write_file(name: String, content: String) {
	let mut file = data_dir().unwrap();
	file.push(LISTS_DIR);
	file.push(name);

	write(file, content).unwrap();
}
