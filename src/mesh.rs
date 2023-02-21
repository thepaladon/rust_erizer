use std::sync::Arc;

use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use rand::Rng;

use crate::{
    camera::Camera, data::Vertex, material::Material, render_utils, sliced_buffer::SlicedBuffers,
    texture::Texture, transform::Transform, triangle::Triangle,
};
pub enum RenderMode {
    Color,
    VertexColor,
    Texture,
    TextureColor,
    Normal,
    Uv,
    Bary,
    Depth,
    Aabb,
}

impl RenderMode {
    fn next_mode(&self) -> Self {
        use RenderMode::*;
        match *self {
            Color => VertexColor,
            VertexColor => Texture,
            Texture => TextureColor,
            TextureColor => Normal,
            Normal => Uv,
            Uv => Bary,
            Bary => Depth,
            Depth => Aabb,
            Aabb => Color,
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material: Material,
    pub texture: Option<Arc<Texture>>,
    pub transform: Transform,
    pub render_mode: RenderMode,
    pub aa_bb: Option<[Vec3; 2]>,
}

impl Mesh {
    //Default Empty Constructor
    pub fn new() -> Self {
        let material = Material {
            base_color: Vec4::splat(255.0),
            ..Default::default()
        };
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            material,
            texture: None,
            transform: Transform::IDENTITY,
            render_mode: RenderMode::VertexColor,
            aa_bb: None,
        }
    }

    pub fn from_color(vertices: &[Vertex], indices: &[u32], color: Vec4) -> Self {
        let material = Material {
            base_color: color,
            ..Default::default()
        };

        let aa_bb = Self::get_vertex_min_max(vertices);

        Self {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
            material,
            texture: None,
            transform: Transform::IDENTITY,
            render_mode: RenderMode::Color,
            aa_bb: Some(aa_bb),
        }
    }

    fn get_vertex_min_max(vertices: &[Vertex]) -> [Vec3; 2] {
        let mut max = Vec3::splat(-f32::INFINITY);
        let mut min = Vec3::splat(f32::INFINITY);

        for v in vertices {
            min = min.min(v.position.xyz());
            max = max.max(v.position.xyz());
        }

        [min, max]
    }

    fn cull_mesh_frustum(&self, mvp: Mat4) -> bool {
        let min = mvp * Vec4::from((self.aa_bb.unwrap()[0], 1.0));
        let max = mvp * Vec4::from((self.aa_bb.unwrap()[1], 1.0));

        // Check if the object is too far away
        if min.z > min.w && max.z > max.w {
            // dbg!("Far Culling");
            return false;
        }

        // Check if the object is behind the near plane
        if min.z < 0.0 && max.z < 0.0 {
            // dbg!("Near Culling");
            return false;
        }

        //// Check if the object is outside to the left or right
        //if min.x.abs() > min.w && max.x.abs() > max.w {
        //    // dbg!("Left/Right Culling");
        //    return false;
        //}

        //// Check if the object is outside up or down
        //if min.y.abs() > min.w && max.y.abs() > max.w {
        //    // dbg!("Up/Down Culling");
        //    return false;
        //}

        true
    }

    pub fn from_texture(vertices: &[Vertex], indices: &[u32], texture: &Arc<Texture>) -> Self {
        assert!(
            indices.len() % 3 == 0,
            "Indices size is wrong. {} % 3 == 0",
            indices.len()
        );

        let material = Material {
            base_tex_idx: -1,
            base_color: Vec4::splat(255.0),
        };

        let aa_bb = Self::get_vertex_min_max(vertices);

        Self {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
            material,
            texture: Some(texture.clone()),
            transform: Transform::IDENTITY,
            render_mode: RenderMode::VertexColor,
            aa_bb: Some(aa_bb),
        }
    }

    pub fn replace_transform(&mut self, trans: Transform) {
        self.transform = trans;
    }

    pub fn next_render_mode(&mut self) {
        self.render_mode = self.render_mode.next_mode();
    }

    pub fn render(
        &self,
        slice_buff: &mut SlicedBuffers,
        camera: &Camera,
        parent_trans: &Transform,
    ) {
        let model = self.transform.local() * parent_trans.local();
        let mvp = camera.perspective() * camera.view() * model;

        let triangles_to_render: Vec<Triangle> = Vec::new();

        if self.cull_mesh_frustum(mvp) {
            for i in (0..self.indices.len()).step_by(3) {
                let tri_idx: [usize; 3] = [
                    self.indices[i] as usize,
                    self.indices[i + 1] as usize,
                    self.indices[i + 2] as usize,
                ];

                let clip0 = mvp * self.vertices[tri_idx[0]].position;
                let clip1 = mvp * self.vertices[tri_idx[1]].position;
                let clip2 = mvp * self.vertices[tri_idx[2]].position;

                //https://github.com/graphitemaster/normals_revisited
                let norm0 = render_utils::cofactor(model)
                    * Vec4::from((self.vertices[tri_idx[0]].normal, 0.0));
                let norm1 = render_utils::cofactor(model)
                    * Vec4::from((self.vertices[tri_idx[1]].normal, 0.0));
                let norm2 = render_utils::cofactor(model)
                    * Vec4::from((self.vertices[tri_idx[2]].normal, 0.0));

                let mut copy0 = self.vertices[tri_idx[0]];
                let mut copy1 = self.vertices[tri_idx[1]];
                let mut copy2 = self.vertices[tri_idx[2]];

                copy0.position = clip0;
                copy1.position = clip1;
                copy2.position = clip2;

                copy0.normal = norm0.xyz();
                copy1.normal = norm1.xyz();
                copy2.normal = norm2.xyz();

                let triangle = Triangle::new([copy0, copy1, copy2]);

                //Collect all triangles here and pass them as a mesh to FS
                match triangle.render_triangle() {
                    crate::triangle::ClipResult::Clipped => { /* Fuck all */ }
                    crate::triangle::ClipResult::One(tri) => {
                        slice_buff.triangles.push(tri);
                    }
                    crate::triangle::ClipResult::Two(tri) => {
                        slice_buff.triangles.push(tri.0);
                        slice_buff.triangles.push(tri.1);
                    }
                }
            }
        }
    }

    pub fn gltf_load_mesh(primitive: &gltf::Primitive, buffers: &[gltf::buffer::Data]) -> Self {
        let mut positions: Vec<Vec3> = Vec::new();
        let mut tex_coords: Vec<Vec2> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut indices = vec![];

        let mut mesh_result = Mesh::new();
        let mut mat_result = Material::default();

        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        if let Some(indices_reader) = reader.read_indices() {
            indices_reader.into_u32().for_each(|i| indices.push(i));
        }
        if let Some(positions_reader) = reader.read_positions() {
            positions_reader.for_each(|p| positions.push(Vec3::new(p[0], p[1], p[2])));
        }
        if let Some(normals_reader) = reader.read_normals() {
            normals_reader.for_each(|n| normals.push(Vec3::new(n[0], n[1], n[2])));
        }
        if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
            tex_coord_reader
                .into_f32()
                .for_each(|tc| tex_coords.push(Vec2::new(tc[0], tc[1])));
        }

        let mut rng = rand::thread_rng();
        let colors: Vec<Vec3> = positions
            .iter()
            .map(|_| Vec3::new(rng.gen(), rng.gen(), rng.gen()))
            .collect();
        println!("Num indices: {:?}", indices.len());
        println!("Tex_coords: {:?}", tex_coords.len());
        println!("Positions: {:?}", positions.len());

        let material = primitive.material();
        let base_col_option = material.pbr_metallic_roughness().base_color_texture();
        let base_col_factor = material.pbr_metallic_roughness().base_color_factor();
        //let met_rough = material
        //    .pbr_metallic_roughness()
        //    .metallic_roughness_texture()
        //    .unwrap();

        let mut base_col = -1;
        if let Some(mat) = base_col_option {
            base_col = mat.texture().index() as i32;
        }

        mat_result.base_color = Vec4::from(base_col_factor);
        mat_result.base_tex_idx = base_col;

        mesh_result.material = mat_result;
        mesh_result.add_section_from_buffers(&indices, &positions, &normals, &colors, &tex_coords);

        let aa_bb = Self::get_vertex_min_max(&mesh_result.vertices);

        mesh_result.aa_bb = Some(aa_bb);

        mesh_result
    }

    pub fn add_ref_tex(&mut self, texture: &Arc<Texture>) {
        self.texture = Some(texture.clone());
    }

    pub fn add_section_from_buffers(
        &mut self,
        indices: &[u32],
        positions: &[Vec3],
        normals: &[Vec3],
        colors: &[Vec3],
        tex_coords: &[Vec2],
    ) {
        for i in 0..positions.len() {
            let v = Vertex {
                position: Vec4::from((positions[i], 1.0)),
                normal: normals[i],
                color: colors[i],
                uv: tex_coords[i],
            };
            self.vertices.push(v);
        }

        self.indices.extend_from_slice(indices);
    }
}
