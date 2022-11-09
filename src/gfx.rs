use num::clamp;

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 480;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Pixel(pub u32);

impl Pixel {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(
            ((a as u32) << (3 * 8))
                | ((b as u32) << (2 * 8))
                | ((g as u32) << (1 * 8))
                | ((r as u32) << (0 * 8)),
        )
    }
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(
            ((255 as u32) << (3 * 8))
                | ((b as u32) << (2 * 8))
                | ((g as u32) << (1 * 8))
                | ((r as u32) << (0 * 8)),
        )
    }
}

#[repr(C)]
pub struct Display {
    pub(crate) pixels: [Pixel; WIDTH * HEIGHT],
}

impl Display {
    pub fn fill(&mut self, pixel: Pixel) {
        for pixel_ref in self.pixels.iter_mut() {
            *pixel_ref = pixel;
        }
    }

    pub fn fill_point(&mut self, x: i32, y: i32, pixel: Pixel) {
        let x1 = clamp(x, 0, (WIDTH - 1) as i32) as usize;
        let y1 = clamp(y, 0, (HEIGHT - 1) as i32) as usize;
        if let Some(pixel_ref) = self.pixels.get_mut(y1 * WIDTH + x1) {
            *pixel_ref = pixel
        }
    }

    pub fn fill_rect(&mut self, x0: i32, y0: i32, w: i32, h: i32, pixel: Pixel) {
        let x1 = clamp(x0, 0, (WIDTH - 1) as i32) as usize;
        let x2 = clamp(x0 + w - 1, 0, (WIDTH - 1) as i32) as usize;
        let y1 = clamp(y0, 0, (HEIGHT - 1) as i32) as usize;
        let y2 = clamp(y0 + h - 1, 0, (HEIGHT - 1) as i32) as usize;

        for y in y1..=y2 {
            for x in x1..=x2 {
                if let Some(pixel_ref) = self.pixels.get_mut(y * WIDTH + x) {
                    *pixel_ref = pixel
                }
            }
        }
    }

    pub fn fill_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, pixel: Pixel) {
        // https://www.cc.gatech.edu/grads/m/Aaron.E.McClennen/Bresenham/code.html
        let dx = x1 - x0;
        let dy = y1 - y0;
        let incr_e = 2 * dy;
        let incr_ne = 2 * (dy - dx);
        let mut d = 2 * dy - dx;
        let mut x = x0;
        let mut y = y0;
        self.fill_point(x, y, pixel);
        while x < x1 {
            if d <= 0 {
                d += incr_e;
                x += 1;
            } else {
                d += incr_ne;
                x += 1;
                y += 1;
            }
            self.fill_point(x, y, pixel);
        }
    }

    pub fn put(&mut self, x: i32, y: i32, pixel: Pixel) {
        let index = y as usize * WIDTH + x as usize;
        if let Some(pixel_ref) = self.pixels.get_mut(index) {
            *pixel_ref = pixel;
        }
    }
}
