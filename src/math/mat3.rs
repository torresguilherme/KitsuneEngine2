use std::ops;
use std::f32;

#[derive(Debug, PartialEq, Clone)]
pub struct Mat3 {
    pub mat: [[f32; 3]; 3]
}

impl ops::Index<usize> for Mat3 {
    type Output = [f32; 3];

    fn index<'a>(&'a self, index: usize) -> &'a [f32; 3] {
        &self.mat[index]
    }
}

impl ops::IndexMut<usize> for Mat3 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [f32; 3] {
        &mut self.mat[index]
    }
}