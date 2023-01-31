use minifb::{Key, Window};

use crate::camera::Camera;

pub fn move_camera(window: &Window, camera: &mut Camera, dt: f32) {
    if window.is_key_down(Key::W) {
        camera.move_forward(dt);
    }

    if window.is_key_down(Key::S) {
        camera.move_forward(-dt);
    }

    if window.is_key_down(Key::D) {
        camera.move_side(dt);
    }

    if window.is_key_down(Key::A) {
        camera.move_side(-dt);
    }

    if window.is_key_down(Key::R) {
        camera.move_up(dt);
    }

    if window.is_key_down(Key::F) {
        camera.move_up(-dt);
    }
}

pub fn change_fov(window: &Window, camera: &mut Camera, dt: f32) {
    let dir = window.get_scroll_wheel().unwrap_or((0.0, 0.0));
    camera.add_fov(-dir.1 * dt);
}
