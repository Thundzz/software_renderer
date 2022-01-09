use crate::bitmap::BitMap;
use crate::vertex::Vertex;

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

    fn screen_space_matrix(&self) -> glm::Mat4 {
        let half_width = self.width as f32 / 2.0;
        let half_height = self.height as f32 / 2.0;
        glm::mat4(
            half_width , 0.0, 0.0, half_width,
            0.0 , -half_height, 0.0, half_height,
            0.0 , 0.0, 1.0, 0.0,
            0.0 , 0.0, 0.0, 1.0,
        )
    }

    pub fn render_triangle<'a, 'b>(&mut self, bitmap : &'b mut BitMap<'a>, v1 : Vertex, v2 : Vertex, v3 : Vertex) -> &'b mut BitMap<'a> {
        let vmin = v1.transform(self.screen_space_matrix()).perspective_divide();
        let vmid = v2.transform(self.screen_space_matrix()).perspective_divide();
        let vmax = v3.transform(self.screen_space_matrix()).perspective_divide();

        let (vmin, vmax) = if vmin.y() > vmax.y() { (vmax, vmin) } else { (vmin, vmax) };
        let (vmid, vmin) = if vmid.y() < vmin.y() { (vmin, vmid) } else { (vmid, vmin) };
        let (vmax, vmid) = if vmax.y() < vmid.y() { (vmid, vmax) } else { (vmax, vmid) };

        let vec1_x = vmid.x() - vmax.x();
        let vec1_y = vmid.y() - vmax.y();
        let vec2_x = vmid.x() - vmin.x();
        let vec2_y = vmid.y() - vmin.y();

        let det = vec1_x * vec2_y - vec1_y * vec2_x;
        let handedness = if det >= 0.0 { 0 } else { 1 };

        self.rasterize_triangle_ordered(vmin, vmid, vmax, handedness);
        self.render_scanbuffer(bitmap, vmin.y().ceil() as u32, vmax.y().ceil() as u32)
    }

    fn rasterize_triangle_ordered(&mut self, v1 : Vertex, v2 : Vertex, v3 : Vertex, hand : u32) {
        self.scan_convert_line(v1, v2, 1 - hand);
        self.scan_convert_line(v1, v3, hand);
        self.scan_convert_line(v2, v3, 1 - hand);
    }

    fn scan_convert_line(&mut self, min_v : Vertex, max_v : Vertex, whichside : u32) {

        assert!(whichside == 0 || whichside == 1);
        let ystart  : u32 = min_v.y().ceil() as u32;
        let yend : u32 = max_v.y().ceil() as u32;

        if yend == ystart {
            return;

        }

        let xstep : f32 = (max_v.x() - min_v.x()) / (max_v.y() - min_v.y());
        let y_prestep : f32 = (ystart as f32) - min_v.y();
        let mut xcurrent : f32 = min_v.x() + xstep * y_prestep;


        for y in ystart..yend {
            self.scan_buffer[(2*y + whichside) as usize] = xcurrent.ceil() as u32;
            xcurrent += xstep;
        }
    }


    pub fn render_scanbuffer<'a, 'b>(&self, bitmap : &'b mut BitMap<'a>, ymin : u32, ymax : u32) -> &'b mut BitMap<'a> {
        let white_pixel = sdl2::pixels::Color::RGB(255, 255,255);

        for y in ymin..ymax {
            let xmin = self.scan_buffer[(2*y) as usize];
            let xmax = self.scan_buffer[(2*y +1) as usize];
            for x in xmin..xmax {
                bitmap.replace(x as u32, y as u32, white_pixel);
            }
        }
        bitmap
    }

}