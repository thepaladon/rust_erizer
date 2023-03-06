use std::sync::Arc;

use glam::{IVec2, Vec2, Mat4, UVec2};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    texture::Texture, triangle::Triangle,
};

pub struct Tile {
    pub pos: IVec2,           //pos of top left pixel of the tile
    pub size: IVec2,          //size of tile in pixels
    pub idx: IVec2,           //index of the tile
    pub depth_data: Vec<f32>, //data to get written to in fragment shader
    pub color_data: Vec<u32>, //data to get written to in fragment shader
    pub tri_idx: Vec<u32>,    //indices from the triangle buffer to render
}

impl Tile {
    fn new(pos: IVec2, size: IVec2, idx: IVec2, buffers_scale : UVec2) -> Self {
        let mut size = size;

        if pos.x + size.x >= buffers_scale.x as i32 {
            size.x = buffers_scale.x as i32 - pos.x;
        }

        if pos.y + size.y >= buffers_scale.y as i32 {
            size.y = buffers_scale.y as i32 - pos.y;
        }

        Self {
            pos,
            size,
            idx,
            color_data: vec![0; (size.x * size.y) as usize],
            depth_data: vec![0.0; (size.x * size.y) as usize],
            tri_idx: Vec::new(),
        }
    }

    fn is_point_in_aabb(&self, point: Vec2) -> bool {
        let (x, y) = (point.x, point.y);

        x >= self.pos.x as f32
            && x < (self.pos.x + self.size.x) as f32
            && y >= self.pos.y as f32
            && y < (self.pos.y + self.size.y) as f32
    }

    pub fn clear_buffers_color(&mut self, val: u32) {
        self.color_data.fill(val);
    }

    pub fn clear_buffers_depth(&mut self, val: f32) {
        self.depth_data.fill(val);
    }

    fn render(
        &mut self,
        tri_buff: &[Triangle],
        camera: &Mat4,
        render_type: &i32,
        texture: Option<&Arc<Texture>>,
        //material: &Material,
    ) {
        for i in self.tri_idx.iter() {
            tri_buff[*i as usize].render_fragments(
                self.pos,
                self.size,
                self.color_data.as_mut_slice(),
                self.depth_data.as_mut_slice(),
                texture,
                material,
                render_type,
            )
            //This is most of the data, now we just draw.
        }
    }
}

pub struct SlicedBuffers {
    pub tiles: Vec<Tile>,
    pub triangles: Vec<Triangle>, //in screen space, to do AABB to check which tile should draw which triangle
    pub amount_of_tiles: IVec2,
    pub size_of_tiles: i32,

    pub buffers_scale : UVec2
}

impl SlicedBuffers {

    //pub fn render(&mut self, camera: &Camera, render_type: &RenderMode) {
    //    //Remove the "par_" from the line below to check performance single threaded
    //    self.tiles.par_iter_mut().enumerate().for_each(|tile| {
    //        tile.1.render(&self.triangles, camera, render_type, None);
    //    });
    //}

    pub fn extern_render(
        &mut self,
        triangles: &[Triangle],
        camera: &Mat4,
        render_type: &i32,
        texture: Option<&Arc<Texture>>,
        material: &Material,
    ) {
        //Remove the "par_" from the line below to check performance single threaded
        self.tiles.par_iter_mut().enumerate().for_each(|tile| {
            tile.1
                .render(triangles, camera, render_type, texture, material);
        });
    }

    pub fn from_buffers(buff_width : u32, buff_height : u32, size_of_tile: i32) -> Self {
        let mut sliced_buff = Self {
            tiles: Vec::new(),
            triangles: Vec::new(),
            amount_of_tiles: IVec2::splat(0),
            size_of_tiles: size_of_tile,
            buffers_scale: UVec2::new(buff_width, buff_height),
        };

        let amount_tiles: IVec2 = IVec2::new(
            f32::ceil(buff_width as f32 / size_of_tile as f32) as i32,
            f32::ceil(buff_height as f32 / size_of_tile as f32) as i32,
        );
        sliced_buff.amount_of_tiles = amount_tiles;

        sliced_buff
            .tiles
            .reserve((amount_tiles.x * amount_tiles.y) as usize);

        for y in 0..amount_tiles.y {
            for x in 0..amount_tiles.x {
                let new_tile = Tile::new(
                    IVec2::new(x * size_of_tile, y * size_of_tile),
                    IVec2::splat(size_of_tile),
                    IVec2::new(x, y),
                    UVec2::new(buff_width, buff_height)
                );
                sliced_buff.tiles.push(new_tile);
            }
        }

        let bp = sliced_buff.tiles.len();
        sliced_buff
    }

