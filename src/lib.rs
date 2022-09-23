#![no_main]

mod gfx;

use gfx::{Display, HEIGHT, WIDTH};

use crate::gfx::Pixel;

const BACKGROUND: Pixel = Pixel::rgba(20, 20, 100, 255);

static mut DISPLAY: Display = Display {
    pixels: [Pixel(0); WIDTH * HEIGHT],
};

pub fn render(display: &mut Display) {
    display.fill(BACKGROUND);
    //display.fill_line(10, 10, 50, 80, Pixel::rgba(0, 255, 255, 255));
    //display.fill_line(100, 10, 180, 400, Pixel::rgba(0, 255, 255, 255));
    display.fill_rect(
        0,
        (HEIGHT - 50) as i32,
        WIDTH as i32,
        50,
        Pixel::rgba(100, 200, 100, 255),
    );
}

// exports to javascript

#[no_mangle]
pub unsafe extern "C" fn step(dt: f32) {
    render(&mut DISPLAY);
}

#[no_mangle]
pub unsafe extern "C" fn init() {}

#[no_mangle]
pub unsafe extern "C" fn get_width() -> usize {
    WIDTH
}

#[no_mangle]
pub unsafe extern "C" fn get_height() -> usize {
    HEIGHT
}

#[no_mangle]
pub unsafe extern "C" fn get_display() -> &'static mut Display {
    &mut DISPLAY
}
