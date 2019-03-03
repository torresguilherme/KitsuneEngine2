use std::f64::consts::PI;

pub mod vec2;
pub mod vec3;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod quaternion;
pub mod ray2d;
pub mod ray3d;

fn deg2rad(degrees: f32) -> f32 {
    degrees * (PI as f32 / 180.0)
}

fn rad2deg(radians: f32) -> f32 {
    radians * (180.0 / PI as f32)
}