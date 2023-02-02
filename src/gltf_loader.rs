use crate::{camera::Camera, mesh::Mesh, render_utils::argb8_to_u32, texture::Texture};
use gltf::{self, Gltf};

pub struct Model<'a> {
    pub meshes: Vec<Mesh<'a>>,
    pub textures: Vec<Texture>,
}

impl<'a> Model<'a> {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            textures: Vec::new(),
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
                        .map(|rgb| argb8_to_u32(255, rgb[0], rgb[1], rgb[2]))
                        .collect()
                }
                gltf::image::Format::R8G8B8A8 => todo!(),
                gltf::image::Format::R16 => todo!(),
                gltf::image::Format::R16G16 => todo!(),
                gltf::image::Format::R16G16B16 => todo!(),
                gltf::image::Format::R16G16B16A16 => todo!(),
                gltf::image::Format::R32G32B32FLOAT => todo!(),
                gltf::image::Format::R32G32B32A32FLOAT => todo!(),
            }

            model.textures.push(Texture {
                width: image.width,
                height: image.height,
                data,
            })
        }

        for scene in document.scenes() {
            for node in scene.nodes() {
                if let Some(mesh) = node.mesh() {
                    let my_mesh = Mesh::gltf_load_mesh(&mesh, &buffers);
                    model.meshes.push(my_mesh);
                }
            }
        }

        model
    }

    pub fn render(&self, buffer: &mut [u32], depth: &mut [f32], camera: &Camera) {
        for mesh in &self.meshes {
            mesh.render(buffer, depth, camera)
        }
    }

    pub fn next_render_mode(&mut self) {
        for mesh in &mut self.meshes {
            mesh.next_render_mode();
        }
    }
}
