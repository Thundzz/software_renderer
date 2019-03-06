use crate::bitmap::BitMap;
use crate::star::StarField;
use crate::star::Star;

pub struct MyRenderer;


impl MyRenderer  {
    pub fn render<'a, 'b>(star_field: &'a StarField, bitmap : &'b mut BitMap) -> &'b BitMap {
        let width = 1024;
        let height = 1024;

        let white_pixel = sdl2::pixels::Color::RGB(255, 255,255);

        for star in star_field.stars.iter() {

            let x = (star.x * width as f64).floor() as i32;
            let y = (star.y * height as f64).floor() as i32;
            //println!("{} {}", x, y);
            bitmap.replace(x, y % height, white_pixel);
        }
        return bitmap;
    }
}