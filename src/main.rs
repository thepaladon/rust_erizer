extern crate minifb;

use std::time::Instant;
use minifb::{Key, Window, WindowOptions};
use glam::Vec2;
use glam::Vec3;

const WIDTH: usize = 1024;
const HEIGHT: usize = 800;

#[derive(Copy, Clone)]
struct Vertex {
    positions : Vec2,
    uv : Vec2,
}


struct Triangle {
    vertices : [Vertex; 3],
    color : Vec3
}

impl Triangle { 
    fn new(vertices: [Vertex; 3], color : Vec3) -> Self {
        Self { vertices, color }
    }

    fn render(&self, p : Vec2) -> Vec3 {
        
        let x = self.vertices[0].positions;
        let y = self.vertices[1].positions;
        let z = self.vertices[2].positions;
        
        let mut fc = Vec3::new( 0.0, 0.0, 0.0 );
    
        // clock wise check
        let area0 = edge_fun(p, x, y);
        let area1 = edge_fun(p, y, z);
        let area2 = edge_fun(p, z, x);

        if area0 < 0.0 && area1 < 0.0 && area2 < 0.0 { 
            fc = self.color;
        }

        fc
    }

    fn render_bary(&self, p : Vec2) -> Vec3 {
        let x = self.vertices[0].positions;
        let y = self.vertices[1].positions;
        let z = self.vertices[2].positions;
        
        let mut fc = Vec3::new( 0.0, 0.0, 0.0 );
    
        // clock wise check
        let area0 = edge_fun(p, x, y);
        let area1 = edge_fun(p, y, z);
        let area2 = edge_fun(p, z, x);

        if area0 < 0.0 && area1 < 0.0 && area2 < 0.0 { 
            fc = self.color;
            fc = bary_coord([x, y, z], p);
            fc *= Vec3::new(255.0, 255.0, 255.0);
        }

        fc
    }

    //fn render_uv(&self, p : Vec2) -> Vec3 {
    //    let color = render_triangle(self.color, self.vertices[0].positions, self.vertices[1].positions, self.vertices[2].positions, p);
    //    color
    //}

}

pub fn edge_fun(p : Vec2, v0 : Vec2, v1 : Vec2) -> f32 {
    let v0_p = p - v0;
    let v0_v1 = v1 - v0;

    v0_p.x * v0_v1.y - v0_p.y * v0_v1.x
}

//Barycentric coordinates
pub fn bary_coord(vertices : [Vec2; 3], p : Vec2) -> Vec3 {

    let area0 = edge_fun(p, vertices[1], vertices[2] ) / edge_fun(vertices[2], vertices[0], vertices[1] );
    let area1 = edge_fun(p, vertices[2], vertices[0] ) / edge_fun(vertices[2], vertices[0], vertices[1] );
    let area2 = 1.0 - area0 - area1;

    let bary = Vec3::new(area0, area1, area2);
    bary
}

fn to_argb8(a : u8, r : u8, g : u8, b : u8) -> u32 {

    let mut argb : u32 = a as u32; 
    argb = (argb << 8) + r as u32; 
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb

}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let v0 = Vertex { positions : Vec2::new(600.0, 100.0),  uv : Vec2::new( 0.0 , 1.0) } ;
    let v1 = Vertex { positions : Vec2::new(400.0, 300.0),  uv : Vec2::new( 0.0 , 1.0) } ;
    let v2 = Vertex { positions : Vec2::new(200.0, 100.0 ), uv : Vec2::new( 0.0 , 1.0) } ;
    let v3 = Vertex { positions : Vec2::new(600.0, 300.0),  uv : Vec2::new( 0.0 , 1.0) } ;

    let mut window = Window::new(
        "Hello Triangle",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let _red = Vec3::new(255.0, 0.0, 0.0);
    let _green = Vec3::new(255.0, 0.0, 0.0);
    let _blue = Vec3::new(0.0, 0.0, 255.0);
    let _white = Vec3::new(255.0, 255.0, 255.0);
    let _gray = Vec3::new(128.0, 128.0, 128.0);
    let _black = Vec3::new(0.0, 0.0, 0.0);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        // Clear Screen (doesn't work currently because of the other iteration)
        //let clear_color = to_argb8(255, white.x as u8, white.y as u8, white.z as u8);
        //buffer.fill(0xffffffff);

        for i in 0..buffer.len() {
            let x = i as f32 % WIDTH as f32                   + 0.5;
            let y = f32::floor(i as f32 / WIDTH as f32) + 0.5;
            
            let p = Vec2::new(x, y);

            let mut fc = Vec3::new(0.0, 0.0, 0.0); //final color 
            
            let tri0 = Triangle::new([v0, v1, v2], _white); 
            let tri1 = Triangle::new([v0, v3, v1], _gray); 
            
            fc += tri0.render_bary(p);
            fc += tri1.render_bary(p);

            
            buffer[i] = to_argb8(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        
        println!("Time elapsed: {:?}", now.elapsed());
    }
}