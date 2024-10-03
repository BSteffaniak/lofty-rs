#![no_main]
use moosicbox_lofty::ParsingMode;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
	let _ = moosicbox_lofty::Picture::from_flac_bytes(data, true, ParsingMode::Relaxed);
	let _ = moosicbox_lofty::Picture::from_flac_bytes(data, false, ParsingMode::Relaxed);
});
