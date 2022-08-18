use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct Task {
	id: String,
	title: String,
	compleated: bool,
}

#[tauri::command]
pub fn new_task(title: String) -> Task {
	Task {
		id: Uuid::new_v4().to_string(),
		title,
		compleated: false,
	}
}
