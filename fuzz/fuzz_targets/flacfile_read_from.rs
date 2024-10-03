#![no_main]

use std::io::Cursor;

use libfuzzer_sys::fuzz_target;
use moosicbox_lofty::{AudioFile, ParseOptions};

fuzz_target!(|data: Vec<u8>| {
	let _ = moosicbox_lofty::flac::FlacFile::read_from(&mut Cursor::new(data), ParseOptions::new());
});
