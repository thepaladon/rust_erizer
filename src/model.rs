use crate::{
    camera::Camera, mesh::VertexMesh, sliced_buffer::SlicedBuffers, tex_manager::TEXTURE_MANAGER,
    transform::Transform,
};
use gltf::{self, Gltf};

pub struct Model {
    pub meshes: Vec<VertexMesh>,
    pub textures: Vec<i32>,
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

        {
            let mut manager = TEXTURE_MANAGER.write().unwrap();
            let textures = manager
                .load_from_gltf_images(images)
                .expect("Textures not found");
            model.textures = textures; //Stores idx to textures as they are in the tex_manager
        }

        for scene in document.scenes() {
            for node in scene.nodes() {
                if let Some(mesh) = node.mesh() {
                    for primitive in mesh.primitives() {
                        let mut my_mesh = VertexMesh::gltf_load_mesh(&primitive, &buffers);

                        if my_mesh.material.base_tex_idx != -1 {
                            my_mesh.texture =
                                Some(model.textures[my_mesh.material.base_tex_idx as usize]);
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
