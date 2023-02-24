use glam::{Mat4, Vec2, Vec3, Vec3Swizzles, Vec4};

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

pub fn lerp<T>(start: T, end: T, alpha: f32) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    start + (end - start) * alpha
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

pub fn barycentric_coordinates(point: Vec2, v0: Vec2, v1: Vec2, v2: Vec2, area: f32) -> Vec3 {
    let m0 = edge_fun(point, v1, v2);
    let m1 = edge_fun(point, v2, v0);
    let m2 = edge_fun(point, v0, v1);
    // instead of 3 divisions we can do 1/area *
    let a = 1.0 / area;
    glam::vec3(m0 * a, m1 * a, m2 * a)
}

pub fn minor(m: Mat4, r0: usize, r1: usize, r2: usize, c0: usize, c1: usize, c2: usize) -> f32 {
    let m = m.to_cols_array();

    m[4 * r0 + c0] * (m[4 * r1 + c1] * m[4 * r2 + c2] - m[4 * r2 + c1] * m[4 * r1 + c2])
        - m[4 * r0 + c1] * (m[4 * r1 + c0] * m[4 * r2 + c2] - m[4 * r2 + c0] * m[4 * r1 + c2])
        + m[4 * r0 + c2] * (m[4 * r1 + c0] * m[4 * r2 + c1] - m[4 * r2 + c0] * m[4 * r1 + c1])
}

pub fn cofactor(src: Mat4) -> Mat4 {
    let mut dst = [0.0; 16];

    dst[0] = minor(src, 1, 2, 3, 1, 2, 3);
    dst[1] = -minor(src, 1, 2, 3, 0, 2, 3);
    dst[2] = minor(src, 1, 2, 3, 0, 1, 3);
    dst[3] = -minor(src, 1, 2, 3, 0, 1, 2);
    dst[4] = -minor(src, 0, 2, 3, 1, 2, 3);
    dst[5] = minor(src, 0, 2, 3, 0, 2, 3);
    dst[6] = -minor(src, 0, 2, 3, 0, 1, 3);
    dst[7] = minor(src, 0, 2, 3, 0, 1, 2);
    dst[8] = minor(src, 0, 1, 3, 1, 2, 3);
    dst[9] = -minor(src, 0, 1, 3, 0, 2, 3);
    dst[10] = minor(src, 0, 1, 3, 0, 1, 3);
    dst[11] = -minor(src, 0, 1, 3, 0, 1, 2);
    dst[12] = -minor(src, 0, 1, 2, 1, 2, 3);
    dst[13] = minor(src, 0, 1, 2, 0, 2, 3);
    dst[14] = -minor(src, 0, 1, 2, 0, 1, 3);
    dst[15] = minor(src, 0, 1, 2, 0, 1, 2);

    Mat4::from_cols_array(&dst)
}

pub fn vec3_to_u32(val: Vec3) -> u32 {
    let a = 255_u8;
    let r = val.x as u8;
    let g = val.y as u8;
    let b = val.z as u8;

    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

pub fn vec4_to_u32(val: Vec4) -> u32 {
    let a = val.w as u8;
    let r = val.x as u8;
    let g = val.y as u8;
    let b = val.z as u8;

    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

pub fn argb8_to_u32(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

pub fn rgba8_to_u32(a: u8, r: u8, g: u8, b: u8) -> u32 {
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
