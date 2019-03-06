mod star;
mod renderer;
mod bitmap;


use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::video::Window;



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window: Window = video_subsystem.window("rust-sdl2 demo", 1024, 720)
        .position_centered()
        .build()
        .unwrap();


    let mut canvas = window.into_canvas().build().unwrap();
 
    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let renderer = renderer::MyRenderer { };

    let mut starfield : star::StarField = star::StarField::new_star_field();
    //let mut bm : bitmap::BitMap = bitmap::BitMap::new(800, 600);

    'running: loop {

        let mut bm : bitmap::BitMap = bitmap::BitMap::new(1024, 720);


        // Scan events
        canvas.set_draw_color(Color::RGB(0, 0, 0));
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
        // The rest of the game loop goes here...

        //canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Move stars
        for i in 0..starfield.stars.len() {
            let star = starfield.stars[i];
            starfield.stars[i] = star::Star { x : star.x, y : star.y + 0.002, z: star.z }
        }
        //println!("{:?}", starfield.stars.len());

        renderer::MyRenderer::render(&starfield, &mut bm);

        let width = bm.width;
        let height = bm.height;

        for x in 0..width {
            for y in 0..height {
                let color = bm.get(x, y);
                canvas.set_draw_color(color);
                if(color.r == 0 && color.g == 0 && color.b == 0){

                } else {
                    let point = Point::new(x, y);
                    let res = canvas.draw_point(point);
                }
            }
        }
        //println!("{} {} {}", color.r,  color.g, color.b);
        //canvas.clear();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 16_000_000u32 / 60));
    }
}





