#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate earl;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(utf8) = str::from_utf8(data) {
        if let Ok(parsed) = earl::Url::parse(utf8) {
            let as_str = parsed.as_str();
            assert_eq!(parsed, earl::Url::parse(as_str).unwrap());
        }
    }
});
