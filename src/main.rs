#![allow(dead_code)]
#![allow(unused_variables)]

extern crate minifb;

mod camera;
mod data;
mod input;
mod material;
mod mesh;
mod model;
mod mouse_diff;
mod render_utils;
mod sampler;
mod texture;
mod transform;
mod triangle;

use mesh::Mesh;
use minifb::MouseButton;
use minifb::MouseMode;
use model::Model;
use texture::Texture;
use transform::Transform;

use camera::Camera;

use glam::Vec3;
use minifb::{Key, Window, WindowOptions};
use std::rc::Rc;
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

    let mut window = Window::new("Hello Triangle", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // let mut gltf_obj = Model::from_filepath("resources/cube/Cube.gltf");
    let mut gltf_obj = Model::from_filepath("resources/helmet/Helmet.gltf");

    let texture = Rc::new(Texture::from_filepath("resources/textures/bojan.jpg"));
    let mut plane = Mesh::from_texture(&data::PLANE_DATA, &[0, 2, 1, 0, 3, 2], &texture);
    let mut triangle = Mesh::from_texture(&data::PLANE_DATA, &[0, 3, 2], &texture);
    let mut cube = Mesh::from_texture(&data::CUBE_VERTICES, &data::CUBE_INDICES, &texture);
    let mut rhombus = Mesh::from_texture(&data::RHOMBUS_VERTICES, &data::RHOMBUS_INDEX, &texture);
    let mut pyramid = Mesh::from_texture(&data::PYRAMID_VERTEX, &data::PYRAMID_INDEX, &texture);

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
        let cube_trans = Transform::from_rotation_quat(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            rotation,
            rotation,
            0.0,
        ));

        let plane_trans = Transform::from_translation(Vec3::new(4.0, 0.0, 0.0));
        let rhomb_trans = Transform::from_translation(Vec3::new(0.0, 3.0, 0.0));
        let pyramid_trans = Transform::from_translation(Vec3::new(2.0, 3.0, 2.0));
        let tri_trans = Transform::from_translation(Vec3::new(0.0, 0.0, 2.0));

        plane.replace_transform(plane_trans);
        gltf_obj.replace_transform(cube_trans);
        triangle.replace_transform(tri_trans);
        rhombus.replace_transform(rhomb_trans);
        pyramid.replace_transform(pyramid_trans);

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

        if window.get_mouse_down(MouseButton::Left) {
            plane.next_render_mode();
            cube.next_render_mode();
            triangle.next_render_mode();
            rhombus.next_render_mode();
            pyramid.next_render_mode();
            gltf_obj.next_render_mode();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Render 2 Triangles
        //cube.render(&mut buffer, &mut depth_buffer, &camera, &Transform::default());
        plane.render(
            &mut buffer,
            &mut depth_buffer,
            &camera,
            &Transform::default(),
        );
        triangle.render(
            &mut buffer,
            &mut depth_buffer,
            &camera,
            &Transform::default(),
        );
        rhombus.render(
            &mut buffer,
            &mut depth_buffer,
            &camera,
            &Transform::default(),
        );
        pyramid.render(
            &mut buffer,
            &mut depth_buffer,
            &camera,
            &Transform::default(),
        );
        gltf_obj.render(&mut buffer, &mut depth_buffer, &camera);

        //Input
        input::move_camera(&window, &mut camera, dt);
        mouse_diff::change_fov(&window, &mut camera, dt);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        println!("Time elapsed: {:?}", now.elapsed());
    }
}
