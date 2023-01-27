extern crate minifb;

use std::ops::Mul;
use std::time::Instant;
use image::DynamicImage;
use minifb::{Key, Window, WindowOptions};
use glam::Vec2;
use glam::Vec3;
use glam::Vec3Swizzles;
use image::{open};

const WIDTH: usize = 1024;
const HEIGHT: usize = 800;

#[derive(Copy, Clone)]
struct Vertex {
    positions : Vec3,
    uv : Vec2,
}

struct Triangle {
    vertices : [Vertex; 3],
    color : Vec3,

    aabb : Option<[Vec2; 2]>, 
}

impl Triangle { 

    fn new(vertices: [Vertex; 3], color : Vec3) -> Self {
        let mut tri = Self { vertices, color, aabb : None};
        tri.calc_aabb();
        tri
    }

    fn render_to_buffer(&self, buffer : &mut Vec<u32>) {

        match self.aabb {
            Some(aabb) => {

                for x in (aabb[0].x.floor() as usize)..(aabb[1].x.floor() as usize){
                    for y in (aabb[0].y.floor() as usize)..(aabb[1].y.floor() as usize){
                    
                        let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                        let idx : usize =  x + y * WIDTH as usize;

                        let src = buffer[idx];
                        let src = u32_to_argb8(src);

                        let mut fc = Vec3::new(src[1] as f32,  src[2] as f32, src[3] as f32); //final color 
                        
                        fc += self.render(p);
                        
                        buffer[idx] = argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
                }

            }},
            None => todo!(),
        }        
    }

    fn calc_aabb(&mut self) {
        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;

        let tmin : Vec2 =  Vec2::new( v0_p.x.min(v1_p.x).min(v2_p.x), v0_p.y.min(v1_p.y).min(v2_p.y)  );
        let tmax : Vec2 = Vec2::new(v0_p.x.max(v1_p.x).max(v2_p.x), v0_p.y.max(v1_p.y).max(v2_p.y));

        let taabb = [tmin, tmax];

        self.aabb = Some(taabb);
    }

    fn render(&self, p : Vec2) -> Vec3 {

        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;
        
        let mut fc = Vec3::new( 0.0, 0.0, 0.0 );
    
        // clock wise check
        let area0 = edge_fun(p, v0_p.xy(), v1_p.xy());
        let area1 = edge_fun(p, v1_p.xy(), v2_p.xy());
        let area2 = edge_fun(p, v2_p.xy(), v0_p.xy());

        if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 { 
            fc += self.color;
        }

        fc
    }

    fn render_bary(&self, p : Vec2) -> Vec3 {
        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;
        
        let mut fc = Vec3::new( 0.0, 0.0, 0.0 );
    
        // clock wise check
        let area0 = edge_fun(p, v0_p.xy(), v1_p.xy());
        let area1 = edge_fun(p, v1_p.xy(), v2_p.xy());
        let area2 = edge_fun(p, v2_p.xy(), v0_p.xy());

        if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 { 
            fc += bary_coord([v0_p, v1_p, v2_p], p);
            fc *= Vec3::new(255.0, 255.0, 255.0);
        }

        fc
    }

    fn render_uv(&self, p : Vec2) -> Vec3 {
        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;
        
        let mut fc = Vec3::new( 0.0, 0.0, 0.0 );
    
        // clock wise check
        let area0 = edge_fun(p, v0_p.xy(), v1_p.xy());
        let area1 = edge_fun(p, v1_p.xy(), v2_p.xy());
        let area2 = edge_fun(p, v2_p.xy(), v0_p.xy());

        if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 { 
            let bary = bary_coord([v0_p, v1_p, v2_p], p);
            
            let v0_uv = self.vertices[0].uv.mul(bary.x); 
            let v1_uv = self.vertices[1].uv.mul(bary.y);
            let v2_uv = self.vertices[2].uv.mul(bary.z);  
            
            //Uv coords pog
            let uv =  (v0_uv + v1_uv + v2_uv) * Vec2::new(255.0, 255.0);

            fc += Vec3::new(uv.x, uv.y, 0.0);
        }

        fc
    }

