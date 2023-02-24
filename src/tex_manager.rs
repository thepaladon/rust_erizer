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
            let mut data: Vec<u32> = Vec::new();

            match image.format {
                gltf::image::Format::R8 => {
                    dbg!("Todo, but removed cuz it crashed lol");
                }
                gltf::image::Format::R8G8 => todo!(),
                gltf::image::Format::R8G8B8 => {
                    data = image
                        .pixels
                        .chunks(3)
                        .map(|rgb| crate::render_utils::rgba8_to_u32(rgb[0], rgb[1], rgb[2], 255))
                        .collect()
                }
                gltf::image::Format::R8G8B8A8 => {
                    data = image
                        .pixels
                        .chunks(4)
                        .map(|rgba| {
                            crate::render_utils::rgba8_to_u32(rgba[0], rgba[1], rgba[2], rgba[3])
                        })
                        .collect()
                }
                gltf::image::Format::R16 => todo!(),
                gltf::image::Format::R16G16 => todo!(),
                gltf::image::Format::R16G16B16 => todo!(),
                gltf::image::Format::R16G16B16A16 => todo!(),
                gltf::image::Format::R32G32B32FLOAT => todo!(),
                gltf::image::Format::R32G32B32A32FLOAT => todo!(),
            }

            let texture = Arc::new(Texture {
                width: image.width,
                height: image.height,
                data,
                ..Default::default()
            });

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
