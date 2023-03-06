use glam::IVec2;
use gltf::{image::Data};
use index_buffer::IndexBuffer;
use sliced_buffer::SlicedBuffers;
use tex_manager::TEXTURE_MANAGER;
use vertex_buffer::VertexBuffer;
use viewport::VIEWPORT;

mod render_utils;
mod sampler;
mod sliced_buffer;
mod tex_manager;
mod texture;
mod triangle;
mod vertex;
mod render_mode;
mod viewport;
mod vertex_buffer;
mod index_buffer;

pub struct TanglRenderer {

    buffer_size : IVec2,
    tile_size : i32,

    //Data
    buffers: SlicedBuffers,
    indices: Vec<IndexBuffer>,
    vertices: Vec<VertexBuffer>,
}

impl TanglRenderer {

    pub fn Tangl_Init(buff_width : u32, buff_height : u32, tile_size : i32) -> Self {

        let buffers = SlicedBuffers::from_buffers(buff_width, buff_height, tile_size);

        // TANGL - Needs to be abstracted better
        let buff_size = {
            let mut manager = VIEWPORT.write().unwrap();
            manager.width = buff_width;
            manager.height = buff_height;
            [manager.width, manager.height]
        };

        Self{
            buffer_size: IVec2::new(buff_width as i32, buff_height as i32),
            tile_size: tile_size,
            buffers,
            indices: Vec::new(),
            vertices: Vec::new(),

        }

    }

    pub fn Tangl_GenVertexBuffer(&mut self, data_ptr: *const f32, data_len: usize, stride : &[i32]) -> i32 {
        self.vertices.push(VertexBuffer::from_pointer(data_ptr, data_len, stride));
        
        //Return the position where the buffer was created
        self.vertices.len() as i32
    }

    pub fn Tangl_GenIndexBuffer(&mut self, data_ptr: *const i32, data_len: usize) -> i32 {
        self.indices.push(IndexBuffer::from_pointer(data_ptr, data_len));
        
        //Return the position where the buffer was created
        self.indices.len() as i32
    }

    pub fn Tangl_GenTextureFilepath(fp: &str) -> i32 {
        let mut manager = TEXTURE_MANAGER.write().unwrap();
        
        manager
            .load_from_filepath(fp)
            .expect("Not found")
    }

    pub fn Tangl_GenTextureImage(image : Vec<Data>) -> Vec<i32> {
        let mut manager = TEXTURE_MANAGER.write().unwrap();
        
        manager
            .load_from_gltf_images(image)
            .expect("Not found")
    }

    pub fn Tangl_DestroyTexture(index : &i32) {
        let mut manager = TEXTURE_MANAGER.write().unwrap();
        
        manager.destroy_texture(index);
    }

    //glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0, GL_RGB, GL_UNSIGNED_BYTE, data);
    //Figure out how to generate images with raw data.

    pub fn Tangl_ClearColor(&mut self, val: u32){
        self.buffers.clear_color(val);
    }

    pub fn Tangl_ClearDepth(&mut self, val: f32){
        self.buffers.clear_depth(val);
    }

    pub fn Tangl_SendForDisplay(&self) ->  Vec<u32> {
        self.buffers.transfer_buffer()
    }

}

//let mut buffer: Vec<u32> = vec![0; BUFF_WIDTH * BUFF_HEIGHT];
//let depth_buffer: Vec<f32> = vec![f32::INFINITY; BUFF_WIDTH * BUFF_HEIGHT];

pub fn add(left: usize, right: usize) -> usize {
    left + right
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
