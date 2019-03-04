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
    fn invert(&self) {

    }

    fn transpose(&self) {

    }

    // transforms
    fn translate(&self, t: Vec3) -> Mat4 {
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

    fn rotate_x(&self, deg: f32) -> Mat4 {
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

    fn rotate_y(&self, deg: f32) -> Mat4 {
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

    fn rotate_z(&self, deg: f32) -> Mat4 {
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

    fn scale(&self, s: Vec3) -> Mat4 {
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
    fn shear_xy(&self, s: f32) {

    }

    fn shear_xz(&self, s: f32) {
        
    }

    fn shear_yx(&self, s: f32) {
        
    }

    fn shear_yz(&self, s: f32) {
        
    }

    fn shear_zx(&self, s: f32) {
        
    }

    fn shear_zy(&self, s: f32) {
        
    }

    fn euler_transform(&self, h: f32, p: f32, r: f32) -> Mat4 {
        self.rotate_y(h).rotate_x(p).rotate_z(r)
    }

    fn orthographic_proj(&self, s: f32) {

    }

    fn perspective_proj(&self, s: f32) {

    }

    fn slerp(&self, q: Quat, r: Quat, t: f32) {

    }
}