use glam::{Mat4, Vec2, Vec3};

use super::{Error, Mesh, Vertex};
use crate::util;

pub fn walk_node(
    node: &::gltf::Node,
    buffers: &[::gltf::buffer::Data],
    parent_transform: Mat4,
    mesh: &mut Mesh,
) -> Result<(), Error> {
    let local = Mat4::from_cols_array_2d(&node.transform().matrix());
    let global = parent_transform * local;

    if let Some(node_mesh) = node.mesh() {
        for primitive in node_mesh.primitives() {
            let material_ix = primitive.material().index().map(|i| i as u32);

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let vertex_offset = mesh.vertices.len() as u32;
            let positions: Vec<Vec3> = match reader.read_positions() {
                Some(pos_iter) => pos_iter
                    .map(|pos| global.transform_point3(Vec3::from_array(pos)))
                    .collect(),
                None => continue,
            };

            let diffuse_uvs = reader.read_tex_coords(0).map(|tex_iter| {
                tex_iter
                    .into_f32()
                    .map(Vec2::from_array)
                    .collect::<Vec<_>>()
            });

            let colors = reader.read_colors(0).map(|color_iter| {
                color_iter
                    .into_rgba_f32()
                    .map(util::Color::from)
                    .collect::<Vec<_>>()
            });

            for index in 0..positions.len() {
                mesh.vertices.push(Vertex {
                    position: positions[index],
                    diffuse_uv: diffuse_uvs.as_ref().map(|uvs| uvs[index]),
                    material_ix,
                    color: colors.as_ref().map(|colors| colors[index]),
                });
            }

            if let Some(idx_iter) = reader.read_indices() {
                for idx in idx_iter.into_u32() {
                    mesh.indices.push(idx + vertex_offset);
                }
            } else {
                return Err(Error::MeshNotIndexed);
            }
        }
    }

    for child in node.children() {
        walk_node(&child, buffers, global, mesh)?;
    }

    return Ok(());
}
