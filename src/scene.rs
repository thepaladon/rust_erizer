use std::collections::HashMap;

use minifb::{MouseButton, Window};

use crate::{
    camera::Camera, mesh::VertexMesh, model::Model, sliced_buffer::SlicedBuffers,
    transform::Transform,
};

pub struct Scene {
    name: String,
    camera: Camera,
    render_models: HashMap<String, Model>,
}

impl Scene {
    pub fn new(name: String) -> Self {
        Self {
            name,
            camera: Camera::default(),
            render_models: HashMap::new(),
        }
    }

    pub fn add_mesh(&mut self, name: &str, mesh: VertexMesh) {
        self.render_models.insert(
            name.to_string(),
            Model::from_mesh(mesh, Transform::default()),
        );
    }

    pub fn add_gltf(&mut self, name: &str, filepath: &str) {
        self.render_models
            .insert(name.to_string(), Model::from_filepath(filepath));
    }

    pub fn get_model(&mut self, key: &str) -> Option<&mut Model> {
        self.render_models.get_mut(key)
    }

    pub fn change_render_mode(&mut self, window: &Window) {
        if window.get_mouse_down(MouseButton::Left) {
            for model in &mut self.render_models {
                model.1.next_render_mode();
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        if window.get_mouse_down(MouseButton::Right) {
            for model in &mut self.render_models {
                model.1.prev_render_mode();
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn render(&mut self, buffer: &mut SlicedBuffers, camera: &Camera) {
        for model in &self.render_models {
            model.1.render(buffer, camera);
        }
    }
}
