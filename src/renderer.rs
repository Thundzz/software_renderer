use crate::bitmap::BitMap;
use crate::star::StarField;
use crate::star::Star;

pub struct MyRenderer;


impl MyRenderer  {
    
    pub fn render<'a, 'b>(star_field: &'a StarField, bitmap : &'b mut BitMap, _width : u32, _height : u32) -> &'b BitMap {
        let width = _width as f64;
        let height = _height as f64;

        let halfWidth : f64 = width / 2.0 as f64;
        let halfHeight : f64 = height / 2.0 as f64;

        let white_pixel = sdl2::pixels::Color::RGB(255, 255,255);

        for star in star_field.stars.iter() {

            let x = (star.x / star.z) * halfWidth + halfWidth  as f64;
            let y = (star.y / star.z) * halfHeight + halfHeight as f64;
            //println!("{} {}", x, y);

            if (x >= 0.0 && x < width) && (y >= 0.0 && y < height){
                bitmap.replace(x.floor() as u32, y.floor() as u32, white_pixel);
            }
        }
        return bitmap;
    }
}