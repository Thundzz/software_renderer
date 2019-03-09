mod star;
mod renderer;
mod bitmap;


use time::{ PreciseTime };
use std::time::{ Duration };
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

fn move_stars(starfield : &mut star::StarField, speed : f64, duration : f64) {
    let z_change = speed * duration;
    for i in 0..starfield.stars.len() {
        starfield.stars[i].z -= z_change;
        if starfield.stars[i].z <= 0.0 {
            starfield.stars[i] = star::Star::random(128.0);
        }
    }
}

fn should_stop(event_pump : &mut sdl2::EventPump) -> bool {
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

    // let res_x = 1920 as u32;
    // let res_y = 1080 as u32;

    let nb_stars = 3096;

    let speed = 0.02;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    println!("{:}, {:}, {:}" , display_mode.w, display_mode.h, display_mode.refresh_rate);
    let res_x = display_mode.w as u32;
    let res_y = display_mode.h as u32;

 
    let window: Window = video_subsystem.window("Super starfield", res_x, res_y)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut starfield : star::StarField = star::StarField::new_star_field(nb_stars, 128.0);
    let mut bm :  bitmap::BitMap = bitmap::BitMap::new(res_x, res_y, &mut canvas);

    let mut previous_time = PreciseTime::now();
    
    'running: loop {
        let current_time = PreciseTime::now();
        let delta =  previous_time.to(current_time);
        previous_time = current_time;

        let delta_nanos = delta.num_nanoseconds().unwrap();
        let delta_millis = delta_nanos as f64 / 1_000_000.0;

        println!("{:?} ms", delta_millis);


        let background_color = Color::RGB(0, 0, 0);
        bm.clear(background_color);

        if should_stop(&mut event_pump){
            break 'running;
        }

        move_stars(&mut starfield, speed, delta_millis);
        renderer::MyRenderer::render(&starfield, &mut bm, res_x, res_y);

        bm.present();

        ::std::thread::sleep(Duration::new(0, 6_000_000));
    }
}





