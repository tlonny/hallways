mod geometry;
mod link;

pub use geometry::Geometry;
pub use link::Link;

use parry3d::math::{Isometry, Vector};
use parry3d::query::{cast_shapes, ShapeCastHit, ShapeCastOptions};
use parry3d::shape::{Shape, TriMesh};
use url::Url;

use crate::hallways::gltf::{Error, Mesh};
use crate::hallways::gpu::buffer;
use crate::hallways::gpu::buffer::vertex::{self, portal};
use crate::hallways::level::cache::CacheEntry;
use crate::hallways::level::Cache;
use crate::hallways::util::parry3d::trimesh::Ext;

use self::geometry::DecodeError;
use super::manifest::Manifest;

#[derive(Debug)]
pub enum LoadError {
    UrlJoin(url::ParseError),
    Gltf(Error),
    Geometry(DecodeError),
    Write(portal::WriteError),
}

pub struct LoadParams<'a> {
    pub name: String,
    pub portal_ix: usize,
    pub base_url: &'a Url,
    pub manifest: &'a Manifest,
    pub collider_href: &'a str,
    pub target: Option<&'a super::manifest::Target>,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
}

pub struct Target {
    pub url: Url,
    pub name: String,
}

pub struct Portal {
    pub name: String,
    pub index: usize,
    pub geometry: Geometry,
    pub vertex_buffer: vertex::Portal,
    pub index_buffer: buffer::Index,
    collider: TriMesh,
    pub target: Option<Target>,
}

impl Portal {
    pub fn load(params: LoadParams<'_>) -> Result<Self, LoadError> {
        let collider_data = params.manifest.asset(params.collider_href).unwrap();
        let portal_mesh = Mesh::decode(collider_data).map_err(LoadError::Gltf)?;

        let target = match params.target {
            Some(target) => {
                let target_url = params
                    .base_url
                    .join(&target.href)
                    .map_err(LoadError::UrlJoin)?;
                Some(Target {
                    url: target_url,
                    name: target.name.clone(),
                })
            }
            None => None,
        };
        let geometry = Geometry::decode(portal_mesh.vertices(), portal_mesh.indices())
            .map_err(LoadError::Geometry)?;

        let render_buffer: Vec<_> = portal_mesh
            .vertices()
            .iter()
            .map(|vertex| portal::Data {
                position: vertex.position,
            })
            .collect();
        let mut vertex_buffer = vertex::Portal::create(params.device, render_buffer.len());
        vertex_buffer
            .write(params.queue, &render_buffer)
            .map_err(LoadError::Write)?;

        let mut index_buffer = buffer::Index::create(params.device, portal_mesh.indices().len());
        index_buffer.upload(params.queue, portal_mesh.indices());

        let portal_collider = TriMesh::from_gltf_mesh(&portal_mesh);

        return Ok(Self {
            name: params.name,
            index: params.portal_ix,
            geometry,
            vertex_buffer,
            index_buffer,
            collider: portal_collider,
            target,
        });
    }

    pub fn sweep(
        &self,
        pos: &Isometry<f32>,
        vel: &Vector<f32>,
        shape: &dyn Shape,
        max_toi: f32,
    ) -> Option<ShapeCastHit> {
        return cast_shapes(
            pos,
            vel,
            shape,
            &Isometry::identity(),
            &Vector::zeros(),
            &self.collider,
            ShapeCastOptions::with_max_time_of_impact(max_toi),
        )
        .unwrap();
    }

    pub fn link(&self, cache: &mut Cache) -> Option<Link> {
        let target = self.target.as_ref()?;

        let Some(CacheEntry::Ready(level)) = cache.get(&target.url) else {
            return None;
        };
        let dst_portal = level.portal(&target.name)?;
        if !self.geometry.matches(&dst_portal.geometry) {
            return None;
        }

        return Some(Link {
            portal_ix: dst_portal.index,
            src: self.geometry.clone(),
            dst: dst_portal.geometry.clone(),
        });
    }
}
