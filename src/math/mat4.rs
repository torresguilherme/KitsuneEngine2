use std::ops;
use std::f32;

#[derive(Debug, PartialEq, Clone)]
pub struct Mat4 {
    pub mat: [[f32; 4]; 4]
}

impl ops::Index<usize> for Mat4 {
    type Output = [f32; 4];

    fn index<'a>(&'a self, index: usize) -> &'a [f32; 4] {
        &self.mat[index]
    }
}

impl ops::IndexMut<usize> for Mat4 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [f32; 4] {
        &mut self.mat[index]
    }
}

// mul ops
impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut ret = Mat4 {
            mat: [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]]
        };
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    ret[i][j] += self[i][k] * other[k][j]
                }
            }
        }

        ret
    }
}