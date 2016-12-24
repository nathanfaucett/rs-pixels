

pub trait Pixel {

    fn set(&mut self, &Pixel);

    fn to_u32(&self) -> u32;
    fn from_u32(&mut self, value: u32);

    fn to_u64(&self) -> u64;
    fn from_u64(&mut self, value: u64);
}


impl Pixel for u32 {
    fn set(&mut self, pixel: &Pixel) { self.from_u32(pixel.to_u32());}

    fn to_u32(&self) -> u32 { *self }
    fn from_u32(&mut self, value: u32) { *self = value; }

    fn to_u64(&self) -> u64 {
        let s = *self;
        ((((s << 24) as f64 / 255_f64) * 65536_f64) as u64) << 48 |
        ((((s << 16) as f64 / 255_f64) * 65536_f64) as u64) << 32 |
        ((((s << 8) as f64 / 255_f64) * 65536_f64) as u64) << 16 |
        (((s as f64 / 255_f64) * 65536_f64) as u64)
    }
    fn from_u64(&mut self, value: u64) {
        *self =
            ((((value >> 48) as f64 / 65535_f64) * 256_f64) as u32) << 24 |
            ((((value >> 32) as f64 / 65535_f64) * 256_f64) as u32) << 16 |
            ((((value >> 16) as f64 / 65535_f64) * 256_f64) as u32) << 8 |
            ((value as f64 / 65535_f64) * 256_f64) as u32;
    }
}

impl Pixel for u64 {
    fn set(&mut self, pixel: &Pixel) { self.from_u64(pixel.to_u64());}

    fn to_u32(&self) -> u32 {
        let s = *self;
        ((((s << 48) as f64 / 65536_f64) * 256_f64) as u32) << 24 |
        ((((s << 32) as f64 / 65536_f64) * 256_f64) as u32) << 16 |
        ((((s << 16) as f64 / 65536_f64) * 256_f64) as u32) << 8 |
        (((s as f64 / 65536_f64) * 256_f64) as u32)
    }
    fn from_u32(&mut self, value: u32) {
        *self =
            ((((value >> 24) as f64 / 256_f64) * 65536_f64) as u64) << 48 |
            ((((value >> 16) as f64 / 256_f64) * 65536_f64) as u64) << 32 |
            ((((value >> 8) as f64 / 256_f64) * 65536_f64) as u64) << 16 |
            ((value as f64 / 256_f64) * 65536_f64) as u64;
    }

    fn to_u64(&self) -> u64 { *self }
    fn from_u64(&mut self, value: u64) { *self = value }
}

impl Pixel for [u8; 4] {
    fn set(&mut self, pixel: &Pixel) { self.from_u32(pixel.to_u32());}

    fn to_u32(&self) -> u32 {
        (self[0] as u32) << 24 |
        (self[1] as u32) << 16 |
        (self[2] as u32) << 8 |
        (self[3] as u32)
    }
    fn from_u32(&mut self, value: u32) {
        self[0] = (value >> 24) as u8;
        self[1] = (value >> 16) as u8;
        self[2] = (value >> 8) as u8;
        self[3] = value as u8;
    }
    fn to_u64(&self) -> u64 {
        (((self[0] as f64 / 255_f64) * 65536_f64) as u64) << 48 |
        (((self[1] as f64 / 255_f64) * 65536_f64) as u64) << 32 |
        (((self[2] as f64 / 255_f64) * 65536_f64) as u64) << 16 |
        (((self[3] as f64 / 255_f64) * 65536_f64) as u64)
    }
    fn from_u64(&mut self, value: u64) {
        self[0] = (((value >> 48) as f64 / 65535_f64) * 256_f64) as u8;
        self[1] = (((value >> 32) as f64 / 65535_f64) * 256_f64) as u8;
        self[2] = (((value >> 16) as f64 / 65535_f64) * 256_f64) as u8;
        self[3] = ((value as f64 / 65535_f64) * 256_f64) as u8;
    }
}

impl Pixel for [u16; 4] {
    fn set(&mut self, pixel: &Pixel) { self.from_u32(pixel.to_u32());}

