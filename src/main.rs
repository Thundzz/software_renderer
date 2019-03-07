mod star;
mod renderer;
mod bitmap;


use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::video::Window;



fn move_stars(starfield : &mut star::StarField) {
    for i in 0..starfield.stars.len() {
        starfield.stars[i].z -= 0.8;
        if starfield.stars[i].z <= 0.0 {
            starfield.stars[i] = star::Star::random(128.0);
        }
    }
}

pub fn main() {

    let res_x = 800 as u32;
    let res_y = 600 as u32;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window: Window = video_subsystem.window("Super starfield", res_x, res_y)
        .position_centered()
        .build()
        .unwrap();


    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut starfield : star::StarField = star::StarField::new_star_field(2048, 128.0);
    let mut bm : bitmap::BitMap = bitmap::BitMap::new(res_x, res_y);

    'running: loop {


        let background_color = Color::RGB(0, 0, 0);
        let white = Color::RGB(255, 255, 255);
        // Scan events
        bm.clear(background_color);
        canvas.set_draw_color(background_color);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                _ => {}
            }
        }

        move_stars(&mut starfield);
        renderer::MyRenderer::render(&starfield, &mut bm, res_x, res_y);

        let width = bm.width;
        let height = bm.height;       

        canvas.set_draw_color(white);

        for x in 0..width {
            for y in 0..height {
                let color = bm.get(x, y);
                
                if !(color.r == 0 && color.g == 0 && color.b == 0) {
                    let point = Point::new(x as i32, y as i32);
                    let res = canvas.draw_point(point);
                }
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 16_000_000));
    }
}





