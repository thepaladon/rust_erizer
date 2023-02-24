use crate::{
    camera::Camera, mesh::VertexMesh, sliced_buffer::SlicedBuffers, tex_manager::TEXTURE_MANAGER,
    transform::Transform,
};
use gltf::{self, buffer::Data, Gltf, Node};

pub struct Model {
    pub meshes: Vec<VertexMesh>,
    pub transform: Transform,
    textures: Vec<i32>, //all texture indices of models
}

impl Model {
    fn new() -> Self {
        Self {
            meshes: Vec::new(),
            transform: Transform::IDENTITY,
            textures: Vec::new(),
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
            model.textures = manager
                .load_from_gltf_images(images)
                .expect("Textures not found");
        }

        for scene in document.scenes() {
            for node in scene.nodes() {
                Self::load_data_from_node(&mut model, &node, &buffers);
            }
        }

        model
    }

    fn load_data_from_node(model: &mut Model, node: &Node, buffers: &Vec<Data>) {
        //Load mesh if there's on in the node
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                let mut my_mesh = VertexMesh::gltf_load_mesh(&primitive, buffers);

                if my_mesh.material.base_tex_idx != -1 {
                    my_mesh.texture = Some(model.textures[my_mesh.material.base_tex_idx as usize]);
                }

                model.meshes.push(my_mesh);
            }
        }

        //check for children and load meshes from their nodes
        for child in node.children() {
            Self::load_data_from_node(model, &child, buffers);
        }
    }

    pub fn from_mesh(mesh: VertexMesh, transform: Transform) -> Self {
        let mut model = Self::new();
        model.transform = transform;
        model.meshes.push(mesh);
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

    pub fn prev_render_mode(&mut self) {
        for mesh in &mut self.meshes {
            mesh.prev_render_mode();
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        let mut manager = TEXTURE_MANAGER.write().unwrap();

        for tex in &self.textures {
            manager.destroy_texture(tex);
        }
    }
}
