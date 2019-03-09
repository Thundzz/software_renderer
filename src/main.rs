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
        starfield.stars[i].z -= 0.99;
        if starfield.stars[i].z <= 0.0 {
            starfield.stars[i] = star::Star::random(128.0);
        }
    }
}

fn handle_input(event_pump : &mut sdl2::EventPump) -> bool {
   for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return true
            }
            _ => {}
        }
    } 
    return false;
}

pub fn main() {

    let res_x = 1920 as u32;
    let res_y = 1080 as u32;

    let nb_stars = 10000;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window: Window = video_subsystem.window("Super starfield", res_x, res_y)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut starfield : star::StarField = star::StarField::new_star_field(nb_stars, 128.0);
    let mut bm :  bitmap::BitMap = bitmap::BitMap::new(res_x, res_y, &mut canvas);

    'running: loop {

        let background_color = Color::RGB(0, 0, 0);

        // Scan events
        bm.clear(background_color);
        if handle_input(&mut event_pump){
            break 'running;
        }

        move_stars(&mut starfield);
        renderer::MyRenderer::render(&starfield, &mut bm, res_x, res_y);

        bm.present();

        ::std::thread::sleep(Duration::new(0, 16_000_000));
    }
}





