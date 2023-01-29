use crate::render_utils;

use super::data::Vertex;
use glam::{Vec2, Vec3, Vec3Swizzles};
use image::DynamicImage;
use std::ops::Mul;

pub struct Triangle<'a> {
    pub vertices: [Vertex; 3],
    pub color: Vec3,
    pub aabb: Option<[Vec2; 2]>,
    pub texture: Option<&'a DynamicImage>,
}

impl<'a> Triangle<'a> {
    pub fn new_c(vertices: [Vertex; 3], color: Vec3) -> Self {
        let mut tri = Self {
            vertices,
            color,
            aabb: None,
            texture: None,
        };
        tri.calc_aabb(); //This needs to be done every time the triangle position is moved
        tri
    }

    pub fn new_t(vertices: [Vertex; 3], color: Vec3, tex: &'a DynamicImage) -> Self {
        let mut tri = Self {
            vertices,
            color,
            aabb: None,
            texture: Some(tex),
        };
        tri.calc_aabb(); //This needs to be done every time the triangle position is moved
        tri
    }

    pub fn render_to_buffer(&self, buffer: &mut [u32]) {
        match self.aabb {
            //If an AABB exists, check only within that AABB
            Some(aabb) => {
                for x in (aabb[0].x.floor() as usize)..(aabb[1].x.floor() as usize) {
                    for y in (aabb[0].y.floor() as usize)..(aabb[1].y.floor() as usize) {
                        let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                        let idx: usize = x + y * crate::WIDTH;

                        let src = buffer[idx];
                        let src = render_utils::u32_to_argb8(src);

                        let mut fc = Vec3::new(src[1] as f32, src[2] as f32, src[3] as f32); //final color

                        fc += self.render(p);

                        buffer[idx] =
                            render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
                    }
                }
            }
            //else render in the entire buffer
            None => {
                for x in 0..crate::WIDTH {
                    for y in 0..crate::HEIGHT {
                        let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                        let idx: usize = x + y * crate::WIDTH;

                        let src = buffer[idx];
                        let src = render_utils::u32_to_argb8(src);

                        let mut fc = Vec3::new(src[1] as f32, src[2] as f32, src[3] as f32); //final color

                        fc += self.render(p);

                        buffer[idx] =
                            render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
                    }
                }
            }
        }
    }

    fn calc_aabb(&mut self) {
        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;

        let tmin: Vec2 = Vec2::new(
            v0_p.x.min(v1_p.x).min(v2_p.x),
            v0_p.y.min(v1_p.y).min(v2_p.y),
        );
        let tmax: Vec2 = Vec2::new(
            v0_p.x.max(v1_p.x).max(v2_p.x),
            v0_p.y.max(v1_p.y).max(v2_p.y),
        );

        let taabb = [tmin, tmax];

        self.aabb = Some(taabb);
    }

    fn render(&self, p: Vec2) -> Vec3 {
        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;

        let mut fc = Vec3::new(0.0, 0.0, 0.0);

        // clock wise check
        let area0 = render_utils::edge_fun(p, v0_p.xy(), v1_p.xy());
        let area1 = render_utils::edge_fun(p, v1_p.xy(), v2_p.xy());
        let area2 = render_utils::edge_fun(p, v2_p.xy(), v0_p.xy());

        if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 {
            if let Some(texture) = &self.texture {
                let image_buffer = texture.as_rgb8().expect("Shit's not there >:( ");
                let bary = render_utils::bary_coord([v0_p, v1_p, v2_p], p);

                let v0_uv = self.vertices[0].uv.mul(bary.x);
                let v1_uv = self.vertices[1].uv.mul(bary.y);
                let v2_uv = self.vertices[2].uv.mul(bary.z);

                //Uv coords pog
                let uv = v0_uv + v1_uv + v2_uv;

                let img_width = image_buffer.width() as f32 * uv.x;
                let img_height = image_buffer.height() as f32 * uv.y;

                let color = image_buffer.get_pixel(img_width as u32, img_height as u32);

                fc += Vec3::new(color[0] as f32, color[1] as f32, color[2] as f32);
            } else {
                fc += self.color;
            }
        }

        fc
    }

    fn _render_bary(&self, p: Vec2) -> Vec3 {
        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;

        let mut fc = Vec3::new(0.0, 0.0, 0.0);

        // clock wise check
        let area0 = render_utils::edge_fun(p, v0_p.xy(), v1_p.xy());
        let area1 = render_utils::edge_fun(p, v1_p.xy(), v2_p.xy());
        let area2 = render_utils::edge_fun(p, v2_p.xy(), v0_p.xy());

        if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 {
            fc += render_utils::bary_coord([v0_p, v1_p, v2_p], p);
            fc *= Vec3::new(255.0, 255.0, 255.0);
        }

        fc
    }

    fn _render_uv(&self, p: Vec2) -> Vec3 {
        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;

        let mut fc = Vec3::new(0.0, 0.0, 0.0);

        // clock wise check
        let area0 = render_utils::edge_fun(p, v0_p.xy(), v1_p.xy());
        let area1 = render_utils::edge_fun(p, v1_p.xy(), v2_p.xy());
        let area2 = render_utils::edge_fun(p, v2_p.xy(), v0_p.xy());

        if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 {
            let bary = render_utils::bary_coord([v0_p, v1_p, v2_p], p);

            let v0_uv = self.vertices[0].uv.mul(bary.x);
            let v1_uv = self.vertices[1].uv.mul(bary.y);
            let v2_uv = self.vertices[2].uv.mul(bary.z);

            //Uv coords pog
            let uv = (v0_uv + v1_uv + v2_uv) * Vec2::new(255.0, 255.0);

            fc += Vec3::new(uv.x, uv.y, 0.0);
        }

        fc
    }

    fn _render_tex(&self, p: Vec2, tex: &DynamicImage) -> Vec3 {
        let v0_p = self.vertices[0].positions;
        let v1_p = self.vertices[1].positions;
        let v2_p = self.vertices[2].positions;

        let mut fc = Vec3::new(0.0, 0.0, 0.0);

        // clock wise check
        let area0 = render_utils::edge_fun(p, v0_p.xy(), v1_p.xy());
        let area1 = render_utils::edge_fun(p, v1_p.xy(), v2_p.xy());
        let area2 = render_utils::edge_fun(p, v2_p.xy(), v0_p.xy());

        let image_buffer = tex.as_rgb8().expect("Shit's not there >:( ");

        if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 {
            let bary = render_utils::bary_coord([v0_p, v1_p, v2_p], p);

            let v0_uv = self.vertices[0].uv.mul(bary.x);
            let v1_uv = self.vertices[1].uv.mul(bary.y);
            let v2_uv = self.vertices[2].uv.mul(bary.z);

            //Uv coords pog
            let uv = v0_uv + v1_uv + v2_uv;

            let img_width = image_buffer.width() as f32 * uv.x;
            let img_height = image_buffer.height() as f32 * uv.y;

            let color = image_buffer.get_pixel(img_width as u32, img_height as u32);

            fc += Vec3::new(color[0] as f32, color[1] as f32, color[2] as f32);
        }

        fc
    }
}
