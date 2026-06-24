use std::collections::HashMap;

use crate::hallways::gpu::bind_group;
use crate::hallways::gpu::bind_group::TEXTURE_BUCKETS;
use crate::hallways::gpu::buffer::storage::{self, material_index};
use crate::hallways::gpu::texture::Array;
use crate::hallways::util;

use super::manifest::{Manifest, Material, TextureAddressing};

const DEFAULT_ANIMATION_SPEED: f32 = 0.5;
const DEFAULT_TEXTURE_ADDRESSING: TextureAddressing = TextureAddressing::Linear;
const TEXTURE_ADDRESSING_LINEAR: u32 = 0;
const TEXTURE_ADDRESSING_NEAREST: u32 = 1;

pub struct LoadResult {
    pub bind_group: bind_group::Level,
}

#[derive(Debug)]
pub enum LoadError {
    ImageDecode(image::ImageError),
    TextureBucketMissing { width: u32, height: u32 },
    TextureBucketFull { width: u32, height: u32 },
    MaterialIndex(material_index::WriteError),
}

fn find_texture_bucket(w: u32, h: u32) -> Option<usize> {
    return TEXTURE_BUCKETS
        .iter()
        .position(|b| b.dimensions.x == w && b.dimensions.y == h);
}

fn load_image<'a>(manifest: &'a Manifest, href: &str) -> &'a [u8] {
    return manifest.asset(href).unwrap();
}

fn get_texture_addressing_mode(addressing: TextureAddressing) -> u32 {
    return match addressing {
        TextureAddressing::Linear => TEXTURE_ADDRESSING_LINEAR,
        TextureAddressing::Nearest => TEXTURE_ADDRESSING_NEAREST,
    };
}

fn load_frame_refs(
    queue: &wgpu::Queue,
    manifest: &Manifest,
    frame_paths: &[String],
    diffuse: &mut [Array; TEXTURE_BUCKETS.len()],
    next_free: &mut [usize; TEXTURE_BUCKETS.len()],
    texture_ref_cache: &mut HashMap<String, material_index::TextureRef>,
) -> Result<Vec<material_index::TextureRef>, LoadError> {
    let mut frames: Vec<material_index::TextureRef> = Vec::with_capacity(frame_paths.len());

    for frame_path in frame_paths {
        if let Some(&cached_ref) = texture_ref_cache.get(frame_path) {
            frames.push(cached_ref);
            continue;
        }

        let frame_data = load_image(manifest, frame_path);
        let img = image::load_from_memory(frame_data)
            .map_err(LoadError::ImageDecode)?
            .to_rgba8();
        let (w, h) = img.dimensions();

        let bucket_ix = find_texture_bucket(w, h).ok_or(LoadError::TextureBucketMissing {
            width: w,
            height: h,
        })?;
        let layer = next_free[bucket_ix];
        let bucket = TEXTURE_BUCKETS[bucket_ix];
        if layer >= bucket.layers {
            return Err(LoadError::TextureBucketFull {
                width: bucket.dimensions.x,
                height: bucket.dimensions.y,
            });
        }
        diffuse[bucket_ix].write(queue, layer, &img);
        next_free[bucket_ix] += 1;

        let texture_ref = material_index::TextureRef {
            bucket: bucket_ix as u16,
            layer: layer as u16,
        };
        texture_ref_cache.insert(frame_path.clone(), texture_ref);
        frames.push(texture_ref);
    }

    return Ok(frames);
}

pub fn load(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    manifest: &Manifest,
    materials: &[Option<&Material>],
) -> Result<LoadResult, LoadError> {
    let mut diffuse =
        TEXTURE_BUCKETS.map(|b| Array::create(device, (b.dimensions.x, b.dimensions.y), b.layers));
    let mut material_index_data = material_index::Data::create();
    let mut next_free: [usize; TEXTURE_BUCKETS.len()] = [0; TEXTURE_BUCKETS.len()];
    let mut texture_ref_cache: HashMap<String, material_index::TextureRef> = HashMap::new();

    for (ix, material) in materials.iter().enumerate() {
        let material = match material {
            Some(material) => *material,
            None => continue,
        };
        let frame_paths = material.frames.as_deref().unwrap_or(&[]);
        let frames = load_frame_refs(
            queue,
            manifest,
            frame_paths,
            &mut diffuse,
            &mut next_free,
            &mut texture_ref_cache,
        )?;
        let animation_speed = material.animation_speed.unwrap_or(DEFAULT_ANIMATION_SPEED);
        let color = material.color.unwrap_or(util::color::WHITE);
        let texture_addressing = material
            .texture_addressing
            .unwrap_or(DEFAULT_TEXTURE_ADDRESSING);
        let texture_addressing_mode = get_texture_addressing_mode(texture_addressing);

        material_index_data
            .write(
                ix as u32,
                animation_speed,
                &frames,
                color,
                texture_addressing_mode,
            )
            .map_err(LoadError::MaterialIndex)?;
    }

    let material_index = storage::MaterialIndex::create(device);
    material_index.write(queue, &material_index_data);
    let bind_group = bind_group::Level::create(device, &diffuse, &material_index);

    return Ok(LoadResult { bind_group });
}
