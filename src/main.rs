extern crate nalgebra_glm as glm;

mod renderer;
mod bitmap;
mod vertex;

use std::time::{ Duration };
use time::{ PreciseTime };
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use glm::Mat4;

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

fn deg_to_rad(deg : f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
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

    let v1 = Vertex::new(-1.0, -1.0, 0.0);
    let v2 = Vertex::new(0.0 ,  1.0, 0.0);
    let v3 = Vertex::new(1.0 ,  -1.0, 0.0);

    let fov_deg = 70.0 as f32;
    let fov_rad = std::f32::consts::PI * fov_deg / 180.0;
    let projection : glm::Mat4 = glm::perspective_rh_no(4.0 / 3.0, fov_rad, 0.001, 100.0);
    let mut angle_deg = 0 as f32;

    'running: loop {
        let current_time = PreciseTime::now();
        let delta =  previous_time.to(current_time);
        previous_time = current_time;

        
        let delta_nanos = delta.num_nanoseconds().unwrap();
        let delta_millis = delta_nanos as f64 / 1_000_000.0;

        let angle = deg_to_rad(angle_deg);
        angle_deg = angle_deg + 1.0 ;

        //println!("Angle : {:?} took {:?} ms to render.",  angle_deg, delta_millis);

        let background_color = Color::RGB(0, 0, 0);
        bm.clear(background_color);

        if should_stop(&mut event_pump){
            break 'running;
        }
        
        let id = Mat4::identity();


        let translate = glm::translate(&id, &glm::vec3(0.0, 0.0, -3.0));
        let rotation = glm::rotate(&id, angle,  &glm::vec3(0.0, 1.0, 0.0));

        let transform = projection * translate * rotation;

        let v1t = v1.transform(transform);
        let v2t = v2.transform(transform);
        let v3t = v3.transform(transform);

        //println!("{:.5} {:.5} {:.5}", v1t.x(), v1t.y(), v1t.z());
        println!("{:.5} {:.5} {:.5}", v2t.x(), v2t.y(), v2t.z());
        //println!("{:.5} {:.5} {:.5}", v3t.x(), v3t.y(), v3t.z());
        

        renderer.render_triangle(
            &mut bm,
            v1.transform(transform),
            v2.transform(transform),
            v3.transform(transform)
        );

        bm.present();

        let sleep_time : i32 = 16_000_000 - (delta_nanos as i32);
        // println!("{:?}", sleep_time);
        if sleep_time > 0 {
            ::std::thread::sleep(Duration::new(0,  sleep_time as u32));
        }
    }
}





