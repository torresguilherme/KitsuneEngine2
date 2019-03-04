use std::ops;
use std::f32;

#[derive(Debug, PartialEq, Clone)]
pub struct Mat2 {
    pub mat: [[f32; 2]; 2]
}

// indices
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

// mul ops
impl ops::Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, other: Mat2) -> Mat2 {
        let mut ret = Mat2 {
            mat: [[0.0, 0.0], [0.0, 0.0]]
        };
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    ret[i][j] += self[i][k] * other[k][j]
                }
            }
        }

        ret
    }
}