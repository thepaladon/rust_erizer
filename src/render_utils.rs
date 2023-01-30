use glam::{Vec2, Vec3, Vec3Swizzles};

pub fn edge_fun(p: Vec2, v0: Vec2, v1: Vec2) -> f32 {
    let v0_p = p - v0;
    let v0_v1 = v1 - v0;

    v0_p.x * v0_v1.y - v0_p.y * v0_v1.x
}

//Barycentric coordinates
pub fn bary_coord(vertices: [Vec3; 3], p: Vec2, total_area: f32) -> Vec3 {
    let area0 = edge_fun(p, vertices[1].xy(), vertices[2].xy())
        / edge_fun(vertices[2].xy(), vertices[0].xy(), vertices[1].xy());

    let area1 = edge_fun(p, vertices[2].xy(), vertices[0].xy())
        / edge_fun(vertices[2].xy(), vertices[0].xy(), vertices[1].xy());

    let area2 = 1.0 - area0 - area1;

    Vec3::new(area0, area1, area2)
}

pub fn better_bary(calc_area: [f32; 2], p: Vec2, total_area: f32) -> Vec3 {
    let rec = 1.0 / total_area;
    let area0 = calc_area[0]  //1, 2
    * rec;

    let area1 = calc_area[1] // 2, 0
    * rec;
    let area2 = 1.0 - area0 - area1;

    Vec3::new(area0, area1, area2)
}

pub fn argb8_to_u32(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

pub fn u32_to_argb8(pix: u32) -> [u8; 4] {
    let mut argb: [u8; 4] = [0, 0, 0, 0];

    argb[3] = (pix & 0xff) as u8;
    argb[2] = ((pix >> 8) & 0xff) as u8;
    argb[1] = ((pix >> 16) & 0xff) as u8;
    argb[0] = ((pix >> 24) & 0xff) as u8;

    argb
}

// Map to range
pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    b1 + (v - a1) * (b2 - b1) / (a2 - a1)
}
