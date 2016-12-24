extern crate vector;
extern crate stack;

extern crate pixels;


use vector::Vector;
use stack::Stack;

use pixels::{Pixel, Pixels};


pub fn new_pixels(width: usize, height: usize) -> Vector<Vector<[f32; 4]>> {
    let mut pixels = Vector::with_capacity(width);

    for _ in 0..width {
        let mut cols = Vector::with_capacity(height);

        for _ in 0..height {
            cols.push([0f32, 0f32, 0f32, 1f32]);
        }

        pixels.push(cols);
    }

    pixels
}

#[test]
fn test_pixel() {
    let mut pixels = new_pixels(2, 2);

    pixels.get_mut(0, 0).set(&[0f32, 0f32, 0f32, 0f32]);
    pixels.get_mut(0, 1).set(&[0f32, 1f32, 0f32, 0f32]);
    pixels.get_mut(1, 0).set(&[0f32, 0f32, 1f32, 0f32]);
    pixels.get_mut(1, 1).set(&[0f32, 0f32, 0f32, 1f32]);

    assert_eq!(pixels.get(0, 0).to_u32(), 0x00000000);
    assert_eq!(pixels.get(0, 1).to_u32(), 0x00FF0000);
    assert_eq!(pixels.get(1, 0).to_u32(), 0x0000FF00);
    assert_eq!(pixels.get(1, 1).to_u32(), 0x000000FF);

    assert_eq!(pixels.to_u32(), &[0x00000000, 0x00FF0000, 0x0000FF00, 0x000000FF]);
}
