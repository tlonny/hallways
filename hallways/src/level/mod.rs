use std::collections::HashMap;
use url::Url;

use crate::hallways::audio::Data;
use crate::hallways::gltf::Mesh;
use crate::hallways::gpu::bind_group;
use crate::hallways::gpu::buffer::{self, vertex};
use crate::hallways::util::parry3d::trimesh::Ext;
use parry3d::shape::TriMesh;

use self::manifest::Manifest;
use self::manifest::Material;
use self::portal::LoadParams;

pub mod cache;
mod error;
mod manifest;
mod material;
mod model;
mod portal;
pub mod render;

pub struct Level {
    manifest: Manifest,
    collider: TriMesh,
    vertex_buffer: vertex::Level,
    index_buffer: buffer::Index,
    bind_group: bind_group::Level,
    portals: Vec<portal::Portal>,
    portal_lookup: HashMap<String, usize>,
    track: Option<Data>,
}

pub use cache::Cache;
pub use error::LoadError;
pub use portal::Portal;

use glam::Vec3;
use parry3d::math::{Isometry, Vector};
use parry3d::query::{cast_shapes, ShapeCastHit, ShapeCastOptions};
use parry3d::shape::Shape;

fn build_material_index<'a>(manifest: &'a Manifest, mesh: &Mesh) -> Vec<Option<&'a Material>> {
    let mut mapped: Vec<Option<&Material>> = Vec::with_capacity(mesh.materials().len());

    for material in mesh.materials() {
        let material = manifest.material(material.name());
        mapped.push(material);
    }
    return mapped;
}

fn load_mesh(manifest: &Manifest, mesh_href: &str) -> Result<Mesh, LoadError> {
    let mesh_data = manifest.asset(mesh_href).unwrap();
    return Mesh::decode(mesh_data).map_err(LoadError::Mesh);
}

fn load_track(manifest: &Manifest, track_href: &str) -> Result<Data, LoadError> {
    let track_bytes = manifest.asset(track_href).unwrap();
    return Data::create(track_bytes, true).map_err(LoadError::Track);
}

impl Level {
    pub fn load(url: Url, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self, LoadError> {
        let manifest = Manifest::load(&url).map_err(LoadError::Manifest)?;
        let level_model_mesh = load_mesh(&manifest, &manifest.model)?;
        let material_index = build_material_index(&manifest, &level_model_mesh);

        let material_result = material::load(device, queue, &manifest, &material_index)
            .map_err(LoadError::Material)?;
        let model_result = model::build(device, queue, &level_model_mesh, &material_index)
            .map_err(LoadError::Model)?;

        let collider_href = match manifest.collider.as_deref() {
            Some(collider) => collider,
            None => manifest.model.as_str(),
        };
        let level_collider_mesh = load_mesh(&manifest, collider_href)?;
        let collider = TriMesh::from_gltf_mesh(&level_collider_mesh);

        let mut portals = Vec::new();
        let mut portal_lookup = HashMap::new();
        for (name, manifest_portal) in manifest.portals() {
            let portal_ix = portals.len();
            let portal = Portal::load(LoadParams {
                name: name.clone(),
                portal_ix,
                base_url: &url,
                manifest: &manifest,
                collider_href: &manifest_portal.collider,
                target: manifest_portal.target.as_ref(),
                device,
                queue,
            })
            .map_err(LoadError::Portal)?;
            portal_lookup.insert(name.clone(), portal_ix);
            portals.push(portal);
        }

        let track = match manifest.track.as_deref() {
            Some(track_href) => Some(load_track(&manifest, track_href)?),
            None => None,
        };

        return Ok(Self {
            manifest,
            collider,
            vertex_buffer: model_result.vertex_buffer,
            index_buffer: model_result.index_buffer,
            bind_group: material_result.bind_group,
            portals,
            portal_lookup,
            track,
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

    pub fn track(&self) -> Option<&Data> {
        return self.track.as_ref();
    }

    pub fn portal(&self, name: &str) -> Option<&Portal> {
        let portal_ix = *self.portal_lookup.get(name)?;
        return self.portals.get(portal_ix);
    }

    pub fn portals(&self) -> &[Portal] {
        return &self.portals;
    }

    pub fn spawn_position(&self) -> Vec3 {
        return self.manifest.spawn.unwrap_or(Vec3::ZERO);
    }
}
