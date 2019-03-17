use crate::bitmap::BitMap;
use crate::star::StarField;
use crate::vertex::Vertex;

use std::f64;
use std::vec;

pub struct Renderer
{
    width : u32,
    height : u32,
    scan_buffer : Vec<u32>
}

enum Handedness {

}

impl Renderer  {
    

    pub fn new(_width : u32, _height : u32) -> Self {
        Renderer { 
            width : _width, 
            height : _height,
            scan_buffer : vec![0; 2*_height as usize]
        }
    }

    fn to_randians(degrees: f64) -> f64 {
        f64::consts::PI * degrees / 180.0
    }

    pub fn set_trapezoid_in_scanbuffer(&mut self) {
        for y in 50..100 {
            self.scan_buffer[(2*y) as usize] = 300 - y;
            self.scan_buffer[(2*y +1) as usize] = 300 + y;
        }
    }

    pub fn rasterize_triangle(&mut self, v1 : Vertex, v2 : Vertex, v3 : Vertex) {
        self.rasterize_line(v1, v2, 0);
        self.rasterize_line(v1, v3, 1);
        self.rasterize_line(v3, v2, 1);
    }
    
    fn rasterize_line(&mut self, v1 : Vertex, v2 : Vertex, whichside : u32) {
        assert!(whichside == 0 || whichside == 1);
        let xstart = v1.x as f64;
        let xend = v2.x as f64;
        let ystart : f64 = v1.y as f64;
        let yend : f64 = v2.y as f64;
        let xstep : f64 = (xend - xstart) / (yend-ystart);

        let mut xcurrent= xstart;

        for y in v1.y..v2.y {
            self.scan_buffer[(2*y + whichside) as usize] = xcurrent.floor() as u32;
            xcurrent += xstep;
        }

    }


    pub fn render_scanbuffer<'a, 'b>(&self, bitmap : &'b mut BitMap<'a>) -> &'b mut BitMap<'a> {
        let white_pixel = sdl2::pixels::Color::RGB(255, 255,255);

        for y in 1..self.height {
            let xmin = self.scan_buffer[(2*y) as usize];
            let xmax = self.scan_buffer[(2*y +1) as usize];
            for x in xmin..xmax {
                bitmap.replace(x as u32, y as u32, white_pixel);
            }
        }
        bitmap
    }

    pub fn render<'a, 'b>(&self,
                          star_field: &StarField, 
                          bitmap : &'b mut BitMap<'a>) -> &'b mut BitMap<'a> {

        let fov_deg = 170.0;
        let tan_half_fov = Renderer::to_randians(fov_deg / 2.0).tan();

        let width = self.width as f64;
        let height = self.height as f64;

        let half_width : f64 = width / 2.0 as f64;
        let half_height : f64 = height / 2.0 as f64;

        let white_pixel = sdl2::pixels::Color::RGB(255, 255,255);

        for star in star_field.stars.iter() {

            let x = (star.x / (star.z * tan_half_fov)) * half_width + half_width  as f64;
            let y = (star.y / (star.z * tan_half_fov)) * half_height + half_height as f64;

            if (x >= 0.0 && x < width) && (y >= 0.0 && y < height){
                bitmap.replace(x.floor() as u32, y.floor() as u32, white_pixel);
            }
        }

        bitmap
    }
}