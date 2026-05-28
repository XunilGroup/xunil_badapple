#![no_std]
#![no_main]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use xunil::{
    graphics::{framebuffer::WindowFrameBuffer, primitives::rectangle_filled, rgb},
    io::{
        time::sleep_ms,
        window::{request_window, set_dirty},
    },
};

static BAD_APPLE_ASCII: &'static str = include_str!("../assets/bad_apple_ascii.txt");

fn split_ascii(s: &str, width: usize) -> String {
    let mut result = String::new();
    let mut col = 0;

    for ch in s.chars() {
        if col >= width {
            result.push('\n');
            col = 0;
        }
        result.push(ch);
        col += 1;
    }

    result
}

#[unsafe(no_mangle)]
extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let rows_per_frame = BAD_APPLE_ASCII
        .split("NEWFRAME")
        .next()
        .map(|f| (f.len() + 99) / 100)
        .unwrap_or(75);

    let window = unsafe { request_window(4 * 100, 4 * rows_per_frame) };

    let mut back_buffer: Vec<u32> = Vec::new();
    back_buffer.resize(window.width * window.height, 0);

    for frame in BAD_APPLE_ASCII.split("NEWFRAME") {
        for pixel in back_buffer.iter_mut() {
            *pixel = rgb(0, 0, 0);
        }

        let mut back_fb = WindowFrameBuffer {
            ptr: back_buffer.as_mut_ptr(),
            width: window.width,
            height: window.height,
        };

        let formatted_frame = split_ascii(frame, 100);
        let mut y = 0;
        let mut x = 0;
        for character in formatted_frame.as_str().chars() {
            if character == '\n' {
                x = 0;
                y += 4;
                continue;
            }

            let grayness = character.to_digit(10).unwrap_or(0) as f32 * 255.0 / 9.0;

            if grayness > 0.0 {
                let color = rgb(grayness as u8, grayness as u8, grayness as u8);
                rectangle_filled(&mut back_fb, x, y, 2, 2, color);
            }

            x += 4;
        }

        let window_fb = WindowFrameBuffer::from_window(&window);
        unsafe {
            core::ptr::copy_nonoverlapping(
                back_buffer.as_ptr(),
                window_fb.ptr,
                window.width * window.height,
            );
        }

        set_dirty();
        unsafe { sleep_ms(1000 / 40) };
    }

    0
}
