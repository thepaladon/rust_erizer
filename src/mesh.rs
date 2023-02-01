use glam::Vec3;
use image::DynamicImage;

use crate::{camera::Camera, data::Vertex, transform::Transform, triangle::Triangle};

pub struct Mesh<'a> {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub color: Vec3,
    pub texture: Option<&'a DynamicImage>,
    pub transform: Transform,
    pub render_mode: i32,
}

impl<'a> Mesh<'a> {
    pub fn from_color(vertices: &[Vertex], indices: &[u32], color: Vec3) -> Self {
        Self {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
            color,
            texture: None,
            transform: Transform::IDENTITY,
            render_mode: 0,
        }
    }

    pub fn from_texture(vertices: &[Vertex], indices: &[u32], texture: &'a DynamicImage) -> Self {
        assert!(
            indices.len() % 3 == 0,
            "Indices size is wrong. {} % 3 == 0",
            indices.len()
        );

        Self {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
            color: Vec3::splat(255.0),
            texture: Some(texture),
            transform: Transform::IDENTITY,
            render_mode: 0,
        }
    }

    pub fn replace_transform(&mut self, trans: Transform) {
        self.transform = trans;
    }

    pub fn render_triangle(&self, buffer: &mut [u32], depth: &mut [f32], camera: &Camera) {
        let mvp = camera.perspective() * camera.view() * self.transform.local();

        for i in (0..self.indices.len()).step_by(3) {
            let clip0 = mvp * self.vertices[i].position;
            let clip1 = mvp * self.vertices[i + 1].position;
            let clip2 = mvp * self.vertices[i + 2].position;

            let mut copy0 = self.vertices[i];
            let mut copy1 = self.vertices[i + 1];
            let mut copy2 = self.vertices[i + 2];

            copy0.position = clip0;
            copy1.position = clip1;
            copy2.position = clip2;

            let triangle = Triangle::new([copy0, copy1, copy2]);

            triangle.render_triangle(
                buffer,
                depth,
                camera,
                self.texture,
                &self.color,
                self.render_mode,
            );
        }
    }
}
