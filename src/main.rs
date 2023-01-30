#![allow(dead_code)]
#![allow(unused_variables)]

extern crate minifb;

mod camera;
mod data;
mod render_utils;
mod transform;
mod triangle;

use data::Vertex;
use minifb::MouseMode;
use transform::Transform;
use triangle::Triangle;

use camera::Camera;

use glam::Vec2;
use glam::Vec3;
use image::open;
use minifb::{Key, Window, WindowOptions};
use std::f32::INFINITY;
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
    let depth_buffer: Vec<f32> = vec![INFINITY; WIDTH * HEIGHT];

    let v0 = Vertex {
        positions: glam::vec3(-1.0, -1.0, 1.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let v1 = Vertex {
        positions: glam::vec3(-1.0, 1.0, 1.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let v2 = Vertex {
        positions: glam::vec3(1.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let v3 = Vertex {
        positions: glam::vec3(1.0, -1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
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

    let mut tri0 = Triangle::new_t([v0, v2, v1], _WHITE, &_tex);
    let mut tri1 = Triangle::new_t([v0, v3, v2], _WHITE, &_tex);
    //let mut tri2 = Triangle::new_c([v6, v4, v5], _BLUE);

    let mut camera = Camera::default();
    camera.set_position(Vec3::new(0.0, 0.0, 8.0));

    let mut rotation: f32 = 1.0;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));

    let mut dmouse = window.get_mouse_pos(MouseMode::Pass).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        let clear_color =
            render_utils::argb8_to_u32(255, _BLACK.x as u8, _BLACK.y as u8, _BLACK.z as u8);

        rotation += 0.01;
        let transform = Transform::from_rotation_quat(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            0.0,
            0.0,
            rotation,
        ));

        buffer.fill(clear_color);

        let mouse = window.get_mouse_pos(MouseMode::Pass).unwrap();
        let diff = Vec2::new(dmouse.0 - mouse.0, dmouse.1 - mouse.1) * 0.01;
        dmouse = mouse;

        camera
            .transform
            .add_rotation(Vec3::new(diff.y, diff.x, 0.0));

        tri0.replace_transform(transform);
        tri1.replace_transform(transform);

        tri0.render_to_buffer(&mut buffer, &camera);
        tri1.render_to_buffer(&mut buffer, &camera);

        if window.is_key_down(Key::S) {
            camera.move_forward(1.0);
        }

        if window.is_key_down(Key::W) {
            camera.move_forward(-1.0);
        }

        if window.is_key_down(Key::A) {
            camera.move_side(1.0);
        }

        if window.is_key_down(Key::D) {
            camera.move_side(-1.0);
        }

        if window.is_key_down(Key::R) {
            camera.move_up(-1.0);
        }

        if window.is_key_down(Key::F) {
            camera.move_up(1.0);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        //println!("Time elapsed: {:?}", now.elapsed());
    }
}
