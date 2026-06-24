use parry3d::math::Point;
use parry3d::shape::TriMesh;

use crate::hallways::gltf::Mesh;

pub trait Ext {
    fn from_gltf_mesh(mesh: &Mesh) -> TriMesh;
}

impl Ext for TriMesh {
    fn from_gltf_mesh(mesh: &Mesh) -> TriMesh {
        let vertices = mesh
            .vertices()
            .iter()
            .map(|vertex| Point::new(vertex.position.x, vertex.position.y, vertex.position.z))
            .collect();
        let indices = mesh
            .indices()
            .chunks(3)
            .map(|triangle| [triangle[0], triangle[1], triangle[2]])
            .collect();
        return TriMesh::new(vertices, indices);
    }
}
