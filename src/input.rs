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

pub fn enable_mouse(window: &Window, enabled: &mut bool) {
    if window.is_key_down(Key::M) {
        *enabled = !*enabled;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
