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

use glam::{Mat4, Vec3};

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
    pub fov_sensitivity: f32,
    pub transform: Transform,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near_plane: 0.01,
            far_plane: 100.0,
            fov: f32::to_radians(60.0),
            aspect_ratio: WIDTH as f32 / HEIGHT as f32,
            move_speed: 2.0,
            fov_sensitivity: 8.0,
            sensitivity: 0.05,
            transform: Transform::IDENTITY,
        }
    }
}

impl Camera {
    pub fn set_position(&mut self, translation: Vec3) {
        self.transform.translation = translation;
    }

    pub fn add_position(&mut self, translation: Vec3) {
        self.transform.translation += translation;
    }

    pub fn add_fov(&mut self, change: f32) {
        self.fov += f32::to_radians(change * self.fov_sensitivity);
        self.fov = f32::clamp(self.fov, f32::to_radians(20.0), f32::to_radians(160.0));
    }

    pub fn move_forward(&mut self, dir: f32) {
        self.transform.translation += self.transform.forward() * self.move_speed * dir;
    }

    pub fn move_side(&mut self, dir: f32) {
        self.transform.translation += self.transform.right() * self.move_speed * dir;
    }

    pub fn move_up(&mut self, dir: f32) {
        self.transform.translation += self.transform.up() * self.move_speed * dir;
    }

    pub fn perspective(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near_plane, self.far_plane)
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.transform.translation,
            self.transform.translation + self.transform.forward(),
            self.transform.up(),
        )
    }
}
