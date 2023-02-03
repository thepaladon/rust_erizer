#[derive(Default, Clone)]
pub struct Sampler {
    pub wrap_s: Wrap, // horizontal a.k.a - u
    pub wrap_t: Wrap, // vertical   a.k.a - v
}

#[derive(Clone)]
pub enum Wrap {
    ClampToEdge,
    Repeat,
    Mirror,
}

impl Default for Wrap {
    fn default() -> Self {
        Wrap::Repeat
    }
}

pub fn clamp_to_edge(uv: f32) -> f32 {
    f32::min(1.0, f32::max(0.0, uv))
}

pub fn repeat(uv: f32) -> f32 {
    uv - uv.floor()
}

pub fn mirror(uv: f32) -> f32 {
    let i = (uv.floor() % 2.0 + 2.0) % 2.0;

    if i == 0.0 {
        uv - uv.floor()
    } else {
        1.0 - (uv - uv.floor())
    }
}
