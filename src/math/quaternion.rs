use std::ops;
use std::f32;
use super::deg2rad;

#[derive(Debug, Clone, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}