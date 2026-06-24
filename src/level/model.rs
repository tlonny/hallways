use glam::Vec2;

use crate::gltf::Mesh;
use crate::gpu::buffer;
use crate::gpu::buffer::vertex::{self, level};
use crate::util;

use super::manifest::Material;

#[derive(Debug)]
pub enum BuildError {
    MaterialIXMissing,
    MaterialConfigMissing,
    Write(level::WriteError),
}

pub struct BuildResult {
    pub vertex_buffer: vertex::Level,
    pub index_buffer: buffer::Index,
}

pub fn build(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    mesh: &Mesh,
    materials: &[Option<&Material>],
) -> Result<BuildResult, BuildError> {
    let mut vertices: Vec<level::Data> = Vec::with_capacity(mesh.vertices().len());
    for vertex in mesh.vertices() {
        let material_ix = match vertex.material_ix {
            Some(material_ix) => material_ix,
            None => return Err(BuildError::MaterialIXMissing),
        };
        if !matches!(materials.get(material_ix as usize), Some(Some(_))) {
            return Err(BuildError::MaterialConfigMissing);
        }
        vertices.push(level::Data {
            position: vertex.position,
            diffuse_uv: vertex.diffuse_uv.unwrap_or(Vec2::ZERO),
            material_ix,
            color: vertex.color.unwrap_or(util::color::WHITE),
        });
    }

    let mut vertex_buffer = vertex::Level::create(device, vertices.len());
    vertex_buffer
        .write(queue, &vertices)
        .map_err(BuildError::Write)?;

    let mut index_buffer = buffer::Index::create(device, mesh.indices().len());
    index_buffer.upload(queue, mesh.indices());

    return Ok(BuildResult {
        vertex_buffer,
        index_buffer,
    });
}
