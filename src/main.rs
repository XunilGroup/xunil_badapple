#![no_std]
#![no_main]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use xunil::{
    graphics::{font_render::render_text, framebuffer::WindowFrameBuffer, rgb},
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
    let window = unsafe { request_window() };

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

        let formatted_frame = split_ascii(frame, 140);
        render_text(
            &mut back_fb,
            0,
            0,
            &formatted_frame,
            1,
            rgb(255, 255, 255),
            0,
        );

        let window_fb = WindowFrameBuffer::from_window(&window);
        unsafe {
            core::ptr::copy_nonoverlapping(
                back_buffer.as_ptr(),
                window_fb.ptr,
                window.width * window.height,
            );
        }

        set_dirty();
        unsafe { sleep_ms(1000 / 25) };
    }

    0
}
