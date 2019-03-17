use crate::bitmap::BitMap;
use crate::vertex::Vertex;

use std::f64;
use std::vec;

pub struct Renderer
{
    width : u32,
    height : u32,
    scan_buffer : Vec<u32>
}

impl Renderer  {

    pub fn new(_width : u32, _height : u32) -> Self {
        Renderer { 
            width : _width, 
            height : _height,
            scan_buffer : vec![0; 2*_height as usize]
        }
    }

    pub fn rasterize_triangle(&mut self, v1 : Vertex, v2 : Vertex, v3 : Vertex) {
        let vmin = v1;
        let vmid = v2;
        let vmax = v3;
        
        let (vmin, vmax) = if vmin.y() > vmax.y() { (vmax, vmin) } else { (vmin, vmax) };
        let (vmid, vmin) = if vmid.y() < vmin.y() { (vmin, vmid) } else { (vmid, vmin) };
        let (vmax, vmid) = if vmax.y() < vmid.y() { (vmid, vmax) } else { (vmax, vmid) };
        
        let vec1_x = vmid.x() as i32 - v1.x() as i32;
        let vec1_y = vmid.y() as i32 - v1.y() as i32;
        let vec2_x = vmid.x() as i32 - v2.x() as i32;
        let vec2_y = vmid.y() as i32 - v2.y() as i32;

        let det = vec1_x * vec2_y - vec1_y * vec2_x;
        let handedness = if det >= 0 { 0 } else { 1 };
        
        self.rasterize_triangle_ordered(vmin, vmid, vmax, handedness);
    }


    pub fn rasterize_triangle_ordered(&mut self, v1 : Vertex, v2 : Vertex, v3 : Vertex, hand : u32) {
        self.rasterize_line(v1, v2, 1 - hand);
        self.rasterize_line(v1, v3, hand);
        self.rasterize_line(v2, v3, 1 - hand);
    }
    
    fn rasterize_line(&mut self, v1 : Vertex, v2 : Vertex, whichside : u32) {
        assert!(whichside == 0 || whichside == 1);
        let xstart = v1.x() as f64;
        let xend = v2.x() as f64;
        let ystart : f64 = v1.y() as f64;
        let yend : f64 = v2.y() as f64;
        let xstep : f64 = (xend - xstart) / (yend-ystart);

        let mut xcurrent= xstart;

        for y in v1.y()..v2.y() {
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

}