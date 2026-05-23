#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use xunil::{print, time::sleep_ms};

static BAD_APPLE_ASCII: &'static str = include_str!("../assets/bad_apple_ascii.txt");

fn split_ascii(s: &str, n: usize) -> Vec<&str> {
    (0..s.len())
        .step_by(n)
        .map(|i| &s[i..usize::min(i + n, s.len())])
        .collect()
}

#[unsafe(no_mangle)]
extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    for frame in BAD_APPLE_ASCII.split("NEWFRAME") {
        print("\x1b[2J");

        for text in split_ascii(frame, 100) {
            print(text);
            print("\n");
        }

        unsafe { sleep_ms(1000 / 30) };
    }

    0
}
