use crate::bitmap::BitMap;
use crate::star::StarField;
use std::f64;


pub struct MyRenderer;


impl MyRenderer  {
    

    fn to_randians(degrees: f64) -> f64 {
        return f64::consts::PI * degrees / 180.0;
    }

    pub fn render<'a, 'b>(star_field: &StarField, bitmap : &'b mut BitMap<'a>, _width : u32, _height : u32) -> &'b mut BitMap<'a> {

        let fovDeg = 170.0;
        let tanHalfFov = MyRenderer::to_randians(fovDeg / 2.0).tan();

        let width = _width as f64;
        let height = _height as f64;

        let half_width : f64 = width / 2.0 as f64;
        let half_height : f64 = height / 2.0 as f64;

        let white_pixel = sdl2::pixels::Color::RGB(255, 255,255);

        for star in star_field.stars.iter() {

            let x = (star.x / (star.z * tanHalfFov)) * half_width + half_width  as f64;
            let y = (star.y / (star.z * tanHalfFov)) * half_height + half_height as f64;

            // let x = (star.x) * half_width + half_width  as f64;
            // let y = (star.y) * half_height + half_height as f64;

            if (x >= 0.0 && x < width) && (y >= 0.0 && y < height){
                bitmap.replace(x.floor() as u32, y.floor() as u32, white_pixel);
            }
        }
        return bitmap;
    }
}