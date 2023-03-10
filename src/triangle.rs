use std::sync::Arc;

use crate::{
    material::Material,
    mesh::RenderMode,
    render_utils::{self, edge_fun},
    sampler::*,
    texture::Texture,
};

use super::data::Vertex;
use glam::{IVec2, Vec2, Vec3, Vec4, Vec4Swizzles};

#[derive(Copy, Clone)]
pub struct Triangle {
    pub v: [Vertex; 3],
    pub rec: [f32; 3],  //Perspective Correction Coords
    pub ssc: [Vec2; 3], //screen coordinates
    pub total_area: f32,
    pub aabb: Option<[Vec2; 2]>, // 0 -> min / 1 -> max
}

#[allow(clippy::upper_case_acronyms)]
pub enum VerticesOrder {
    ABC,
    ACB,
    BAC,
    BCA,
    CAB,
    CBA,
}
#[allow(clippy::large_enum_variant)]
pub enum ClipResult {
    Clipped,
    One(Triangle),
    Two((Triangle, Triangle)),
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self {
            v: vertices,
            aabb: None,
            rec: [0.0, 0.0, 0.0],
            ssc: [Vec2::splat(0.0); 3],
            total_area: 0.0,
        }
    }

    pub fn reorder(&self, order: VerticesOrder) -> Triangle {
        let mut copy = *self;
        match order {
            VerticesOrder::ABC => *self,
            VerticesOrder::ACB => {
                copy.v[0] = self.v[0];
                copy.v[2] = self.v[1];
                copy.v[1] = self.v[2];
                copy
            }
            VerticesOrder::BAC => {
                copy.v[1] = self.v[0];
                copy.v[0] = self.v[1];
                copy.v[2] = self.v[2];
                copy
            }
            VerticesOrder::BCA => {
                copy.v[1] = self.v[2];
                copy.v[2] = self.v[0];
                copy.v[0] = self.v[1];
                copy
            }
            VerticesOrder::CAB => {
                copy.v[2] = self.v[1];
                copy.v[0] = self.v[2];
                copy.v[1] = self.v[0];
                copy
            }
            VerticesOrder::CBA => {
                copy.v[2] = self.v[0];
                copy.v[1] = self.v[1];
                copy.v[0] = self.v[2];
                copy
            }
        }
    }

    pub fn triangle_to_screen(triangle: &Triangle) -> Triangle {
        let mut tri = *triangle;

        // Used for Perspective Correct Mapping for vertices
        let rec0 = 1.0 / tri.v[0].position.w;
        let rec1 = 1.0 / tri.v[1].position.w;
        let rec2 = 1.0 / tri.v[2].position.w;

        //Normalized Device Coordinates [-1 ; 1]
        let ndc0 = tri.v[0].position * rec0;
        let ndc1 = tri.v[1].position * rec1;
        let ndc2 = tri.v[2].position * rec2;

        // screeen coordinates remapped to window
        let sc0 = glam::vec2(
            render_utils::map_to_range(ndc0.x, -1.0, 1.0, 0.0, crate::BUFF_WIDTH as f32),
            render_utils::map_to_range(-ndc0.y, -1.0, 1.0, 0.0, crate::BUFF_HEIGHT as f32),
        );
        let sc1 = glam::vec2(
            render_utils::map_to_range(ndc1.x, -1.0, 1.0, 0.0, crate::BUFF_WIDTH as f32),
            render_utils::map_to_range(-ndc1.y, -1.0, 1.0, 0.0, crate::BUFF_HEIGHT as f32),
        );
        let sc2 = glam::vec2(
            render_utils::map_to_range(ndc2.x, -1.0, 1.0, 0.0, crate::BUFF_WIDTH as f32),
            render_utils::map_to_range(-ndc2.y, -1.0, 1.0, 0.0, crate::BUFF_HEIGHT as f32),
        );

        tri.calc_aabb([sc0, sc1, sc2]);
        let total_area = edge_fun(sc0, sc1, sc2);

        tri.v[0].position = ndc0;
        tri.v[1].position = ndc1;
        tri.v[2].position = ndc2;

        tri.rec[0] = rec0;
        tri.rec[1] = rec1;
        tri.rec[2] = rec2;

        tri.ssc[0] = sc0;
        tri.ssc[1] = sc1;
        tri.ssc[2] = sc2;

        tri.total_area = total_area;

        //I need to add the triangles here to some sort of VECTOR
        tri
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_fragments(
        &self,
        pos: IVec2,
        size: IVec2,
        color_buff: &mut [u32],
        depth_buff: &mut [f32],
        texture: Option<&Arc<Texture>>,
        material: &Material,
        render_type: &RenderMode,
    ) {
        for idx_x in 0..size.x {
            for idx_y in 0..size.y {
                let x = pos.x + idx_x;
                let y = pos.y + idx_y;

                //Fragment Shader
                let p = Vec2::new(x as f32, y as f32) + 0.5;
                let idx: usize = (idx_x + idx_y * size.x) as usize;

                let src = color_buff[idx];
                let src = render_utils::u32_to_argb8(src);

                let fc = Vec3::new(src[1] as f32, src[2] as f32, src[3] as f32); //final color

                if let RenderMode::Aabb = render_type {
                    self.render_aabb(color_buff, idx);
                }

                // clock wise check
                let area0 = render_utils::edge_fun(p, self.ssc[1], self.ssc[2]) / self.total_area;
                let area1 = render_utils::edge_fun(p, self.ssc[2], self.ssc[0]) / self.total_area;
                let area2 = render_utils::edge_fun(p, self.ssc[0], self.ssc[1]) / self.total_area;
                let m_all_sign = ((area0.to_bits() | area1.to_bits() | area2.to_bits()) >> 31) == 0;

                if m_all_sign {
                    match render_type {
                        RenderMode::Default => {
                            self.render_default(
                                p,
                                self.rec,
                                self.ssc,
                                self.total_area,
                                color_buff,
                                depth_buff,
                                texture,
                                material,
                                idx,
                            );
                        }
                        RenderMode::Error => {
                            self.render_error(
                                p,
                                self.rec,
                                self.ssc,
                                self.total_area,
                                color_buff,
                                depth_buff,
                                idx,
                            );
                        }
                        RenderMode::VertexColor => {
                            self.render_pixel_vertex_col(
                                p,
                                self.rec,
                                self.ssc,
                                self.total_area,
                                color_buff,
                                depth_buff,
                                idx,
                            );
                        }
                        RenderMode::Texture => {
                            if let Some(texture) = &texture {
                                self.render_pixel_texture(
                                    p,
                                    self.rec,
                                    self.ssc,
                                    self.total_area,
                                    color_buff,
                                    depth_buff,
                                    texture,
                                    idx,
                                );
                            } else {
                                self.render_error(
                                    p,
                                    self.rec,
                                    self.ssc,
                                    self.total_area,
                                    color_buff,
                                    depth_buff,
                                    idx,
                                );
                            }
                        }
                        RenderMode::TextureColor => {
                            if let Some(texture) = &texture {
                                self.render_pixel_tex_col(
                                    p,
                                    self.rec,
                                    self.ssc,
                                    self.total_area,
                                    color_buff,
                                    depth_buff,
                                    texture,
                                    idx,
                                );
                            } else {
                                self.render_pixel_vertex_col(
                                    p,
                                    self.rec,
                                    self.ssc,
                                    self.total_area,
                                    color_buff,
                                    depth_buff,
                                    idx,
                                );
                            }
                        }
                        RenderMode::Uv => {
                            self.render_pixel_uv(
                                p,
                                self.rec,
                                self.ssc,
                                self.total_area,
                                color_buff,
                                depth_buff,
                                idx,
                            );
                        }
                        RenderMode::Bary => {
                            self.render_pixel_bary(
                                p,
                                self.rec,
                                self.ssc,
                                self.total_area,
                                color_buff,
                                depth_buff,
                                idx,
                            );
                        }
                        RenderMode::Depth => {
                            self.render_pixel_depth(
                                p,
                                self.rec,
                                self.ssc,
                                self.total_area,
                                color_buff,
                                depth_buff,
                                idx,
                            );
                        }
                        RenderMode::Aabb => {
                            //covered above.
                        }
                        RenderMode::Normal => {
                            self.render_pixel_normal(
                                p,
                                self.rec,
                                self.ssc,
                                self.total_area,
                                color_buff,
                                depth_buff,
                                idx,
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn render_triangle(&self) -> ClipResult {
        match Self::clip_cull_triangle(self) {
            ClipResult::Clipped => ClipResult::Clipped,
            ClipResult::One(tri) => ClipResult::One(Self::triangle_to_screen(&tri)),
            ClipResult::Two(tri) => ClipResult::Two((
                Self::triangle_to_screen(&tri.0),
                Self::triangle_to_screen(&tri.1),
            )),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_default(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        texture: Option<&Arc<Texture>>,
        material: &Material,
        idx: usize,
    ) {
        //Bary -> Depth
        let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);
        let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
        let correction = 1.0 / correction;
        let depth = bary.x * self.v[0].position.z
            + bary.y * self.v[1].position.z
            + bary.z * self.v[2].position.z;

        if depth < z_buffer[idx] {
            z_buffer[idx] = depth;

            let v0_uv = self.v[0].uv * rec[0];
            let v1_uv = self.v[1].uv * rec[1];
            let v2_uv = self.v[2].uv * rec[2];

            let v0_normal = self.v[0].normal * rec[0];
            let v1_normal = self.v[1].normal * rec[1];
            let v2_normal = self.v[2].normal * rec[2];

            let uv = v0_uv * bary.x + v1_uv * bary.y + v2_uv * bary.z;
            let uv = uv * correction;

            let mut tex_color = Vec4::splat(1.0);
            if let Some(texture) = texture {
                let uv = Triangle::calc_uv_sampler(uv, &texture.sampler);

                let img_width = (texture.width as f32 - 1.0) * uv.x;
                let img_height = (texture.height as f32 - 1.0) * uv.y;

                //let img_width = f32::clamp(img_width, 0, tex )

                if img_width < 0.0 || img_width >= texture.width as f32 {
                    let b = 1.0;
                    panic!("Image WIDTH out of bounds. Value: {img_width}");
                }
                if img_height < 0.0 || img_height >= texture.height as f32 {
                    panic!("Image HEIGHT out of bounds. Value: {img_height}")
                }

                tex_color = texture.get_pixel(img_width as u32, img_height as u32);
                tex_color *= 0.003_921_569; //reciprocal of 255
            }

            let normals = v0_normal * bary.x + v1_normal * bary.y + v2_normal * bary.z;
            let normals = normals * correction;

            let light_dir = Vec3::new(-1.0, -1.0, -1.0).normalize();
            let ambient_col = Vec4::new(0.2, 0.2, 0.2, 1.0);

            let intensity = normals.dot(-light_dir);
            let object_col = tex_color * material.base_color;

            let diffuse = f32::max(intensity, 0.0) * object_col;
            let ambient = ambient_col * object_col;

            let mut fc = ambient + diffuse;
            fc *= 255.0;

            color_buff[idx] = render_utils::vec4_to_u32(fc);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_error(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        idx: usize,
    ) {
        let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

        let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
        let correction = 1.0 / correction;
        let depth = bary.x * self.v[0].position.z
            + bary.y * self.v[1].position.z
            + bary.z * self.v[2].position.z;

        if depth < z_buffer[idx] {
            z_buffer[idx] = depth;

            //Color
            let color = self.v[0].position * bary.x
                + self.v[1].position * bary.y
                + self.v[2].position * bary.z;

            let fc = Vec3::new(
                f32::abs(f32::sin(color.x * 209.0)),
                0.0,
                f32::abs(f32::sin(color.x * 209.0)),
            ) * 255.0;

            color_buff[idx] = render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_pixel_texture(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        texture: &Texture,
        idx: usize,
    ) {
        let v0_uv = self.v[0].uv * rec[0];
        let v1_uv = self.v[1].uv * rec[1];
        let v2_uv = self.v[2].uv * rec[2];

        let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

        let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
        let correction = 1.0 / correction;
        let depth = bary.x * self.v[0].position.z
            + bary.y * self.v[1].position.z
            + bary.z * self.v[2].position.z;

        if depth < z_buffer[idx] {
            z_buffer[idx] = depth;

            let uv = v0_uv * bary.x + v1_uv * bary.y + v2_uv * bary.z;
            let uv = uv * correction;

            let uv = Triangle::calc_uv_sampler(uv, &texture.sampler);

            let img_width = (texture.width as f32 - 1.0) * uv.x;
            let img_height = (texture.height as f32 - 1.0) * uv.y;

            //let img_width = f32::clamp(img_width, 0, tex )

            if img_width < 0.0 || img_width >= texture.width as f32 {
                let b = 1.0;
                panic!("Image WIDTH out of bounds. Value: {img_width}");
            }
            if img_height < 0.0 || img_height >= texture.height as f32 {
                panic!("Image HEIGHT out of bounds. Value: {img_height}")
            }

            let color = texture.get_pixel(img_width as u32, img_height as u32);

            let fc = Vec3::new(color[0], color[1], color[2]);

            color_buff[idx] = render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_pixel_tex_col(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        texture: &Texture,
        idx: usize,
    ) {
        let v0_uv = self.v[0].uv * rec[0];
        let v1_uv = self.v[1].uv * rec[1];
        let v2_uv = self.v[2].uv * rec[2];
        let v0_color = self.v[0].color * rec[0];
        let v1_color = self.v[1].color * rec[1];
        let v2_color = self.v[2].color * rec[2];

        let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

        let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
        let correction = 1.0 / correction;
        let depth = bary.x * self.v[0].position.z
            + bary.y * self.v[1].position.z
            + bary.z * self.v[2].position.z;

        if depth < z_buffer[idx] {
            z_buffer[idx] = depth;

            let v_color = v0_color * bary.x + v1_color * bary.y + v2_color * bary.z;
            let uv = v0_uv * bary.x + v1_uv * bary.y + v2_uv * bary.z;
            let v_color = v_color * correction;
            let uv = uv * correction;

            let uv = Triangle::calc_uv_sampler(uv, &texture.sampler);
            let v_color = v_color * Vec3::splat(255.0);

            let img_width = (texture.width as f32 - 1.0) * uv.x;
            let img_height = (texture.height as f32 - 1.0) * uv.y;

            if img_width < 0.0 || img_width >= texture.width as f32 {
                panic!("Image WIDTH out of bounds.")
            }
            if img_height < 0.0 || img_height >= texture.height as f32 {
                panic!("Image HEIGHT out of bounds.")
            }

            let color = texture.get_pixel(img_width as u32, img_height as u32);

            let fc = (Vec3::new(color[0], color[1], color[2]) + v_color) / 2.0;

            color_buff[idx] = render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_pixel_normal(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        idx: usize,
    ) {
        let v0_normal = self.v[0].normal * rec[0];
        let v1_normal = self.v[1].normal * rec[1];
        let v2_normal = self.v[2].normal * rec[2];

        let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

        let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
        let correction = 1.0 / correction;
        let depth = bary.x * self.v[0].position.z
            + bary.y * self.v[1].position.z
            + bary.z * self.v[2].position.z;

        if depth < z_buffer[idx] {
            z_buffer[idx] = depth;

            let normal = v0_normal * bary.x + v1_normal * bary.y + v2_normal * bary.z;
            let normal = normal * correction;

            let normal = normal * 0.5 + 0.5; //correction for rendering normals ffddirectly
            let color = normal * Vec3::splat(255.0);

            let fc = Vec3::new(color[0], color[1], color[2]);

            color_buff[idx] = render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_pixel_bary(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        idx: usize,
    ) {
        let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

        let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
        let correction = 1.0 / correction;
        let depth = bary.x * self.v[0].position.z
            + bary.y * self.v[1].position.z
            + bary.z * self.v[2].position.z;

        if depth < z_buffer[idx] {
            z_buffer[idx] = depth;

            //Bary
            let fc = bary * Vec3::splat(255.0);

            color_buff[idx] = render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_pixel_uv(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        idx: usize,
    ) {
        let v0_uv = self.v[0].uv * rec[0];
        let v1_uv = self.v[1].uv * rec[1];
        let v2_uv = self.v[2].uv * rec[2];

        let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

        let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
        let correction = 1.0 / correction;
        let depth = bary.x * self.v[0].position.z
            + bary.y * self.v[1].position.z
            + bary.z * self.v[2].position.z;

        if depth < z_buffer[idx] {
            z_buffer[idx] = depth;

            let uv = v0_uv * bary.x + v1_uv * bary.y + v2_uv * bary.z;
            let uv = uv * correction;

            let uv = uv.clamp(Vec2::splat(0.0), Vec2::splat(1.0));

            //UV
            let fc = Vec3::new(uv.x, uv.y, 0.0) * Vec3::splat(255.0);

            color_buff[idx] = render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_pixel_vertex_col(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        idx: usize,
    ) {
        let v0_color = self.v[0].color * rec[0];
        let v1_color = self.v[1].color * rec[1];
        let v2_color = self.v[2].color * rec[2];

        let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

        let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
        let correction = 1.0 / correction;
        let depth = bary.x * self.v[0].position.z
            + bary.y * self.v[1].position.z
            + bary.z * self.v[2].position.z;

        if depth < z_buffer[idx] {
            z_buffer[idx] = depth;

            let color = v0_color * bary.x + v1_color * bary.y + v2_color * bary.z;
            let color = color * correction;

            //UV
            let fc = Vec3::new(color.x, color.y, color.z) * Vec3::splat(255.0);

            color_buff[idx] = render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_pixel_depth(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        idx: usize,
    ) {
        let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

        let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
        let correction = 1.0 / correction;
        let depth = bary.x * self.v[0].position.z
            + bary.y * self.v[1].position.z
            + bary.z * self.v[2].position.z;

        if depth < z_buffer[idx] {
            z_buffer[idx] = depth;

            //Depth
            let fc = depth * Vec3::splat(255.0);

            color_buff[idx] = render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
        }
    }

    fn render_aabb(&self, color_buff: &mut [u32], idx: usize) {
        //AABB
        color_buff[idx] = render_utils::argb8_to_u32(255, 0, 0, 255);
    }

    pub fn clip_cull_triangle(tri: &Triangle) -> ClipResult {
        // All triangles not facing the camera are discarded
        //if Self::cull_triangle_backface(&tri.v[0].position, &tri.v[1].position, &tri.v[2].position) {
        //    return ClipResult::Clipped;
        //}

        if Self::cull_triangle_view_frustum(tri) {
            ClipResult::Clipped
        } else {
            // clipping routines
            if tri.v[0].position.z < 0.0 {
                if tri.v[1].position.z < 0.0 {
                    ClipResult::One(Self::clip_triangle_one(tri))
                } else if tri.v[2].position.z < 0.0 {
                    ClipResult::One(Self::clip_triangle_one(&tri.reorder(VerticesOrder::ACB)))
                } else {
                    ClipResult::Two(Self::clip_triangle_two(&tri.reorder(VerticesOrder::ACB)))
                }
            } else if tri.v[1].position.z < 0.0 {
                if tri.v[2].position.z < 0.0 {
                    ClipResult::One(Self::clip_triangle_one(&tri.reorder(VerticesOrder::BCA)))
                } else {
                    ClipResult::Two(Self::clip_triangle_two(&tri.reorder(VerticesOrder::BAC)))
                }
            } else if tri.v[2].position.z < 0.0 {
                ClipResult::Two(Self::clip_triangle_two(&tri.reorder(VerticesOrder::CBA)))
            } else {
                // no near clipping necessary
                //return original
                ClipResult::One(*tri)
            }
        }
    }

    //Cheers to Andrei, for fixing this for me
    // All triangles not facing the camera are discarded
    pub fn cull_triangle_backface(pos_0: &Vec4, pos_1: &Vec4, pos_2: &Vec4) -> bool {
        // Get triangle normal
        let normal = (pos_1.xyz() - pos_0.xyz())
            .cross(pos_2.xyz() - pos_0.xyz())
            .normalize();

        let view_dir0 = pos_0.xyz().normalize();
        let view_dir1 = pos_1.xyz().normalize();
        let view_dir2 = pos_2.xyz().normalize();

        let dot0 = view_dir0.dot(normal);
        let dot1 = view_dir1.dot(normal);
        let dot2 = view_dir2.dot(normal);

        dot0 < 0.0 && dot1 < 0.0 && dot2 < 0.0
    }

    pub fn clip_triangle_one(&self) -> Triangle {
        let v0z = self.v[0].position.z;
        let v1z = self.v[1].position.z;
        let v2z = self.v[2].position.z;

        // calculate alpha values for getting adjusted vertices
        let alpha_a = (-v0z) / (v2z - v0z);
        let alpha_b = (-v1z) / (v2z - v1z);

        // interpolate to get v0a and v0b
        let v0 = render_utils::lerp(self.v[0], self.v[2], alpha_a);
        let v1 = render_utils::lerp(self.v[1], self.v[2], alpha_b);
        let v2 = self.v[2];

        if f32::abs(alpha_b) > 1.0 || f32::abs(alpha_a) > 1.0 {
            let i = 1.0;
        };

        let mut copy = *self;
        copy.v[0] = v0;
        copy.v[1] = v1;
        copy.v[2] = v2;

        copy
    }

    fn clip_triangle_two(&self) -> (Triangle, Triangle) {
        // calculate alpha values for getting adjusted vertices
        let alpha_a = (-self.v[0].position.z) / (self.v[1].position.z - self.v[0].position.z);
        let alpha_b = (-self.v[0].position.z) / (self.v[2].position.z - self.v[0].position.z);

        // interpolate to get v0a and v0b
        let v0_a = render_utils::lerp(self.v[0], self.v[1], alpha_a);
        let v0_b = render_utils::lerp(self.v[0], self.v[2], alpha_b);

        let mut result_a = *self;
        let mut result_b = *self;

        result_a.v[0] = v0_a;
        result_b.v[0] = v0_a;
        result_b.v[1] = v0_b;

        (result_a, result_b)
    }

    pub fn cull_triangle_view_frustum(triangle: &Triangle) -> bool {
        // cull tests against the 6 planes
        if triangle.v[0].position.x.abs() > triangle.v[0].position.w
            && triangle.v[1].position.x.abs() > triangle.v[1].position.w
            && triangle.v[2].position.x.abs() > triangle.v[2].position.w
        {
            return true;
        }

        if triangle.v[0].position.y.abs() > triangle.v[0].position.w
            && triangle.v[1].position.y.abs() > triangle.v[1].position.w
            && triangle.v[2].position.y.abs() > triangle.v[2].position.w
        {
            return true;
        }

        if triangle.v[0].position.z > triangle.v[0].position.w
            && triangle.v[1].position.z > triangle.v[1].position.w
            && triangle.v[2].position.z > triangle.v[2].position.w
        {
            return true;
        }
        if triangle.v[0].position.z < 0.0
            && triangle.v[1].position.z < 0.0
            && triangle.v[2].position.z < 0.0
        {
            return true;
        }

        false
    }

    fn calc_uv_sampler(uv: Vec2, sampler: &Sampler) -> Vec2 {
        Vec2::new(
            match sampler.wrap_s {
                Wrap::ClampToEdge => clamp_to_edge(uv.x),
                Wrap::Repeat => repeat(uv.x),
                Wrap::Mirror => mirror(uv.x),
            },
            match sampler.wrap_s {
                Wrap::ClampToEdge => clamp_to_edge(uv.y),
                Wrap::Repeat => repeat(uv.y),
                Wrap::Mirror => mirror(uv.y),
            },
        )
    }

    fn calc_aabb(&mut self, vertices: [Vec2; 3]) {
        let v0_p = vertices[0];
        let v1_p = vertices[1];
        let v2_p = vertices[2];

        let tmin: Vec2 = Vec2::new(
            v0_p.x.min(v1_p.x).min(v2_p.x),
            v0_p.y.min(v1_p.y).min(v2_p.y),
        );
        let tmax: Vec2 = Vec2::new(
            v0_p.x.max(v1_p.x).max(v2_p.x),
            v0_p.y.max(v1_p.y).max(v2_p.y),
        );

        let mut taabb = [tmin, tmax];

        taabb[0].x = taabb[0].x.max(0.0);
        taabb[0].y = taabb[0].y.max(0.0);
        taabb[1].x = taabb[1].x.min(crate::BUFF_WIDTH as f32 - 1.0);
        taabb[1].y = taabb[1].y.min(crate::BUFF_HEIGHT as f32 - 1.0);

        self.aabb = Some(taabb);
    }
}
