// Camera
// frustum near / far
// fiv
// aspect ratio
// transform
// speed
// DEFAULT

//Functs
// func projection
// func view (look_at_rh)

use glam::{Vec3, Mat4};

use crate::transform::Transform;
use crate::HEIGHT;
use crate::WIDTH;

pub struct Camera {
    pub near_plane: f32,
    pub far_plane: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub move_speed: f32,
    pub sensitivity: f32,
    pub transform: Transform,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near_plane: 0.01,
            far_plane: 200.0,
            fov: f32::to_radians(60.0),
            aspect_ratio: WIDTH as f32 / HEIGHT as f32,
            move_speed: 10.0,
            sensitivity: 10.0,
            transform: Transform::IDENTITY,
        }
    }
}

impl Camera {
    pub fn perspective(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near_plane, self.far_plane)
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(self.transform.translation, Vec3::ZERO, Vec3::Y)
    }
}
