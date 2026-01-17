pub mod fetch;

use glam::Vec3;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
use url::Url;

use crate::util;

use fetch::fetch;

const MANIFEST_VERSION: &str = "coco";
const MAX_PORTALS: usize = 4;
const ASSET_LOAD_THREADS: usize = 4;

static ASSET_LOAD_POOL: OnceLock<ThreadPool> = OnceLock::new();

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TextureAddressing {
    Linear,
    Nearest,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    pub frames: Option<Vec<String>>,
    pub animation_speed: Option<f32>,
    pub color: Option<util::Color>,
    pub texture_addressing: Option<TextureAddressing>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub href: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Portal {
    pub collider: String,
    pub target: Option<Target>,
}

#[derive(Debug)]
pub struct Manifest {
    pub model: String,
    pub collider: Option<String>,
    pub track: Option<String>,
    pub spawn: Option<Vec3>,
    materials: HashMap<String, Material>,
    portals: HashMap<String, Portal>,
    assets: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestData {
    #[serde(rename = "_version")]
    version: String,
    model: String,
    collider: Option<String>,
    track: Option<String>,
    spawn: Option<Vec3>,
    materials: HashMap<String, Material>,
    portals: HashMap<String, Portal>,
}

#[derive(Debug)]
pub enum LoadError {
    Fetch(fetch::Error),
    UTF8(std::str::Utf8Error),
    Decode(serde_json::Error),
    TooManyPortals,
    InvalidVersion,
}

fn asset_load_pool() -> &'static ThreadPool {
    return ASSET_LOAD_POOL.get_or_init(|| {
        ThreadPoolBuilder::new()
            .num_threads(ASSET_LOAD_THREADS)
            .build()
            .unwrap()
    });
}

impl Manifest {
    pub fn portals(&self) -> impl Iterator<Item = (&String, &Portal)> {
        return self.portals.iter();
    }

    pub fn load(url: &Url) -> Result<Self, LoadError> {
        let data = fetch(url, "").map_err(LoadError::Fetch)?;
        let contents = std::str::from_utf8(&data).map_err(LoadError::UTF8)?;

        let manifest_data: ManifestData =
            serde_json::from_str(contents).map_err(LoadError::Decode)?;

        if manifest_data.version != MANIFEST_VERSION {
            return Err(LoadError::InvalidVersion);
        }

        if manifest_data.portals.len() > MAX_PORTALS {
            return Err(LoadError::TooManyPortals);
        }

        let mut manifest = Manifest {
            model: manifest_data.model,
            collider: manifest_data.collider,
            track: manifest_data.track,
            spawn: manifest_data.spawn,
            materials: manifest_data.materials,
            portals: manifest_data.portals,
            assets: HashMap::new(),
        };

        let mut href_set: HashSet<String> = HashSet::new();
        href_set.insert(manifest.model.clone());
        if let Some(collider_href) = manifest.collider.as_deref() {
            href_set.insert(collider_href.to_string());
        }
        if let Some(track_href) = manifest.track.as_deref() {
            href_set.insert(track_href.to_string());
        }
        for (_, portal) in manifest.portals() {
            href_set.insert(portal.collider.clone());
        }
        for material in manifest.materials() {
            if let Some(frame_hrefs) = material.frames.as_ref() {
                for frame_href in frame_hrefs {
                    href_set.insert(frame_href.clone());
                }
            }
        }

        let asset_pairs: Vec<(String, Vec<u8>)> = asset_load_pool().install(|| {
            return href_set
                .into_par_iter()
                .map(|href| {
                    let data = fetch(url, &href).map_err(LoadError::Fetch)?;
                    return Ok((href, data));
                })
                .collect::<Result<Vec<_>, LoadError>>();
        })?;
        let mut assets: HashMap<String, Vec<u8>> = HashMap::with_capacity(asset_pairs.len());
        for (href, data) in asset_pairs {
            assets.insert(href, data);
        }

        manifest.assets = assets;
        return Ok(manifest);
    }

    pub fn material(&self, name: &str) -> Option<&Material> {
        return self.materials.get(name);
    }

    pub fn materials(&self) -> impl Iterator<Item = &Material> {
        return self.materials.values();
    }

    pub fn asset(&self, href: &str) -> Option<&[u8]> {
        return self.assets.get(href).map(Vec::as_slice);
    }
}
