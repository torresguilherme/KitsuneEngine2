use crate::math::vec3::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub uvs: Vec<Vec3>,
    pub uv_indices: Vec<u32>,
    pub normals: Vec<Vec3>,
    pub normal_indices: Vec<u32>
}

impl Mesh {
    pub fn new(new_vertices: Vec<Vec3>, new_indices: Vec<u32>, new_uvs: Vec<Vec3>, new_uv_indices: Vec<u32>, new_normals: Vec<Vec3>, new_normal_indices: Vec<u32>) -> Mesh {
        Mesh {
            vertices: new_vertices,
            indices: new_indices,
            uvs: new_uvs,
            uv_indices: new_uv_indices,
            normals: new_normals,
            normal_indices: new_normal_indices
        }
    }
}