# Angle's Rust_erizer
## Final Screenshot :
Runs with 28ms on my `Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz`
![image](https://github.com/thepaladon/rust_erizer/assets/44022509/12b8b1b7-14ab-445e-b2f9-234c39546bed)

## Overview
A CPU Rasterizer written in Rust between Block B and C during Y2 in BUas. 
More info in this file [info.md](https://github.com/thepaladon/rust_erizer/blob/master/docs/info.md)

This personal project served as my introduction to the Rust programming language. I looked into all of the quality-of-life improvements this language supports over C++ like easy CI/CD, formatting, and warning handling (linting). 

I wrote this project as part of a masterclass held at the university. At the end of the masterclass, there was a competition for the most impressive rasterizer. My renderer was chosen and I won a little 3D-printed Ferris The Crab, Rust's mascot.

Thanks to this and previous school projects I was selected to intern at [Traverse Research](https://traverse.nl/) for their "Summer of Code" internship program. 

<img width="495" alt="image" src="https://github.com/thepaladon/rust_erizer/assets/44022509/77f7eaf2-20ef-47d9-b29d-6740cd48eda9">


## Features
### Models:
+ Basic glTF Model Loading
+ Frustum Culling

### Rendering:
+ Multithreaded Fragment Shader
+ Textures
+ Samplers (ClampToEdge, Repeat, Mirror)
+ Clipping and Backface Culling
+ Normals Rendering
+ Vertex Colors

### Render Modes:
_(Switch render modes with M1 and M2)_
+ Basic Lambertian Albedo shading
+ Vertex Color 
+ Albedo
+ Albedo + Vertex Color
+ Model Vertex Normal
+ Model Texture Coordinates
+ Barycentric View
+ Depth View
+ Aabb View 

## Getting Started
1. Make sure you have Rust installed.
2. Clone the project: `git clone https://github.com/thepaladon/rust_erizer`
3. Inside the project directory run: `cargo run -r`

Note: Running the project in `Release (-r)` is a must since the rendering code runs on the CPU.


## Controls
- W/A/S/D - move around the camera
- R / F - move up/down
- Mouse - Rotate the Camera
  - Note: I've written this in a terrible way which teleports your mouse around, sorry about that 🙏
- Right Click - Next View Mode
- Left Click - Previous View Mode
- Left and Right Bracket ("[" "]") - Change Scene
- B - Load Sponza
- N - Unload Sponza
- M - unlock / lock mouse



