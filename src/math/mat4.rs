use std::ops;
use std::f32;
use super::deg2rad;
use super::vec3::Vec3;
use super::quaternion::Quat;

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
impl ops::Mul<&Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: &Mat4) -> Mat4 {
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

impl Mat4 {
    // todo, only if needed later
    pub fn invert(&self) {

    }

    pub fn transpose(&self) ->Mat4 {
        let mut ret = Mat4 {
            mat: [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]]
        };

        for i in 0..4 {
            for j in 0..4 {
                ret[i][j] = self[j][i];
            }
        }

        ret
    }

    // transforms
    pub fn translate(&self, t: Vec3) -> Mat4 {
        let t_mat = Mat4 {
            mat: [
                [1.0, 0.0, 0.0, t.x],
                [0.0, 1.0, 0.0, t.y],
                [0.0, 0.0, 1.0, t.z],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        t_mat * self
    }

    pub fn rotate_x(&self, deg: f32) -> Mat4 {
        let rad = deg2rad(deg);
        let r_mat = Mat4 {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, rad.cos(), -rad.sin(), 0.0],
                [0.0, rad.sin(), rad.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        r_mat * self
    }

    pub fn rotate_y(&self, deg: f32) -> Mat4 {
        let rad = deg2rad(deg);
        let r_mat = Mat4 {
            mat: [
                [rad.cos(), 0.0, rad.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-rad.sin(), 0.0, rad.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        r_mat * self
    }

    pub fn rotate_z(&self, deg: f32) -> Mat4 {
        let rad = deg2rad(deg);
        let r_mat = Mat4 {
            mat: [
                [rad.cos(), -rad.sin(), 0.0, 0.0],
                [rad.sin(), rad.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        r_mat * self
    }

    pub fn scale(&self, s: Vec3) -> Mat4 {
        let s_mat = Mat4 {
            mat: [
                [s.x, 0.0, 0.0, 0.0],
                [0.0, s.y, 0.0, 0.0],
                [0.0, 0.0, s.z, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        s_mat * self
    }

    // todo: shear
    pub fn shear_xy(&self, s: f32) {

    }

    pub fn shear_xz(&self, s: f32) {
        
    }

    pub fn shear_yx(&self, s: f32) {
        
    }

    pub fn shear_yz(&self, s: f32) {
        
    }

    pub fn shear_zx(&self, s: f32) {
        
    }

    pub fn shear_zy(&self, s: f32) {
        
    }

    pub fn euler_transform(&self, h: f32, p: f32, r: f32) -> Mat4 {
        self.rotate_y(h).rotate_x(p).rotate_z(r)
    }

    // openGL orthographic projection matrix: depth mapped from -1 to 1
    pub fn orthographic_proj(&self, top: f32, bottom: f32, left: f32, right: f32, near: f32, far: f32) -> Mat4 {
        let o_mat = Mat4 {
            mat: [
                [2.0/(right-left), 0.0, 0.0, -(right+left)/(right-left)],
                [0.0, 2.0/(top-bottom), 0.0, -(top+bottom)/(top-bottom)],
                [0.0, 0.0, 2.0/(far-near), -(far+near)/(far-near)],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        o_mat * self
    }

    // openGL perspective projection matrix
    pub fn perspective_proj(&self, top: f32, bottom: f32, left: f32, right: f32, near: f32, far: f32) -> Mat4 {
        let p_mat = Mat4 {
            mat: [
                [2.0*near/(right-left), 0.0, (right+left)/(right-left), 0.0],
                [0.0, 2.0*near/(top-bottom), (top+bottom)/(top-bottom), 0.0],
                [0.0, 0.0, -(far+near)/(far-near), -2.0*far*near/(far-near)],
                [0.0, 0.0, -1.0, 0.0]
            ]
        };

        p_mat * self
    }

    // todo: if needed
    pub fn slerp(&self, q: Quat, r: Quat, t: f32) {

    }
}