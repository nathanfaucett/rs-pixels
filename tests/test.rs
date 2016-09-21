extern crate pixels;


use pixels::*;


#[derive(Copy, Clone)]
pub struct RGBAPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl RGBAPixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBAPixel { r: r, g: g, b: b, a: a }
    }
}
impl Pixel for RGBAPixel {
    fn to_u32(&self) -> u32 {
        (self.a as u32) << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }
    fn from_u32(&mut self, value: u32) {
        self.a = (value >> 24) as u8;
        self.r = (value >> 16) as u8;
        self.g = (value >> 8) as u8;
        self.b = value as u8;
    }
}

pub struct RGBAPixels {
    pixels: Vec<Vec<RGBAPixel>>,
    width: usize,
    height: usize,
}
impl RGBAPixels {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width);

        for _ in 0..width {
            let mut cols = Vec::with_capacity(height);

            for _ in 0..height {
                cols.push(RGBAPixel::new(0, 0, 0, 0));
            }

            pixels.push(cols);
        }

        RGBAPixels {
            pixels: pixels,
            width: width,
            height: height,
        }
    }
}
impl Pixels for RGBAPixels {

    fn get_width(&self) -> usize {
        self.width
    }
    fn get_height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> &Pixel {
        &self.pixels[x][y]
    }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self.pixels[x][y]
    }
    fn set(&mut self, x: usize, y: usize, pixel: &Pixel) {
        self.pixels[x][y].from_u32(pixel.to_u32());
    }
}


#[test]
fn test_pixel() {
    let mut pixels = RGBAPixels::new(16, 16);

    for x in 0..pixels.get_width() {
        for y in 0..pixels.get_height() {
            pixels.get_mut(x, y).from_u32(0xffff0000);
        }
    }

    assert_eq!(pixels.get(0, 0).to_u32(), 0xffff0000);
    assert_eq!(pixels.get(7, 0).to_u32(), 0xffff0000);
    assert_eq!(pixels.get(0, 7).to_u32(), 0xffff0000);
    assert_eq!(pixels.get(7, 7).to_u32(), 0xffff0000);
    assert_eq!(pixels.get(15, 15).to_u32(), 0xffff0000);
}