    //We have all triangles as a buffer,
    //Now we want to distribute the indexes of them to the proper cell
    pub fn aa_bb_comparison(&mut self) {
        //Performance sinkhole
        for i in 0..self.triangles.len() {
            let tri = self.triangles[i];

            let aabb = tri.aabb.unwrap();

            //Gives me the idx of the tile in which it is
            let min = Vec2::floor(aabb[0]) / self.size_of_tiles as f32;
            let max = Vec2::floor(aabb[1]) / self.size_of_tiles as f32;

            for x in min.x as i32..=max.x as i32 {
                for y in min.y as i32..=max.y as i32 {
                    //let idx = (y + x * self.size.y) as usize;
                    let idx = (x + y * self.amount_of_tiles.x) as usize;
                    self.tiles[idx].tri_idx.push(i as u32);
                }
            }
        }
    }

    pub fn external_aa_bb_comparison(&mut self, triangles: &mut [Triangle]) {
        //WIP
        // triangles.par_iter_mut().enumerate().for_each(|(i , tri)|{
        //     let aabb = tri.aabb.unwrap();

        //     //Gives me the idx of the tile in which it is
        //     let min = Vec2::floor(aabb[0]) / self.size_of_tiles as f32;
        //     let max = Vec2::floor(aabb[1]) / self.size_of_tiles as f32;

        //     for x in min.x as i32..=max.x as i32 {
        //         for y in min.y as i32..=max.y as i32 {
        //             //let idx = (y + x * self.size.y) as usize;
        //             let idx = (x + y * self.amount_of_tiles.x) as usize;
        //             self.tiles[idx].tri_idx.push(i as u32);
        //         }
        //     }
        // })

        for (i, tri) in triangles.iter().enumerate() {
            let aabb = tri.aabb.unwrap();

            //Gives me the idx of the tile in which it is
            let min = Vec2::floor(aabb[0]) / self.size_of_tiles as f32;
            let max = Vec2::floor(aabb[1]) / self.size_of_tiles as f32;

            for x in min.x as i32..=max.x as i32 {
                for y in min.y as i32..=max.y as i32 {
                    //let idx = (y + x * self.size.y) as usize;
                    let idx = (x + y * self.amount_of_tiles.x) as usize;
                    self.tiles[idx].tri_idx.push(i as u32);
                }
            }
        }
    }

    pub fn clear_depth(&mut self, val: f32) {
        for tile in self.tiles.iter_mut() {
            tile.clear_buffers_depth(val);
        }
    }

    pub fn clear_color(&mut self, val: u32) {
        for tile in self.tiles.iter_mut() {
            tile.clear_buffers_color(val);
        }
    }

    pub fn clear_tiles(&mut self) {
        self.triangles.clear();
        for tile in self.tiles.iter_mut() {
            tile.tri_idx.clear();
        }
    }

    pub fn transfer_buffer(&self) -> Vec<u32> {
        let mut output = vec![0; self.buffers_scale.x as usize * self.buffers_scale.y as usize ];

        for x in 0..self.amount_of_tiles.x {
            for y in 0..self.amount_of_tiles.y {
                let tile = &self.tiles[(x + y * self.amount_of_tiles.x) as usize];
                for t_x in 0..tile.size.x {
                    for t_y in 0..tile.size.y {
                        let src = (t_x + t_y * tile.size.x) as usize;
                        let dst = (x * self.size_of_tiles
                            + t_x
                            + (y * self.size_of_tiles + t_y) * self.buffers_scale.x as i32)
                            as usize;

                        output[dst] = tile.color_data[src];
                    }
                }
            }
        }

        output
    }
}
