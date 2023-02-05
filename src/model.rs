use std::sync::Arc;

use crate::{
    camera::Camera, mesh::Mesh, render_utils::rgba8_to_u32, sliced_buffer::SlicedBuffers,
    texture::Texture, transform::Transform,
};
use gltf::{self, Gltf};

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Arc<Texture>>,

    transform: Transform,
}

impl Model {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            textures: Vec::new(),
            transform: Transform::IDENTITY,
        }
    }

    pub fn from_filepath(filepath: &str) -> Self {
        let mut model = Self::new();
        let gltf = Gltf::open(filepath);

        let (document, buffers, images) = gltf::import(filepath).unwrap();
        assert_eq!(buffers.len(), document.buffers().count());
        assert_eq!(images.len(), document.images().count());

        for image in images {
            let mut data: Vec<u32> = Vec::new();
            match image.format {
                gltf::image::Format::R8 => {}
                gltf::image::Format::R8G8 => todo!(),
                gltf::image::Format::R8G8B8 => {
                    data = image
                        .pixels
                        .chunks(3)
                        .map(|rgb| rgba8_to_u32(rgb[0], rgb[1], rgb[2], 255))
                        .collect()
                }
                gltf::image::Format::R8G8B8A8 => {
                    data = image
                        .pixels
                        .chunks(4)
                        .map(|rgba| rgba8_to_u32(rgba[0], rgba[1], rgba[2], rgba[3]))
                        .collect()
                }
                gltf::image::Format::R16 => todo!(),
                gltf::image::Format::R16G16 => todo!(),
                gltf::image::Format::R16G16B16 => todo!(),
                gltf::image::Format::R16G16B16A16 => todo!(),
                gltf::image::Format::R32G32B32FLOAT => todo!(),
                gltf::image::Format::R32G32B32A32FLOAT => todo!(),
            }

            model.textures.push(Arc::new(Texture {
                width: image.width,
                height: image.height,
                data,
                ..Default::default()
            }))
        }

        for scene in document.scenes() {
            for node in scene.nodes() {
                if let Some(mesh) = node.mesh() {
                    for primitive in mesh.primitives() {
                        let mut my_mesh = Mesh::gltf_load_mesh(&primitive, &buffers);

                        if my_mesh.material.base_tex_idx != -1 {
                            my_mesh.texture = Some(
                                model.textures[my_mesh.material.base_tex_idx as usize].clone(),
                            );
                        }

                        model.meshes.push(my_mesh);
                    }
                }
            }
        }

        model
    }

    pub fn render(&self, slice_buff: &mut SlicedBuffers, camera: &Camera) {
        for mesh in &self.meshes {
            mesh.render(slice_buff, camera, &self.transform)
        }
    }

    pub fn next_render_mode(&mut self) {
        for mesh in &mut self.meshes {
            mesh.next_render_mode();
        }
    }

    pub fn replace_transform(&mut self, trans: Transform) {
        self.transform = trans;
    }
}
