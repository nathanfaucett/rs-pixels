use pixel::Pixel;


pub trait Pixels {

    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn get(&self, x: usize, y: usize) -> &Pixel;
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Pixel;
    fn set(&mut self, x: usize, y: usize, pixel: &Pixel);
}
