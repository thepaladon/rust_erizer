use glam::IVec2;
use sliced_buffer::SlicedBuffers;
use vertex_buffer::VertexBuffer;
use viewport::VIEWPORT;

mod material;
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

pub struct TanglRenderer {

    buffer_size : IVec2,
    tile_size : i32,

    //Data
    buffers: SlicedBuffers,
    indices: Vec<i32>,
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
        };

        Self{
            buffer_size: IVec2::new(buff_width as i32, buff_height as i32),
            tile_size: tile_size,
            buffers,
            indices: Vec::new(),
            vertices: Vec::new(),

        }

    }

    pub fn Tangl_GenVertexBuffer(&mut self) -> i32 {
        self.vertices.push(VertexBuffer::new());
        
        //Return the position where the buffer was created
        self.vertices.len() as i32
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
