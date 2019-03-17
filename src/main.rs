extern crate nalgebra_glm as glm;

mod renderer;
mod bitmap;
mod vertex;

use time::{ PreciseTime };
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::renderer::Renderer;
use crate::bitmap::BitMap;
use crate::vertex::Vertex;

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

fn resolution(video_subsystem : &VideoSubsystem) -> (u32, u32) {

    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    println!("{:}, {:}, {:}" , display_mode.w, display_mode.h, display_mode.refresh_rate);

    //let res_x = display_mode.w as u32;
    //let res_y = display_mode.h as u32;

    let res_x = 800 as u32;
    let res_y = 600 as u32;

    return (res_x, res_y);
}


pub fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (res_x, res_y) = resolution(&video_subsystem);
 
    let window: Window = video_subsystem.window("Super starfield", res_x, res_y)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut bm = BitMap::new(res_x, res_y, &mut canvas);
    let mut renderer = Renderer::new(res_x, res_y);

    let mut previous_time = PreciseTime::now();


    let v1 = Vertex::new(100.0,  100.0);
    let v2 = Vertex::new(250.0,  200.0);
    let v3 = Vertex::new(50.0,   350.0);

    let v4 = Vertex::new(500.0,  100.0);
    let v5 = Vertex::new(750.0,  200.0);
    let v6 = Vertex::new(550.0,  350.0);

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

        renderer.rasterize_triangle(v3, v1, v2);
        renderer.render_scanbuffer(&mut bm);

        renderer.rasterize_triangle(v4, v5, v6);
        renderer.render_scanbuffer(&mut bm);


        bm.present();

        //::std::thread::sleep(Duration::new(0, 500_000));
    }
}





