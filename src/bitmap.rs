

pub struct BitMap {
    pub width : u32,
    pub height : u32,
    pub  pixels : Vec<sdl2::pixels::Color>
}

impl BitMap {
    pub fn new(_width : u32, _height : u32) -> Self {
        let size  = (_width * _height) as usize;
        BitMap {
            width : _width,
            height : _height,
            pixels : vec![sdl2::pixels::Color::RGB(0,0,0); size]
        }
    }

    pub fn get(&self, x : u32, y : u32) -> sdl2::pixels::Color {
        let index: usize = (y * self.width + x) as usize;
        return self.pixels[index];
    }

    pub fn replace(&mut self, x : u32, y : u32, color : sdl2::pixels::Color) -> &Self {
        let index: usize = (y * self.width + x) as usize;
        self.pixels[index] = color;
        return self;
    }

    pub fn clear(&mut self, color: sdl2::pixels::Color) -> &Self {
        for x in 0..self.width {
            for y in 0..self.height {
                let index: usize = (y * self.width + x) as usize;
                self.pixels[index] = color;
            }
        }
        return self;
    }
}