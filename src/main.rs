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
mod scene;
mod sliced_buffer;
mod tex_manager;
mod texture;
mod transform;
mod triangle;

use mesh::VertexMesh;
use minifb::MouseMode;
use minifb::ScaleMode;
use model::Model;
use scene::Scene;
use sliced_buffer::SlicedBuffers;
use transform::Transform;

use camera::Camera;

use glam::Vec3;
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

use crate::input::enable_mouse;
use crate::mouse_diff::set_mouse_pos;
use crate::tex_manager::*;

const _RED: Vec3 = Vec3::new(255.0, 0.0, 0.0);
const _GREEN: Vec3 = Vec3::new(0.0, 255.0, 0.0);
const _BLUE: Vec3 = Vec3::new(0.0, 0.0, 255.0);
const _WHITE: Vec3 = Vec3::new(255.0, 255.0, 255.0);
const _GRAY: Vec3 = Vec3::new(128.0, 128.0, 128.0);
const _BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);

const WIN_WIDTH: usize = 1920;
const WIN_HEIGHT: usize = 1080;
const BUFF_WIDTH: usize = 1920 / BUFF_SCALE;
const BUFF_HEIGHT: usize = 1080 / BUFF_SCALE;
const BUFF_SCALE: usize = 4;
const TILE_SIZE: i32 = 4;

// BUFF_SCALE Down Testing
// /1 - 1920 x 1080 - 8x8 tiles - ~300ms
// /2 - 960 x 540 - 8x8 tiles - ~150ms
// /4 - 480 x 270
// /8 - 240 x 135

fn main() {
    let mut buffer: Vec<u32> = vec![0; BUFF_WIDTH * BUFF_HEIGHT];
    let depth_buffer: Vec<f32> = vec![f32::INFINITY; BUFF_WIDTH * BUFF_HEIGHT];

    let mut sliced_buffers = SlicedBuffers::from_buffers(&buffer, &depth_buffer, TILE_SIZE);
    let win_ops = WindowOptions {
        resize: true,
        scale_mode: ScaleMode::AspectRatioStretch,
        ..Default::default()
    };
    let mut window = Window::new("Angle's Rust_erizer", WIN_WIDTH, WIN_HEIGHT, win_ops)
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let bojan_tex = {
        let mut manager = TEXTURE_MANAGER.write().unwrap();
        manager
            .load_from_filepath("resources/textures/bojan.jpg")
            .expect("Not found")
    };

    let mut scene1 = Scene::new("Hello Scene".to_string());

    let cube = VertexMesh::from_texture(&data::CUBE_VERTICES, &data::CUBE_INDICES, bojan_tex);

    //Triangle
    let mut triangle = VertexMesh::from_texture(&data::PLANE_DATA, &[0, 3, 2], bojan_tex);
    triangle.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 2.0));

    //Plane
    let mut plane = VertexMesh::from_texture(&data::PLANE_DATA, &[0, 2, 1, 0, 3, 2], bojan_tex);
    plane.transform = Transform::from_translation(Vec3::new(4.0, 0.0, 0.0));

    //Rhombus
    let mut rhombus =
        VertexMesh::from_texture(&data::RHOMBUS_VERTICES, &data::RHOMBUS_INDEX, bojan_tex);
    rhombus.transform = Transform::from_translation(Vec3::new(0.0, 3.0, 0.0));

    //Pyramid
    let mut pyramid =
        VertexMesh::from_texture(&data::PYRAMID_VERTEX, &data::PYRAMID_INDEX, bojan_tex);
    pyramid.transform = Transform::from_translation(Vec3::new(2.0, 3.0, 2.0));

    // Camera Init
    let mut mouse_camera_controls = true;
    let mut camera = Camera::default();

    //For Sponza
    camera.set_position(Vec3::new(7.0, 2.5, -0.1));
    camera.yaw = 1.6348684;

    //Sponza
    let mut gltf_obj = Model::from_filepath("resources/sponza/Sponza.gltf");
    gltf_obj.transform = Transform::from_scale(Vec3::new(0.008, 0.008, 0.008));

    //Cube
    // let mut gltf_obj = Model::from_filepath("resources/cube/Cube.gltf");

    // Helmet
    // let mut gltf_obj = Model::from_filepath("resources/helmet/Helmet.gltf");

    let mut rotation: f32 = 0.0;
    let mut dmouse = window.get_mouse_pos(MouseMode::Pass).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));
    window.set_cursor_visibility(false);

    let mut ignore_mouse = false;
    let mut first_frame = true;

    let mut prev_dt = Instant::now();

    scene1.add_mesh("Cube", cube);
    scene1.add_mesh("Triangle", triangle);
    scene1.add_mesh("Plane", plane);
    scene1.add_mesh("Rhombus", rhombus);
    scene1.add_mesh("Pyramid", pyramid);
    scene1.add_gltf("Sponza", "resources/sponza/Sponza.gltf");

    if let Some(model) = scene1.get_model("Sponza") {
        model.transform = Transform::from_scale(Vec3::new(0.008, 0.008, 0.008));
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //Delta Time
        let now = Instant::now();
        let dt = now.duration_since(prev_dt).as_secs_f32();
        prev_dt = now;

        //Clear buffers
        let clear_color = render_utils::vec3_to_u32(_RED);

        sliced_buffers.clear_color(clear_color);
        sliced_buffers.clear_depth(f32::INFINITY);
        sliced_buffers.clear_tiles();

        //Rotate object on screen
        rotation += 0.01;
        let cube_trans = Transform::from_rotation_quat(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            rotation,
            rotation,
            0.0,
        ));

        // Mouse diff for camera rotaiton
        if !first_frame {
            enable_mouse(&window, &mut mouse_camera_controls);

            if mouse_camera_controls {
                window.set_cursor_visibility(false);
                let mut mouse_diff =
                    mouse_diff::mouse_diff_fn(&window, &mut ignore_mouse, &mut dmouse, dt);
                mouse_diff *= camera.sensitivity;

                camera.mouse_rotation(mouse_diff.y, mouse_diff.x);
            } else {
                window.set_cursor_visibility(true);
            }
        } else {
            set_mouse_pos(
                window.get_position().0 as i32 + 5,
                window.get_position().1 as i32 + 5,
            )
            .unwrap();
        }

        scene1.change_render_mode(&window);

        scene1.render(&mut sliced_buffers, &camera);

        //All triangles should have their vertices in screen space here
        //sliced_buffers.aa_bb_comparison();
        //
        //sliced_buffers.render(&camera, &cube.render_mode);

        buffer = sliced_buffers.transfer_buffer();

        //Input
        input::move_camera(&window, &mut camera, dt);
        mouse_diff::change_fov(&window, &mut camera, dt);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, BUFF_WIDTH, BUFF_HEIGHT)
            .unwrap();

        first_frame = false;
        println!("Time elapsed: {:?}", now.elapsed());
    }
}
