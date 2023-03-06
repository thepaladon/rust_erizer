pub enum RenderMode {
    Default,
    VertexColor,
    Texture,
    TextureColor,
    Normal,
    Uv,
    Bary,
    Depth,
    Aabb,
    Error,
}

impl Default for RenderMode {
    fn default() -> Self {
        Self::Default
    }
}

impl RenderMode {
    pub fn new(mode : i32) -> Self {
        match mode {
            0 => {RenderMode::Default},
            1 => {RenderMode::VertexColor},
            2 => {RenderMode::Texture},
            3 => {RenderMode::TextureColor},
            4 => {RenderMode::Normal},
            5 => {RenderMode::Uv},
            6 => {RenderMode::Bary},
            7 => {RenderMode::Depth},
            8 => {RenderMode::Aabb},
            9 => {RenderMode::Error},
            i32::MIN..=-1_i32 | 10_i32..=i32::MAX => { panic!("Wrong Number Buck-o"); }
        }

    }

    fn next_mode(&self) -> Self {
        use RenderMode::*;
        match *self {
            Default => VertexColor,
            VertexColor => Texture,
            Texture => TextureColor,
            TextureColor => Normal,
            Normal => Uv,
            Uv => Bary,
            Bary => Depth,
            Depth => Aabb,
            Aabb => Error,
            Error => Default,
        }
    }

    fn previous_mode(&self) -> Self {
        use RenderMode::*;
        match *self {
            Default => Error,
            VertexColor => Default,
            Texture => VertexColor,
            TextureColor => Texture,
            Normal => TextureColor,
            Uv => Normal,
            Bary => Uv,
            Depth => Bary,
            Aabb => Depth,
            Error => Aabb,
        }
    }
}