use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::texture::Texture;
use gltf::image::Data;

pub struct TextureManager {
    textures: HashMap<i32, Arc<Texture>>,
    num_to_assign: i32, //This always goes up, never down.
}

impl TextureManager {
    fn new() -> Self {
        Self {
            textures: HashMap::new(),
            num_to_assign: 0,
        }
    }

    //Returns index of this
    pub fn load_from_filepath(&mut self, fp: &str) -> Result<i32, String> {
        //error "handling" is done in Texture here :p / dunno whether that's the best approach
        let texture = Arc::new(Texture::from_filepath(fp));

        self.num_to_assign += 1;
        self.textures.insert(self.num_to_assign, texture);
        Ok(self.num_to_assign)
    }

    pub fn load_from_gltf_images(&mut self, images: Vec<Data>) -> Result<Vec<i32>, String> {
        if images.is_empty() {
            return Err("Images is empty".to_string());
        }

        //Start of the array of image data from GLTF Image.
        let mut tex_indices = Vec::new();

        for image in images {
            let texture = Arc::new(Texture::from_gltf_image(&image));

            self.num_to_assign += 1;
            tex_indices.push(self.num_to_assign);
            self.textures.insert(self.num_to_assign, texture);
        }

        Ok(tex_indices)
    }

    pub fn destroy_texture(&mut self, idx: &i32) {
        self.textures.remove(idx);
    }

    pub fn get_texture(&self, idx: &i32) -> Option<&Arc<Texture>> {
        self.textures.get(idx)
    }
}

// Global variable that holds the TextureManager
lazy_static::lazy_static! {
    pub static ref TEXTURE_MANAGER: RwLock<TextureManager> = RwLock::new(TextureManager::new());

}
