#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = moosicbox_lofty::Picture::from_ape_bytes("Cover Art (Front)", data);
});