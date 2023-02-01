use crate::{
    camera::Camera,
    render_utils::{self, edge_fun},
};

use super::data::Vertex;
use glam::{Vec2, Vec3, Vec4, Vec4Swizzles};
use image::DynamicImage;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub v: [Vertex; 3],
    pub aabb: Option<[Vec2; 2]>,
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

    pub fn render_clipped_triangle(
        triangle: &Triangle,
        buffer: &mut [u32],
        depth: &mut [f32],
        camera: &Camera,
        texture: Option<&DynamicImage>,
        color: &Vec3,
        render_type: i32,
    ) {
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
            render_utils::map_to_range(ndc0.x, -1.0, 1.0, 0.0, crate::WIDTH as f32),
            render_utils::map_to_range(-ndc0.y, -1.0, 1.0, 0.0, crate::HEIGHT as f32),
        );
        let sc1 = glam::vec2(
            render_utils::map_to_range(ndc1.x, -1.0, 1.0, 0.0, crate::WIDTH as f32),
            render_utils::map_to_range(-ndc1.y, -1.0, 1.0, 0.0, crate::HEIGHT as f32),
        );
        let sc2 = glam::vec2(
            render_utils::map_to_range(ndc2.x, -1.0, 1.0, 0.0, crate::WIDTH as f32),
            render_utils::map_to_range(-ndc2.y, -1.0, 1.0, 0.0, crate::HEIGHT as f32),
        );

        tri.calc_aabb([sc0, sc1, sc2]);
        let total_area = edge_fun(sc0, sc1, sc2);

        tri.v[0].position = ndc0;
        tri.v[1].position = ndc1;
        tri.v[2].position = ndc2;

        //If an AABB exists, check only within that AABB
        if let Some(aabb) = tri.aabb {
            for x in (aabb[0].x.floor() as i32)..(aabb[1].x.floor() as i32) {
                for y in (aabb[0].y.floor() as i32)..(aabb[1].y.floor() as i32) {
                    let p = Vec2::new(x as f32 - 0.5, y as f32 - 0.5);
                    let idx: usize = x as usize + y as usize * crate::WIDTH;

                    let src = buffer[idx];
                    let src = render_utils::u32_to_argb8(src);

                    let fc = Vec3::new(src[1] as f32, src[2] as f32, src[3] as f32); //final color

                    if let Some(texture) = texture {
                        tri.render_pixel_texture(
                            p,
                            [rec0, rec1, rec2],
                            [sc0, sc1, sc2],
                            total_area,
                            buffer,
                            depth,
                            texture,
                            idx,
                        );
                    }
                }
            }
        }
    }

    pub fn render_triangle(
        &self,
        buffer: &mut [u32],
        depth: &mut [f32],
        camera: &Camera,
        texture: Option<&DynamicImage>,
        color: &Vec3,
        render_type: i32,
    ) {
        match Self::clip_cull_triangle(self) {
            ClipResult::Clipped => {}
            ClipResult::One(tri) => {
                Self::render_clipped_triangle(
                    &tri,
                    buffer,
                    depth,
                    camera,
                    texture,
                    color,
                    render_type,
                );
            }
            ClipResult::Two(tri) => {
                Self::render_clipped_triangle(
                    &tri.0,
                    buffer,
                    depth,
                    camera,
                    texture,
                    color,
                    render_type,
                );
                Self::render_clipped_triangle(
                    &tri.1,
                    buffer,
                    depth,
                    camera,
                    texture,
                    color,
                    render_type,
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_pixel_color(
        &self,
        p: Vec2,
        rec: [f32; 3],
        ssc: [Vec2; 3],
        total_area: f32,
        color_buff: &mut [u32],
        z_buffer: &mut [f32],
        color: &Vec3,
        idx: usize,
    ) {
        // clock wise check
        let area0 = render_utils::edge_fun(p, ssc[1], ssc[2]) / total_area;
        let area1 = render_utils::edge_fun(p, ssc[2], ssc[0]) / total_area;
        let area2 = render_utils::edge_fun(p, ssc[0], ssc[1]) / total_area;

        if area0 >= 0.0 && area1 >= 0.0 && area2 >= 0.0 {
            let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

            let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
            let correction = 1.0 / correction;
            let depth = bary.x * self.v[0].position.z
                + bary.y * self.v[1].position.z
                + bary.z * self.v[2].position.z;

            if depth < z_buffer[idx] {
                z_buffer[idx] = depth;

                //Color
                let fc = color;

                color_buff[idx] =
                    render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
            }
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
        texture: &DynamicImage,
        idx: usize,
    ) {
        let v0_uv = self.v[0].uv * rec[0];
        let v1_uv = self.v[1].uv * rec[1];
        let v2_uv = self.v[2].uv * rec[2];

        // clock wise check
        let area0 = render_utils::edge_fun(p, ssc[1], ssc[2]) / total_area;
        let area1 = render_utils::edge_fun(p, ssc[2], ssc[0]) / total_area;
        let area2 = render_utils::edge_fun(p, ssc[0], ssc[1]) / total_area;

        if area0 >= 0.0 && area1 >= 0.0 && area2 >= 0.0 {
            let bary = render_utils::barycentric_coordinates(p, ssc[0], ssc[1], ssc[2], total_area);

            let correction = bary.x * rec[0] + bary.y * rec[1] + bary.z * rec[2];
            let correction = 1.0 / correction;
            let depth = bary.x * self.v[0].position.z
                + bary.y * self.v[1].position.z
                + bary.z * self.v[2].position.z;

            if depth < z_buffer[idx] {
                z_buffer[idx] = depth;

                let image_buffer = texture.as_rgb8().expect("Shit's not there >:( ");

                let uv = v0_uv * bary.x + v1_uv * bary.y + v2_uv * bary.z;
                let uv = uv * correction;

                let uv = uv.clamp(Vec2::splat(0.0), Vec2::splat(1.0));

                let img_width = (image_buffer.width() as f32 - 1.0) * uv.x;
                let img_height = (image_buffer.height() as f32 - 1.0) * uv.y;

                if img_width < 0.0 || img_width >= image_buffer.width() as f32 {
                    panic!("Image WIDTH out of bounds.")
                }
                if img_height < 0.0 || img_height >= image_buffer.height() as f32 {
                    panic!("Image HEIGHT out of bounds.")
                }

                let color = image_buffer.get_pixel(img_width as u32, img_height as u32);

                let fc = Vec3::new(color[0] as f32, color[1] as f32, color[2] as f32);

                color_buff[idx] =
                    render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
            }
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
        texture: &DynamicImage,
        idx: usize,
    ) {
        // clock wise check
        let area0 = render_utils::edge_fun(p, ssc[1], ssc[2]) / total_area;
        let area1 = render_utils::edge_fun(p, ssc[2], ssc[0]) / total_area;
        let area2 = render_utils::edge_fun(p, ssc[0], ssc[1]) / total_area;

        if area0 >= 0.0 && area1 >= 0.0 && area2 >= 0.0 {
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

                color_buff[idx] =
                    render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
            }
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

        // clock wise check
        let area0 = render_utils::edge_fun(p, ssc[1], ssc[2]) / total_area;
        let area1 = render_utils::edge_fun(p, ssc[2], ssc[0]) / total_area;
        let area2 = render_utils::edge_fun(p, ssc[0], ssc[1]) / total_area;

        if area0 >= 0.0 && area1 >= 0.0 && area2 >= 0.0 {
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

                color_buff[idx] =
                    render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
            }
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
        texture: &DynamicImage,
        idx: usize,
    ) {
        // clock wise check
        let area0 = render_utils::edge_fun(p, ssc[1], ssc[2]) / total_area;
        let area1 = render_utils::edge_fun(p, ssc[2], ssc[0]) / total_area;
        let area2 = render_utils::edge_fun(p, ssc[0], ssc[1]) / total_area;

        if area0 >= 0.0 && area1 >= 0.0 && area2 >= 0.0 {
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

                color_buff[idx] =
                    render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
            }
        }
    }

    fn render_aabb(&self, color_buff: &mut [u32], idx: usize) {
        //AABB
        color_buff[idx] = render_utils::argb8_to_u32(255, 0, 0, 255);
    }

    pub fn clip_cull_triangle(tri: &Triangle) -> ClipResult {
        // All triangles not facing the camera are discarded
        if Self::cull_triangle_backface([tri.v[0].position, tri.v[1].position, tri.v[2].position]) {
            return ClipResult::Clipped;
        }

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

    // All triangles not facing the camera are discarded
    fn cull_triangle_backface(vertices: [Vec4; 3]) -> bool {
        let normal =
            (vertices[1].xyz() - vertices[0].xyz()).cross(vertices[2].xyz() - vertices[0].xyz());

        // any is vertex valid
        let view_dir = -Vec3::Z;

        // also we don't care about normalizing
        // if negative facing the camera
        normal.dot(view_dir) >= 0.0
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

        //println!("out tri: {:?}, {:?}, {:?},", v0, v1, v2);
        // draw triangles
        copy
    }

    fn clip_triangle_two(&self) -> (Triangle, Triangle) {
        // calculate alpha values for getting adjusted vertices
        let alpha_a = (-self.v[0].position.z) / (self.v[1].position.z - self.v[0].position.z);
        let alpha_b = (-self.v[0].position.z) / (self.v[2].position.z - self.v[0].position.z);

        // interpolate to get v0a and v0b
        let v0_a = render_utils::lerp(self.v[0], self.v[1], alpha_a);
        let v0_b = render_utils::lerp(self.v[0], self.v[2], alpha_b);

        // draw triangles
        let mut result_a = *self;
        let mut result_b = *self;

        result_a.v[0] = v0_a;
        result_b.v[0] = v0_a;
        result_b.v[1] = v0_b;

        (result_a, result_b)
    }

    pub fn cull_triangle_view_frustum(triangle: &Triangle) -> bool {
        // cull tests against the 6 planes
        if triangle.v[0].position.x > triangle.v[0].position.w
            && triangle.v[1].position.x > triangle.v[1].position.w
            && triangle.v[2].position.x > triangle.v[2].position.w
        {
            return true;
        }
        if triangle.v[0].position.x < -triangle.v[0].position.w
            && triangle.v[1].position.x < -triangle.v[1].position.w
            && triangle.v[2].position.x < -triangle.v[2].position.w
        {
            return true;
        }
        if triangle.v[0].position.y > triangle.v[0].position.w
            && triangle.v[1].position.y > triangle.v[1].position.w
            && triangle.v[2].position.y > triangle.v[2].position.w
        {
            return true;
        }
        if triangle.v[0].position.y < -triangle.v[0].position.w
            && triangle.v[1].position.y < -triangle.v[1].position.w
            && triangle.v[2].position.y < -triangle.v[2].position.w
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

        //check whether it's within the viewport.
        if taabb[0].x < 0.0 {
            taabb[0].x = 0.0;
        }
        if taabb[0].y < 0.0 {
            taabb[0].y = 0.0;
        }
        if taabb[1].x > crate::WIDTH as f32 {
            taabb[1].x = crate::WIDTH as f32 - 1.0;
        }
        if taabb[1].y > crate::HEIGHT as f32 {
            taabb[1].y = crate::HEIGHT as f32 - 1.0;
        }

        self.aabb = Some(taabb);
    }
}
