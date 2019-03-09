

pub struct BitMap<'a> {
    pub width : u32,
    pub height : u32,
    canvas :  &'a mut sdl2::render::Canvas<sdl2::video::Window>
}

impl<'b> BitMap<'b> {
    pub fn new(_width : u32, _height : u32, _canvas : &'b mut sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        let size  = (_width * _height) as usize;
        BitMap {
            width : _width,
            height : _height,
            canvas : _canvas
        }
    }


    pub fn replace(&mut self, x : u32, y : u32, color : sdl2::pixels::Color) -> &Self {
        let index: usize = (y * self.width + x) as usize;
        let point = sdl2::rect::Point::new(x as i32, y as i32);
        self.canvas.set_draw_color(color);
        self.canvas.draw_point(point);
        return self;
    }

    pub fn clear(&mut self, color: sdl2::pixels::Color) -> &Self {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        return self;
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}