    fn to_u32(&self) -> u32 {
        (((self[0] as f64 / 65536_f64) * 256_f64) as u32) << 24 |
        (((self[1] as f64 / 65536_f64) * 256_f64) as u32) << 16 |
        (((self[2] as f64 / 65536_f64) * 256_f64) as u32) << 8 |
        (((self[3] as f64 / 65536_f64) * 256_f64) as u32)
    }
    fn from_u32(&mut self, value: u32) {
        self[0] = (((value >> 24) as f64 / 256_f64) * 65536_f64) as u16;
        self[1] = (((value >> 16) as f64 / 256_f64) * 65536_f64) as u16;
        self[2] = (((value >> 8) as f64 / 256_f64) * 65536_f64) as u16;
        self[3] = ((value as f64 / 256_f64) * 65536_f64) as u16;
    }
    fn to_u64(&self) -> u64 {
        (self[0] as u64) << 48 |
        (self[1] as u64) << 32 |
        (self[2] as u64) << 16 |
        self[3] as u64
    }
    fn from_u64(&mut self, value: u64) {
        self[0] = ((value >> 48) as f64 / 65535_f64) as u16;
        self[1] = ((value >> 32) as f64 / 65535_f64) as u16;
        self[2] = ((value >> 16) as f64 / 65535_f64) as u16;
        self[3] = (value as f64 / 65535_f64) as u16;
    }
}

impl Pixel for [f32; 4] {
    fn set(&mut self, pixel: &Pixel) { self.from_u64(pixel.to_u64());}

    fn to_u32(&self) -> u32 {
        ((self[0] * 255_f32) as u32) << 24 |
        ((self[1] * 255_f32) as u32) << 16 |
        ((self[2] * 255_f32) as u32) << 8 |
        ((self[3] * 255_f32) as u32)
    }
    fn from_u32(&mut self, value: u32) {
        self[0] = (value >> 24) as f32 / 255_f32;
        self[1] = (value >> 16) as f32 / 255_f32;
        self[2] = (value >> 8) as f32 / 255_f32;
        self[3] = value as f32 / 255_f32;
    }
    fn to_u64(&self) -> u64 {
        ((self[0] * 65535_f32) as u64) << 24 |
        ((self[1] * 65535_f32) as u64) << 16 |
        ((self[2] * 65535_f32) as u64) << 8 |
        ((self[3] * 65535_f32) as u64)
    }
    fn from_u64(&mut self, value: u64) {
        self[0] = (value >> 24) as f32 / 65535_f32;
        self[1] = (value >> 16) as f32 / 65535_f32;
        self[2] = (value >> 8) as f32 / 65535_f32;
        self[3] = value as f32 / 65535_f32;
    }
}

impl Pixel for [f64; 4] {
    fn set(&mut self, pixel: &Pixel) { self.from_u64(pixel.to_u64());}

    fn to_u32(&self) -> u32 {
        ((self[0] * 255_f64) as u32) << 24 |
        ((self[1] * 255_f64) as u32) << 16 |
        ((self[2] * 255_f64) as u32) << 8 |
        ((self[3] * 255_f64) as u32)
    }
    fn from_u32(&mut self, value: u32) {
        self[0] = (value >> 24) as f64 / 255_f64;
        self[1] = (value >> 16) as f64 / 255_f64;
        self[2] = (value >> 8) as f64 / 255_f64;
        self[3] = value as f64 / 255_f64;
    }
    fn to_u64(&self) -> u64 {
        ((self[0] * 65535_f64) as u64) << 24 |
        ((self[1] * 65535_f64) as u64) << 16 |
        ((self[2] * 65535_f64) as u64) << 8 |
        ((self[3] * 65535_f64) as u64)
    }
    fn from_u64(&mut self, value: u64) {
        self[0] = (value >> 24) as f64 / 65535_f64;
        self[1] = (value >> 16) as f64 / 65535_f64;
        self[2] = (value >> 8) as f64 / 65535_f64;
        self[3] = value as f64 / 65535_f64;
    }
}