    fn render_tex(&self, p : Vec2, tex : &DynamicImage) -> Vec3 {
        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;
        
        let mut fc = Vec3::new( 0.0, 0.0, 0.0 );
    
        // clock wise check
        let area0 = edge_fun(p, v0_p.xy(), v1_p.xy());
        let area1 = edge_fun(p, v1_p.xy(), v2_p.xy());
        let area2 = edge_fun(p, v2_p.xy(), v0_p.xy());

        let image_buffer = tex.as_rgb8().expect("Shit's not there >:( ");

        if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 { 
            let bary = bary_coord([v0_p, v1_p, v2_p], p);
            
            let v0_uv = self.vertices[0].uv.mul(bary.x); 
            let v1_uv = self.vertices[1].uv.mul(bary.y);
            let v2_uv = self.vertices[2].uv.mul(bary.z);  
           
            //Uv coords pog
            let uv =  v0_uv + v1_uv + v2_uv ;

            let img_width = image_buffer.width() as f32 * uv.x;
            let img_height = image_buffer.height() as f32 * uv.y;
            
            let color = image_buffer.get_pixel(img_width as u32 , img_height as u32);
            
            fc += Vec3::new(color[0] as f32, color[1] as f32 , color[2] as f32);
        }

        fc
    }

}

pub fn edge_fun(p : Vec2, v0 : Vec2, v1 : Vec2) -> f32 {
    let v0_p = p - v0;
    let v0_v1 = v1 - v0;

    v0_p.x * v0_v1.y - v0_p.y * v0_v1.x
}

//Barycentric coordinates
pub fn bary_coord(vertices : [Vec3; 3], p : Vec2) -> Vec3 {

    let area0 = edge_fun(p, vertices[1].xy(), vertices[2].xy() ) / edge_fun(vertices[2].xy(), vertices[0].xy(), vertices[1].xy() );
    let area1 = edge_fun(p, vertices[2].xy(), vertices[0].xy() ) / edge_fun(vertices[2].xy(), vertices[0].xy(), vertices[1].xy() );
    let area2 = 1.0 - area0 - area1;

    let bary = Vec3::new(area0, area1, area2);
    bary
}

fn argb8_to_u32(a : u8, r : u8, g : u8, b : u8) -> u32 {

    let mut argb : u32 = a as u32; 
    argb = (argb << 8) + r as u32; 
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb

}

fn u32_to_argb8(pix : u32) -> [u8; 4] {

    let mut argb : [u8; 4] = [ 0, 0, 0, 0];

    argb[3] = (pix & 0xff) as u8;
    argb[2] = ((pix >> 8) & 0xff) as u8;
    argb[1] = ((pix >> 16) & 0xff) as u8;
    argb[0] = ((pix >> 24) & 0xff) as u8;
    
    argb
}


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let v0 = Vertex { positions : Vec3::new(200.0, 200.0, 0.0),  uv : Vec2::new( 0.0 , 0.0) } ;
    let v1 = Vertex { positions : Vec3::new(200.0, 600.0, 0.0),  uv : Vec2::new( 0.0 , 1.0) } ;
    let v2 = Vertex { positions : Vec3::new(600.0, 200.0, 0.0 ), uv : Vec2::new( 1.0 , 0.0) } ;
    let v3 = Vertex { positions : Vec3::new(600.0, 600.0, 0.0),  uv : Vec2::new( 1.0 , 1.0) } ;

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
    let _green = Vec3::new(0.0, 255.0, 0.0);
    let _blue = Vec3::new(0.0, 0.0, 255.0);
    let _white = Vec3::new(255.0, 255.0, 255.0);
    let _gray = Vec3::new(128.0, 128.0, 128.0);
    let _black = Vec3::new(0.0, 0.0, 0.0);

    let tex = open("resources/Harvey2.jpg").expect("Texture Error: ");

    let tri0 = Triangle::new([v0, v2, v1], _white); 
    let tri1 = Triangle::new([v2, v3, v1], _gray ); 

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        // Clear Screen (doesn't work currently because of the other iteration)
        let clear_color = argb8_to_u32(255, _black.x as u8, _black.y as u8, _black.z as u8);
        buffer.fill(clear_color);

        tri0.render_to_buffer(&mut buffer);
        tri1.render_to_buffer(&mut buffer);

        //for i in 0..buffer.len() {
        //    let x = i as f32 % WIDTH as f32                   + 0.5;
        //    let y = f32::floor(i as f32 / WIDTH as f32)  + 0.5;
        //    
        //    let p = Vec2::new(x, y);
//
        //    let mut fc = Vec3::new(0.0, 0.0, 0.0); //final color 
        //    
        //    //the overdrawing will be fixed once I implement accel structure
        //    fc += tri0.render_tex(p, &tex);
        //    fc += tri1.render_tex(p, &tex);
//
        //    buffer[i] = to_argb8(255, fc.x as u8, fc.y as u8, fc.z as u8);
        //}

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        
        println!("Time elapsed: {:?}", now.elapsed());
    }
}