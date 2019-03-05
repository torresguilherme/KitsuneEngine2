use crate::math::vec3::Vec3;

pub struct Mesh {
    vertices: Vec<Vec3>,
    indices: Vec<u32>,
    uvs: Vec<Vec3>,
    uv_indices: Vec<u32>,
    normals: Vec<Vec3>,
    normal_indices: Vec<u32>
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