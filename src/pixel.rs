

pub trait Pixel {
    fn to_u32(&self) -> u32;
    fn from_u32(&mut self, value: u32);
}
