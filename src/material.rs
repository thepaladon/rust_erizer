use glam::Vec4;


#[derive(Copy, Clone)]
pub struct Material {
    pub base_tex_idx: i32,
    pub base_color: Vec4,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            base_tex_idx: -1,
            base_color: Vec4::splat(0.0),
        }
    }
}