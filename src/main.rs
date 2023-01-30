#![allow(dead_code)]
#![allow(unused_variables)]

extern crate minifb;

mod camera;
mod data;
mod render_utils;
mod transform;
mod triangle;

use data::Vertex;
use transform::Transform;
use triangle::Triangle;

use camera::Camera;

use glam::Vec2;
use glam::Vec3;
use image::open;
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

const _RED: Vec3 = Vec3::new(255.0, 0.0, 0.0);
const _GREEN: Vec3 = Vec3::new(0.0, 255.0, 0.0);
const _BLUE: Vec3 = Vec3::new(0.0, 0.0, 255.0);
const _WHITE: Vec3 = Vec3::new(255.0, 255.0, 255.0);
const _GRAY: Vec3 = Vec3::new(128.0, 128.0, 128.0);
const _BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);

const WIDTH: usize = 1024;
const HEIGHT: usize = 800;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let v0 = Vertex {
        positions: Vec3::new(200.0, 200.0, 0.0),
        uv: Vec2::new(0.0, 0.0),
    };
    let v1 = Vertex {
        positions: Vec3::new(200.0, 600.0, 0.0),
        uv: Vec2::new(0.0, 1.0),
    };
    let v2 = Vertex {
        positions: Vec3::new(600.0, 200.0, 0.0),
        uv: Vec2::new(1.0, 0.0),
    };
    let v3 = Vertex {
        positions: Vec3::new(600.0, 600.0, 0.0),
        uv: Vec2::new(1.0, 1.0),
    };
    let v4 = Vertex {
        positions: Vec3::new(800.0, 750.0, 0.0),
        uv: Vec2::new(1.0, 1.0),
    };
    let v5 = Vertex {
        positions: Vec3::new(200.0, 200.0, 0.0),
        uv: Vec2::new(1.0, 1.0),
    };
    let v6 = Vertex {
        positions: Vec3::new(400.0, 700.0, 0.0),
        uv: Vec2::new(1.0, 1.0),
    };

    let mut window = Window::new("Hello Triangle", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let _tex = open("resources/Harvey2.jpg").expect("Texture Error: ");

    let tri0 = Triangle::new_t([v0, v2, v1], _RED, &_tex);
    let tri1 = Triangle::new_t([v2, v3, v1], _GREEN, &_tex);
    let tri2 = Triangle::new_c([v4, v6, v5], _BLUE);

    let camera = Camera::default();
    let transform = Transform::default();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        let clear_color =
            render_utils::argb8_to_u32(255, _BLACK.x as u8, _BLACK.y as u8, _BLACK.z as u8);

        buffer.fill(clear_color);

        tri0.render_to_buffer(&mut buffer);
        tri1.render_to_buffer(&mut buffer);
        tri2.render_to_buffer(&mut buffer);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        println!("Time elapsed: {:?}", now.elapsed());
    }
}
