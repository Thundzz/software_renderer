use crate::bitmap::BitMap;
use crate::star::StarField;
use crate::star::Star;


pub trait Renderer {
    fn render(star_field: StarField) -> BitMap {
        let mut bitmap =  BitMap::new(1,1);
        let white_pixel = sdl2::pixels::Color::RGB(1, 1,1);

        for star in star_field.stars.iter() {
            // let point = Point::new(i, i + j as i32);
            bitmap = bitmap.replace(star.x, star.y, white_pixel);
        }
        return bitmap;
    }
}