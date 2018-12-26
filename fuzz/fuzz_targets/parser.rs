#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate cel_rs;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = cel_rs::parse(s);
    }
});
