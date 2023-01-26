extern crate minifb;

use std::time::Instant;
use minifb::{Key, Window, WindowOptions};
use glam::Vec2;
use glam::Vec3;

const WIDTH: usize = 1024;
const HEIGHT: usize = 800;


// TODO 
// - Traversal Acceleration Structure
// - Clipping 
// - Anti-Aliasing
// - Textured Quad - Tomorrw 
// - Bresenham
// - Other polygons, circles

pub fn edge_fun(p : Vec2, v0 : Vec2, v1 : Vec2) -> f32 {
    let v0_p = p - v0;
    let v0_v1 = v1 - v0;

    v0_p.x * v0_v1.y - v0_p.y * v0_v1.x
}

//Courtesy of Kamen
//pub fn to_argb8<T: Into<u32> + Copy>(a: T, r: T, g: T, b: T) -> u32
//{
//    let mut argb = a.into();
//
//    argb = (argb << 8) + r.into();
//    argb = (argb << 8) + g.into();
//    argb = (argb << 8) + b.into();
//
//    argb
//}

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

    let red = Vec3::new(255.0, 0.0, 0.0);
    let green = Vec3::new(255.0, 0.0, 0.0);
    let blue = Vec3::new(0.0, 0.0, 255.0);
    let white = Vec3::new(255.0, 255.0, 255.0);
    let gray = Vec3::new(128.0, 128.0, 128.0);
    let black = Vec3::new(0.0, 0.0, 0.0);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        // Clear Screen (doesn't work currently because of the other iteration)
        //let clear_color = to_argb8(255, white.x as u8, white.y as u8, white.z as u8);
        //buffer.fill(0xffffffff);

        for i in 0..buffer.len() {
            let x = i as f32 % WIDTH as f32;
            let y = i as f32 / WIDTH as f32;

            let p = Vec2::new(x, y);

            let mut fc = Vec3::new(0.0, 0.0, 0.0); //final color 
            
            fc += render_triangle(white, v0, v1, v2, p);
            fc += render_triangle(gray, v0, v3, v1, p);
          
            buffer[i] = to_argb8(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        
        println!("Time elapsed: {:?}", now.elapsed());
    }
}