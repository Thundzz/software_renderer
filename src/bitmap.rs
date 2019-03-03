use std::mem;

pub struct BitMap {
    pub width : i32,
    pub height : i32,
    pub  pixels : Vec<sdl2::pixels::Color>
}

impl BitMap {
    pub fn new(width : i32, height : i32) -> Self {
        let size  = (width * height) as usize;
        BitMap {
            width,
            height,
            pixels : vec![sdl2::pixels::Color::RGB(0,0,0); size]
        }
    }

    pub fn replace(mut self, x : i32, y : i32, color : sdl2::pixels::Color) -> Self {
        let index: usize = (y * self.height + x) as usize;
        mem::replace(&mut self.pixels[index], color);
        return self;
    }
}