#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = moosicbox_lofty::FileType::from_buffer(data);
});
