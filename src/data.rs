use std::ops::{Add, Mul, Sub};

use glam::{Vec2, Vec4};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vec4,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec4, uv: Vec2) -> Self {
        Self { position, uv }
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let uv = self.uv + rhs.uv;
        Self { position, uv }
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let position = self.position - rhs.position;
        let uv = self.uv - rhs.uv;
        Self { position, uv }
    }
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let uv = self.uv * rhs;
        Self { position, uv }
    }
}
