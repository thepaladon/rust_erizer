use std::sync::RwLock;


pub struct Viewport {
    pub width : u32,
    pub height : u32,
}

impl Viewport{
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}


// Global variable that holds the TextureManager
lazy_static::lazy_static! {
    pub static ref VIEWPORT: RwLock<Viewport> = RwLock::new(Viewport::new());

}
