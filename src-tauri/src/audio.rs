use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink};

pub fn play_audio(name: &str) {
	let focus_sound = include_bytes!("../assets/audio/focus.ogg").as_ref();
	let break_sound = include_bytes!("../assets/audio/break.ogg").as_ref();
	let long_break_sound = include_bytes!("../assets/audio/long_break.ogg").as_ref();

	let sound = match name {
		"focus" => focus_sound,
		"break" => break_sound,
		"long_break" => long_break_sound,
		_ => panic!("Invalid sound name"),
	};

	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let source = Decoder::new(Cursor::new(sound)).unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();
	sink.append(source);
	sink.sleep_until_end();
}
