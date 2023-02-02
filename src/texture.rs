use glam::Vec3;
use image::open;

#[derive(Default, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
}

impl Texture {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }

    pub fn from_filepath(fp: &str) -> Self {
        let _tex = open("resources/textures/bojan.jpg").expect("Texture Error: ");
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
        }
    }

    pub fn get_pixel(&self, img_width: u32, img_height: u32) -> Vec3 {
        let color = self.data[self.width as usize * img_height as usize + img_width as usize];
        let values = crate::render_utils::u32_to_argb8(color);

        Vec3::new(values[0] as f32, values[1] as f32, values[2] as f32)
    }
}
