use std::ops::{Add, Mul, Sub};

use glam::{Vec2, Vec3, Vec4};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vec4,
    pub normal: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

pub static PLANE_DATA: [Vertex; 4] = [
    Vertex {
        position: glam::vec4(-1.0, -1.0, 1.0, 1.0),
        normal: glam::vec3(-1.0, -1.0, 1.0),
        color: glam::vec3(-1.0, -1.0, 1.0),
        uv: glam::vec2(0.0, 1.0),
    },
    Vertex {
        position: glam::vec4(-1.0, 1.0, 1.0, 1.0),
        normal: glam::vec3(-1.0, -1.0, 1.0),
        color: glam::vec3(-1.0, -1.0, 1.0),
        uv: glam::vec2(0.0, 0.0),
    },
    Vertex {
        position: glam::vec4(1.0, 1.0, 1.0, 1.0),
        normal: glam::vec3(-1.0, -1.0, 1.0),
        color: glam::vec3(-1.0, -1.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    },
    Vertex {
        position: glam::vec4(1.0, -1.0, 1.0, 1.0),
        normal: glam::vec3(-1.0, -1.0, 1.0),
        color: glam::vec3(-1.0, -1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    },
];

pub static CUBE_VERTICES: [Vertex; 8] = [
    Vertex {
        position: Vec4::new(-1.0, -1.0, 1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        color: Vec3::new(1.0, 0.0, 0.0),
        uv: Vec2::new(0.0, 0.0),
    },
    Vertex {
        position: Vec4::new(1.0, -1.0, 1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        color: Vec3::new(0.0, 1.0, 0.0),
        uv: Vec2::new(1.0, 0.0),
    },
    Vertex {
        position: Vec4::new(-1.0, 1.0, 1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        color: Vec3::new(0.0, 0.0, 1.0),
        uv: Vec2::new(0.0, 1.0),
    },
    Vertex {
        position: Vec4::new(1.0, 1.0, 1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        color: Vec3::new(1.0, 1.0, 0.0),
        uv: Vec2::new(1.0, 1.0),
    },
    Vertex {
        position: Vec4::new(-1.0, -1.0, -1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        color: Vec3::new(0.0, 1.0, 1.0),
        uv: Vec2::new(1.0, 0.0),
    },
    Vertex {
        position: Vec4::new(1.0, -1.0, -1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        color: Vec3::new(1.0, 0.0, 1.0),
        uv: Vec2::new(0.0, 0.0),
    },
    Vertex {
        position: Vec4::new(-1.0, 1.0, -1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        color: Vec3::new(1.0, 0.5, 0.0),
        uv: Vec2::new(1.0, 1.0),
    },
    Vertex {
        position: Vec4::new(1.0, 1.0, -1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        color: Vec3::new(0.5, 0.5, 0.5),
        uv: Vec2::new(0.0, 1.0),
    },
];

pub static CUBE_INDICES: [u32; 36] = [
    0, 1, 2, // Front
    1, 2, 3, 4, 5, 6, // Back
    5, 6, 7, 0, 2, 4, // Left
    2, 4, 6, 1, 3, 5, // Right
    3, 5, 7, 0, 1, 4, // Top
    1, 4, 5, 2, 3, 6, // Bottom
    3, 6, 7,
];

pub static RHOMBUS_VERTICES: [Vertex; 6] = [
    Vertex {
        position: Vec4::new(0.0, 0.0, 1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        color: Vec3::new(0.5, 0.7, 0.2),
        uv: Vec2::new(0.5, 0.5),
    },
    Vertex {
        position: Vec4::new(1.0, 0.0, 0.0, 1.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        color: Vec3::new(0.2, 0.3, 0.7),
        uv: Vec2::new(1.0, 0.0),
    },
    Vertex {
        position: Vec4::new(0.0, 1.0, 0.0, 1.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        color: Vec3::new(0.3, 0.1, 0.8),
        uv: Vec2::new(0.5, 1.0),
    },
    Vertex {
        position: Vec4::new(-1.0, 0.0, 0.0, 1.0),
        normal: Vec3::new(-1.0, 0.0, 0.0),
        color: Vec3::new(0.1, 0.9, 0.3),
        uv: Vec2::new(0.0, 0.0),
    },
    Vertex {
        position: Vec4::new(0.0, -1.0, 0.0, 1.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
        color: Vec3::new(0.9, 0.2, 0.4),
        uv: Vec2::new(0.5, 0.0),
    },
    Vertex {
        position: Vec4::new(0.0, 0.0, -1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        color: Vec3::new(0.6, 0.5, 0.1),
        uv: Vec2::new(1.0, 0.5),
    },
];

pub static RHOMBUS_INDEX: [u32; 24] = [
    0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 1, 5, 2, 1, 5, 3, 2, 5, 4, 3, 5, 1, 4,
];

pub static PYRAMID_VERTEX: [Vertex; 5] = [
    Vertex {
        position: Vec4::new(0.0, 1.0, 0.0, 1.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        color: Vec3::new(1.0, 0.5, 0.0),
        uv: Vec2::new(0.5, 0.5),
    },
    Vertex {
        position: Vec4::new(-0.5, 0.0, -0.5, 1.0),
        normal: Vec3::new(-1.0, 0.0, -1.0),
        color: Vec3::new(0.0, 0.5, 1.0),
        uv: Vec2::new(0.2, 0.8),
    },
    Vertex {
        position: Vec4::new(0.5, 0.0, -0.5, 1.0),
        normal: Vec3::new(1.0, 0.0, -1.0),
        color: Vec3::new(0.0, 1.0, 0.5),
        uv: Vec2::new(0.8, 0.8),
    },
    Vertex {
        position: Vec4::new(0.5, 0.0, 0.5, 1.0),
        normal: Vec3::new(1.0, 0.0, 1.0),
        color: Vec3::new(0.5, 0.0, 0.5),
        uv: Vec2::new(0.8, 0.2),
    },
    Vertex {
        position: Vec4::new(-0.5, 0.0, 0.5, 1.0),
        normal: Vec3::new(-1.0, 0.0, 1.0),
        color: Vec3::new(1.0, 0.5, 0.5),
        uv: Vec2::new(0.2, 0.2),
    },
];

pub static PYRAMID_INDEX: [u32; 18] = [0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 1, 1, 4, 3, 1, 3, 2];

impl Vertex {
    pub fn new(position: Vec4, normal: Vec3, color: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            uv,
            normal,
            color,
        }
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let normal = self.normal + rhs.normal;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self {
            position,
            uv,
            normal,
            color,
        }
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let position = self.position - rhs.position;
        let normal = self.normal - rhs.normal;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self {
            position,
            uv,
            normal,
            color,
        }
    }
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let normal = self.normal * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self {
            position,
            uv,
            normal,
            color,
        }
    }
}
