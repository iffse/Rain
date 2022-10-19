use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::command;
use tauri::api::notification::Notification;

use crate::audio::{play_audio};

const FOCUS_TIME: u32= 25 * 60;
const BREAK_TIME: u32 = 5 * 60;
const LONG_BREAK_TIME: u32 = 15 * 60;


static TIMER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(FOCUS_TIME));
static IS_BREAK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static COMPLEATED_COUNT: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

#[command]
pub fn timer_update() -> u32 {
	let mut timer = TIMER.lock().unwrap();
	let mut is_break = IS_BREAK.lock().unwrap();
	let mut compleated_count = COMPLEATED_COUNT.lock().unwrap();

	if *timer == 0 {
		let context = tauri::generate_context!();
		let identifier = context.config().tauri.bundle.identifier.clone();
		if *is_break {
			*is_break = false;
			*timer = FOCUS_TIME;
			Notification::new(&identifier)
				.title("Focus Time")
				.body("Get back to work!")
				.show().unwrap();
			play_audio("focus");
		} else {
			*compleated_count += 1;
			*is_break = true;
			if *compleated_count % 4 == 0 {
				*timer = LONG_BREAK_TIME;
				Notification::new(&identifier)
					.title("Long Break")
					.body("Take a long break!")
					.show().unwrap();
				play_audio("long_break");
			} else {
				*timer = BREAK_TIME;
				Notification::new(&identifier)
					.title("Break")
					.body("Take a break!")
					.show().unwrap();
				play_audio("break");
			}
		}
		*timer as u32
	} else {
		*timer -= 1;
		*timer as u32
	}
}

#[command]
// returning array
pub fn timer_get_parameters() -> (u32, bool, u32) {
	let timer = TIMER.lock().unwrap();
	let compleated_count = COMPLEATED_COUNT.lock().unwrap();
	let is_break = IS_BREAK.lock().unwrap();
	(*timer, *is_break,*compleated_count)
}

#[command]
pub fn timer_set(time: u32) {
	let mut timer = TIMER.lock().unwrap();
	*timer = time as u32;
}

#[command]
pub fn timer_reset() -> u32 {
	let mut timer = TIMER.lock().unwrap();
	let is_break = IS_BREAK.lock().unwrap();
	let compleated_count = COMPLEATED_COUNT.lock().unwrap();
	if *is_break {
		if (*compleated_count != 0) && (*compleated_count % 4 == 0) {
			*timer = LONG_BREAK_TIME;
		} else {
			*timer = BREAK_TIME;
		}
	} else {
		*timer = FOCUS_TIME;
	}
	*timer as u32
}

#[command]
pub fn timer_skip() {
	let mut timer = TIMER.lock().unwrap();
	*timer = 0;
	drop(timer);
	timer_update();
}


