use crate::{
    camera::Camera,
    render_utils::{self, edge_fun},
    transform::Transform,
};

use super::data::Vertex;
use glam::{Vec2, Vec3, Vec4};
use image::DynamicImage;
use std::ops::Mul;

pub struct Triangle<'a> {
    pub vertices: [Vertex; 3],
    pub color: Vec3,
    pub aabb: Option<[Vec2; 2]>,
    pub texture: Option<&'a DynamicImage>,

    pub transform: Transform,
}

impl<'a> Triangle<'a> {
    pub fn new_c(vertices: [Vertex; 3], color: Vec3) -> Self {
        Self {
            vertices,
            color,
            aabb: None,
            texture: None,
            transform: Transform::IDENTITY,
        }
    }

    pub fn new_t(vertices: [Vertex; 3], color: Vec3, tex: &'a DynamicImage) -> Self {
        Self {
            vertices,
            color,
            aabb: None,
            texture: Some(tex),
            transform: Transform::IDENTITY,
        }
    }

    pub fn replace_transform(&mut self, trans: Transform) {
        self.transform = trans;
    }

    pub fn render_to_buffer(&mut self, buffer: &mut [u32], camera: &Camera) {
        let mvp = camera.perspective() * camera.view() * self.transform.local();

        let clip0 = mvp
            * Vec4::new(
                self.vertices[0].positions.x,
                self.vertices[0].positions.y,
                self.vertices[0].positions.z,
                1.0,
            );
        let clip1 = mvp
            * Vec4::new(
                self.vertices[1].positions.x,
                self.vertices[1].positions.y,
                self.vertices[1].positions.z,
                1.0,
            );
        let clip2 = mvp
            * Vec4::new(
                self.vertices[2].positions.x,
                self.vertices[2].positions.y,
                self.vertices[2].positions.z,
                1.0,
            );

        let ndc0 = clip0 / clip0.w;
        let ndc1 = clip1 / clip1.w;
        let ndc2 = clip2 / clip2.w;

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

        self.calc_aabb([sc0, sc1, sc2]);
        let total_area = edge_fun(sc0, sc1, sc2);

        match self.aabb {
            //If an AABB exists, check only within that AABB
            Some(aabb) => {
                for x in (aabb[0].x.floor() as i32)..(aabb[1].x.floor() as i32) {
                    for y in (aabb[0].y.floor() as i32)..(aabb[1].y.floor() as i32) {
                        if x < 0 || x > crate::WIDTH as i32 - 1 {
                            break;
                        };
                        if y < 0 || y > crate::HEIGHT as i32 - 1 {
                            break;
                        };

                        let p = Vec2::new(x as f32 - 0.5, y as f32 - 0.5);
                        let idx: usize = x as usize + y as usize * crate::WIDTH;

                        let src = buffer[idx];
                        let src = render_utils::u32_to_argb8(src);

                        let mut fc = Vec3::new(src[1] as f32, src[2] as f32, src[3] as f32); //final color

                        fc += self.render(p, [ndc0, ndc1, ndc2], [sc0, sc1, sc2], total_area);

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

                        fc += self.render(p, [ndc0, ndc1, ndc2], [sc0, sc1, sc2], total_area);

                        fc.clamp(Vec3::ZERO, Vec3::splat(255.0));
                        buffer[idx] =
                            render_utils::argb8_to_u32(255, fc.x as u8, fc.y as u8, fc.z as u8);
                    }
                }
            }
        }
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

        let taabb = [tmin, tmax];

        self.aabb = Some(taabb);
    }

    //ssc - screen space coords
    fn render(&self, p: Vec2, clip_space: [Vec4; 3], ssc: [Vec2; 3], total_area: f32) -> Vec3 {
        let mut fc = Vec3::new(0.0, 0.0, 0.0);

        // clock wise check
        let area0 = render_utils::edge_fun(p, ssc[1], ssc[2]);
        let area1 = render_utils::edge_fun(p, ssc[2], ssc[0]);
        let area2 = render_utils::edge_fun(p, ssc[0], ssc[1]);

        if area0 >= 0.0 && area1 >= 0.0 && area2 >= 0.0
            || area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0
        {
            if let Some(texture) = &self.texture {
                let image_buffer = texture.as_rgb8().expect("Shit's not there >:( ");
                let bary = render_utils::better_bary([area1, area2], p, total_area);

                let v0_uv = self.vertices[0].uv.mul(bary.x);
                let v1_uv = self.vertices[1].uv.mul(bary.y);
                let v2_uv = self.vertices[2].uv.mul(bary.z);

                //Uv coords pog
                let uv = v0_uv + v1_uv + v2_uv;

                let img_width = (image_buffer.width() as f32 - 1.0) * uv.x;
                let img_height = (image_buffer.height() as f32 - 1.0) * uv.y;

                let color = image_buffer.get_pixel(img_width as u32, img_height as u32);

                //Bary
                //fc += bary * Vec3::splat(255.0);

                //UV
                //fc += Vec3::new(uv.x, uv.y, 0.0) * Vec3::splat(255.0);

                //Color
                //fc += self.color;

                //Tex
                fc += Vec3::new(color[0] as f32, color[1] as f32, color[2] as f32);

                // Tex + color
                //fc += (Vec3::new(color[0] as f32, color[1] as f32, color[2] as f32) + self.color)
                //    .div(2.0)
                //    + 0.5;
            } else {
                //Color
                fc += self.color;

                // Bary
                // UV
            }
        }

        fc
    }

    // fn _render_bary(&self, p: Vec2) -> Vec3 {
    //     let v0_p = self.vertices[0].positions;
    //     let v1_p = self.vertices[1].positions;
    //     let v2_p = self.vertices[2].positions;

    //     let mut fc = Vec3::new(0.0, 0.0, 0.0);

    //     // clock wise check
    //     let area0 = render_utils::edge_fun(p, v0_p.xy(), v1_p.xy());
    //     let area1 = render_utils::edge_fun(p, v1_p.xy(), v2_p.xy());
    //     let area2 = render_utils::edge_fun(p, v2_p.xy(), v0_p.xy());

    //     if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 {
    //         fc += render_utils::bary_coord([v0_p, v1_p, v2_p], p);
    //         fc *= Vec3::new(255.0, 255.0, 255.0);
    //     }

    //     fc
    // }

    // fn _render_uv(&self, p: Vec2) -> Vec3 {
    //     let v0_p = self.vertices[0].positions;
    //     let v1_p = self.vertices[1].positions;
    //     let v2_p = self.vertices[2].positions;

    //     let mut fc = Vec3::new(0.0, 0.0, 0.0);

    //     // clock wise check
    //     let area0 = render_utils::edge_fun(p, v0_p.xy(), v1_p.xy());
    //     let area1 = render_utils::edge_fun(p, v1_p.xy(), v2_p.xy());
    //     let area2 = render_utils::edge_fun(p, v2_p.xy(), v0_p.xy());

    //     if area0 <= 0.0 && area1 <= 0.0 && area2 <= 0.0 {
    //         let bary = render_utils::bary_coord([v0_p, v1_p, v2_p], p);

    //         let v0_uv = self.vertices[0].uv.mul(bary.x);
    //         let v1_uv = self.vertices[1].uv.mul(bary.y);
    //         let v2_uv = self.vertices[2].uv.mul(bary.z);

    //         //Uv coords pog
    //         let uv = (v0_uv + v1_uv + v2_uv) * Vec2::new(255.0, 255.0);

    //         fc += Vec3::new(uv.x, uv.y, 0.0);
    //     }

    //     fc
    // }
}
