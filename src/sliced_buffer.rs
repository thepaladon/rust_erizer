use glam::{IVec2, Vec2, Vec3};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{camera::Camera, mesh::RenderMode, triangle::Triangle};

pub struct Tile {
    pub pos: IVec2,           //pos of top left pixel of the tile
    pub size: IVec2,          //size of tile in pixels
    pub idx: IVec2,           //index of the tile
    pub depth_data: Vec<f32>, //data to get written to in fragment shader
    pub color_data: Vec<u32>, //data to get written to in fragment shader
    pub tri_idx: Vec<u32>,    //indices from the triangle buffer to render
}

impl Tile {
    fn new(pos: IVec2, size: IVec2, idx: IVec2) -> Self {
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

    fn render(&mut self, tri_buff: &[Triangle], camera: &Camera, render_type: &RenderMode) {
        for i in self.tri_idx.iter() {
            tri_buff[*i as usize].render_fragments(
                self.pos,
                self.size,
                self.color_data.as_mut_slice(),
                self.depth_data.as_mut_slice(),
                None,
                &Vec3::splat(255.0),
                render_type,
            )
            //This is most of the data, now we just draw.
        }
    }
}

pub struct SlicedBuffers {
    pub tiles: Vec<Tile>,
    pub triangles: Vec<Triangle>, //in screen space, to do AABB to check which tile should draw which triangle
    pub size: IVec2,
}

impl SlicedBuffers {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            triangles: Vec::new(),
            size: IVec2::splat(0),
        }
    }

    pub fn render(&mut self, camera: &Camera, render_type: &RenderMode) {
        self.tiles.par_iter_mut().enumerate().for_each(|tile| {
            tile.1.render(&self.triangles, camera, render_type);
        });
    }

    pub fn from_buffers(color: &[u32], depth: &[f32], size_of_tile: i32) -> Self {
        let mut sliced_buff = Self {
            tiles: Vec::new(),
            triangles: Vec::new(),
            size: IVec2::splat(0),
        };

        assert!(
            crate::WIDTH as i32 % size_of_tile == 0,
            "Width Doesn't fit perfectly in size {size_of_tile}"
        );
        assert!(
            crate::HEIGHT as i32 % size_of_tile == 0,
            "Height Doesn't fit perfectly in size {size_of_tile}"
        );

        let amount_tiles: IVec2 = IVec2::new(
            crate::WIDTH as i32 / size_of_tile,
            crate::HEIGHT as i32 / size_of_tile,
        );
        sliced_buff.size = amount_tiles;

        for y in 0..amount_tiles.y {
            for x in 0..amount_tiles.x {
                let new_tile = Tile::new(
                    IVec2::new(x * size_of_tile, y * size_of_tile),
                    IVec2::splat(size_of_tile),
                    IVec2::new(x, y),
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
            let min = Vec2::floor(aabb[0]) / 4.0;
            let max = Vec2::floor(aabb[1]) / 4.0;

            for x in min.x as i32..=max.x as i32 {
                for y in min.y as i32..=max.y as i32 {
                    //let idx = (y + x * self.size.y) as usize;
                    let idx = (x + y * self.size.x) as usize;
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
        self.triangles.clear();
        for tile in self.tiles.iter_mut() {
            tile.clear_buffers_color(val);
            tile.tri_idx.clear();
        }
    }

    pub fn transfer_buffer(&self) -> Vec<u32> {
        let mut output = vec![0; crate::WIDTH * crate::HEIGHT];

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let tile = &self.tiles[(x + y * self.size.x) as usize];
                for t_x in 0..tile.size.x {
                    for t_y in 0..tile.size.y {
                        let src = (t_x + t_y * tile.size.x) as usize;
                        let dst =
                            (x * tile.size.x + t_x + (y * tile.size.y + t_y) * crate::WIDTH as i32)
                                as usize;

                        output[dst] = tile.color_data[src];
                    }
                }
            }
        }

        output
    }
}
