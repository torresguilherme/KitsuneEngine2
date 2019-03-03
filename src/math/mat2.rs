use std::ops;
use std::f32;

#[derive(Debug, PartialEq, Clone)]
pub struct Mat2 {
    pub mat: [[f32; 2]; 2]
}

impl ops::Index<usize> for Mat2 {
    type Output = [f32; 2];

    fn index<'a>(&'a self, index: usize) -> &'a [f32; 2] {
        &self.mat[index]
    }
}

impl ops::IndexMut<usize> for Mat2 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [f32; 2] {
        &mut self.mat[index]
    }
}