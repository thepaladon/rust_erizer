#![allow(dead_code)]
#![allow(unused_variables)]

extern crate minifb;

mod camera;
mod data;
mod input;
mod mouse_diff;
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
use std::time::Instant;

use crate::input::enable_mouse;

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
    let mut depth_buffer: Vec<f32> = vec![f32::INFINITY; WIDTH * HEIGHT];

    let v0 = Vertex {
        position: glam::vec4(-1.0, -1.0, 1.0, 1.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let v1 = Vertex {
        position: glam::vec4(-1.0, 1.0, 1.0, 1.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let v2 = Vertex {
        position: glam::vec4(1.0, 1.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let v3 = Vertex {
        position: glam::vec4(1.0, -1.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    };

    let v4 = Vertex {
        position: glam::vec4(800.0, 750.0, 0.0, 1.0),
        uv: Vec2::new(1.0, 1.0),
    };
    let v5 = Vertex {
        position: glam::vec4(200.0, 200.0, 0.0, 1.0),
        uv: Vec2::new(1.0, 1.0),
    };
    let v6 = Vertex {
        position: glam::vec4(400.0, 700.0, 0.0, 1.0),
        uv: Vec2::new(1.0, 1.0),
    };

    let mut window = Window::new("Hello Triangle", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let _tex = open("resources/bojan.jpg").expect("Texture Error: ");

    let mut tri0 = Triangle::new_t([v0, v2, v1], _WHITE, &_tex);
    let mut tri1 = Triangle::new_t([v0, v3, v2], _WHITE, &_tex);
    let mut tri2 = Triangle::new_t([v0, v3, v2], _BLUE, &_tex);

    // Camera Init
    let mut mouse_camera_controls = true;
    let mut camera = Camera::default();
    camera.set_position(Vec3::new(0.0, 0.0, 8.0));

    let mut rotation: f32 = 0.0;
    let mut dmouse = window.get_mouse_pos(MouseMode::Pass).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));
    window.set_cursor_visibility(false);

    let mut ignore_mouse = true;

    let mut prev_dt = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //Delta Time
        let now = Instant::now();
        let dt = now.duration_since(prev_dt).as_secs_f32();
        prev_dt = now;

        //Clear buffers
        let clear_color =
            render_utils::argb8_to_u32(255, _BLACK.x as u8, _BLACK.y as u8, _BLACK.z as u8);
        buffer.fill(clear_color);
        depth_buffer.fill(f32::INFINITY);

        //Rotate object on screen
        rotation += 0.01;
        let transform = Transform::from_rotation_quat(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            rotation,
            rotation,
            0.0,
        ));

        let transform2 = Transform::from_translation(Vec3::new(0.0, 0.0, 2.0));

        tri0.replace_transform(transform);
        tri1.replace_transform(transform);
        tri2.replace_transform(transform2);

        // Mouse diff for camera rotaiton
        enable_mouse(&window, &mut mouse_camera_controls);

        if mouse_camera_controls {
            window.set_cursor_visibility(false);
            let mut mouse_diff =
                mouse_diff::mouse_diff_fn(&window, &mut ignore_mouse, &mut dmouse, dt);
            mouse_diff *= camera.sensitivity;
            camera
                .transform
                .add_rotation(Vec3::new(mouse_diff.y, mouse_diff.x, 0.0));
        } else {
            window.set_cursor_visibility(true);
        }

        // Render 2 Triangles
        tri0.render_triangle(&mut buffer, &mut depth_buffer, &camera);
        tri1.render_triangle(&mut buffer, &mut depth_buffer, &camera);
        tri2.render_triangle(&mut buffer, &mut depth_buffer, &camera);

        //Input
        input::move_camera(&window, &mut camera, dt);
        mouse_diff::change_fov(&window, &mut camera, dt);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        println!("Time elapsed: {:?}", now.elapsed());
    }
}
