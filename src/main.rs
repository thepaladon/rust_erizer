extern crate minifb;

use std::time::Instant;
use minifb::{Key, Window, WindowOptions};
use glam::Vec2;
use glam::Vec3;

const WIDTH: usize = 1024;
const HEIGHT: usize = 800;

pub fn edge_fun(p : Vec2, v0 : Vec2, v1 : Vec2) -> f32 {
    let v0_p = p - v0;
    let v0_v1 = v1 - v0;

    v0_p.x * v0_v1.y - v0_p.y * v0_v1.x
}

fn to_argb8(a : u8, r : u8, g : u8, b : u8) -> u32 {

    let mut argb : u32 = a as u32; 
    argb = (argb << 8) + r as u32; 
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb

}

fn render_triangle(color : Vec3, x : Vec2, y : Vec2, z : Vec2, p : Vec2) -> Vec3 {

        let mut fc = Vec3::new( 0.0, 0.0, 0.0 );
        
        // clock wise check
        let area0 = edge_fun(p, x, y);
        let area1 = edge_fun(p, y, z);
        let area2 = edge_fun(p, z, x);

        if area0 < 0.0 && area1 < 0.0 && area2 < 0.0 { 
            fc = color;
        }

        fc
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let v0 = Vec2::new(600.0, 100.0 );
    let v1 = Vec2::new(400.0, 300.0 );
    let v2 = Vec2::new(200.0, 100.0 );
    let v3 = Vec2::new(600.0, 300.0 );

    let mut window = Window::new(
        "Hello Triangle",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));


    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        //clear function basically
        for i in buffer.iter_mut() {
            *i = to_argb8(0, 0, 0, 0); // write something more funny here!
        }

        for i in 0..buffer.len() {
            let x = i as f32 % WIDTH as f32;
            let y = i as f32 / WIDTH as f32 ;

            let p = Vec2::new(x, y);

            let mut fc = Vec3::new(0.0, 0.0, 0.0); //final color 

            let  color0 = Vec3::new(0.0, 0.0, 255.0);
            let  color1 = Vec3::new(255.0, 0.0, 0.0);

            fc += render_triangle(color0, v0, v1, v2, p);
            fc += render_triangle(color1, v0, v3, v1, p);
          
            buffer[i] = to_argb8(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        
        println!("Time elapsed: {:?}", now.elapsed());
    }
}