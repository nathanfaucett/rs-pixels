use collections::vec::Vec;

use vector::Vector;
use insert::Insert;
use collection::Collection;

use pixel::Pixel;


pub trait Pixels {

    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn get(&self, x: usize, y: usize) -> &Pixel;
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Pixel;

    fn set(&mut self, x: usize, y: usize, pixel: &Pixel) {
        self.get_mut(x, y).set(pixel);
    }

    fn to_u32(&self) -> Vector<u32> {
        let width = self.get_width();
        let height = self.get_height();
        let size = width * height;
        let mut slice = Vector::with_capacity(size);
        let mut i = 0;

        for x in 0..width {
            for y in 0..height {
                slice.insert(i, self.get(x, y).to_u32());
                i += 1;
            }
        }

        slice
    }
    fn to_u64(&self) -> Vector<u64> {
        let width = self.get_width();
        let height = self.get_height();
        let size = width * height;
        let mut slice = Vector::with_capacity(size);
        let mut i = 0;

        for x in 0..width {
            for y in 0..height {
                slice.insert(i, self.get(x, y).to_u64());
                i += 1;
            }
        }

        slice
    }
}

impl<T: Pixel> Pixels for Vector<Vector<T>> {

    fn get_width(&self) -> usize { self.len() }
    fn get_height(&self) -> usize { if self.len() != 0 { self[0].len() } else { 0usize } }

    fn get(&self, x: usize, y: usize) -> &Pixel { &self[x][y] }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Pixel { &mut self[x][y] }
}
impl<T: Pixel> Pixels for Vec<Vec<T>> {

    fn get_width(&self) -> usize { self.len() }
    fn get_height(&self) -> usize { if self.len() != 0 { self[0].len() } else { 0usize } }

    fn get(&self, x: usize, y: usize) -> &Pixel { &self[x][y] }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Pixel { &mut self[x][y] }
}
