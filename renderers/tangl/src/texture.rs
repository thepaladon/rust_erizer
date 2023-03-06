use glam::Vec4;
use gltf::image::Data;
use image::open;

use crate::sampler::Sampler;

#[derive(Default, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
    pub sampler: Sampler,
}
//For the rusterizer. Low data images only.

impl Texture {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
            ..Default::default()
        }
    }

    pub fn from_filepath(fp: &str) -> Self {
        let _tex = open(fp).expect("Texture Error: ");
        let width = _tex.width();
        let height = _tex.height();
        let data: Vec<u32> = _tex
            .as_rgb8()
            .expect("Shit's not there >:( ")
            .to_vec()
            .chunks(3)
            .map(|rgb| crate::render_utils::argb8_to_u32(rgb[0], rgb[1], rgb[2], 255))
            .collect();

        Self {
            width,
            height,
            data,
            ..Default::default()
        }
    }

    pub fn from_gltf_image(image: &Data) -> Self {
        let data = match image.format {
            gltf::image::Format::R8 => image
                .pixels
                .iter()
                .map(|r| crate::render_utils::rgba8_to_u32(*r, 0, 0, 0))
                .collect(),
            gltf::image::Format::R8G8 => todo!(),
            gltf::image::Format::R8G8B8 => image
                .pixels
                .chunks(3)
                .map(|rgb| crate::render_utils::rgba8_to_u32(rgb[0], rgb[1], rgb[2], 255))
                .collect(),
            gltf::image::Format::R8G8B8A8 => image
                .pixels
                .chunks(4)
                .map(|rgba| crate::render_utils::rgba8_to_u32(rgba[0], rgba[1], rgba[2], rgba[3]))
                .collect(),
            gltf::image::Format::R16 => todo!(),
            gltf::image::Format::R16G16 => todo!(),
            gltf::image::Format::R16G16B16 => todo!(),
            gltf::image::Format::R16G16B16A16 => todo!(),
            gltf::image::Format::R32G32B32FLOAT => todo!(),
            gltf::image::Format::R32G32B32A32FLOAT => todo!(),
        };

        Texture {
            width: image.width,
            height: image.height,
            data,
            ..Default::default()
        }
    }

    pub fn get_pixel(&self, img_width: u32, img_height: u32) -> Vec4 {
        let color = self.data[self.width as usize * img_height as usize + img_width as usize];
        let values = crate::render_utils::u32_to_argb8(color);

        Vec4::new(
            values[0] as f32,
            values[1] as f32,
            values[2] as f32,
            values[3] as f32,
        )
    }
}
