

pub struct BitMap {
    pub width : i32,
    pub height : i32,
    pub  pixels : Vec<sdl2::pixels::Color>
}

impl BitMap {
    pub fn new(_width : i32, _height : i32) -> Self {
        let size  = (_width * _height) as usize;
        BitMap {
            width : _width,
            height : _height,
            pixels : vec![sdl2::pixels::Color::RGB(0,0,0); size]
        }
    }

    pub fn get(&self, x : i32, y : i32) -> sdl2::pixels::Color {
        let index: usize = (y * self.height + x) as usize;
        return self.pixels[index];
    }

    pub fn replace(&mut self, x : i32, y : i32, color : sdl2::pixels::Color) -> &Self {
        let index: usize = (y * self.height + x) as usize;
        self.pixels[index] = color;
        return self;
    }
}