mod node;
mod vertex;

use glam::Mat4;

use node::walk_node;

pub use vertex::Vertex;

pub struct Material {
    name: String,
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    materials: Vec<Material>,
}

#[derive(Debug)]
pub enum Error {
    Decode,
    NoScene,
    MultipleScenes,
    MeshNotIndexed,
    MaterialUnnamed,
}

impl Mesh {
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        let (document, buffers, _) = ::gltf::import_slice(data).map_err(|_| Error::Decode)?;

        let scenes: Vec<_> = document.scenes().collect();
        let scene = match scenes.len() {
            0 => return Err(Error::NoScene),
            1 => &scenes[0],
            _ => return Err(Error::MultipleScenes),
        };

        let mut materials = Vec::new();
        for material in document.materials() {
            let name = match material.name() {
                Some(name) => name.to_string(),
                None => return Err(Error::MaterialUnnamed),
            };
            materials.push(Material { name });
        }

        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            materials,
        };

        for node in scene.nodes() {
            walk_node(&node, &buffers, Mat4::IDENTITY, &mut mesh)?;
        }

        return Ok(mesh);
    }

    pub fn materials(&self) -> &[Material] {
        return &self.materials;
    }

    pub fn indices(&self) -> &[u32] {
        return &self.indices;
    }

    pub fn vertices(&self) -> &[Vertex] {
        return &self.vertices;
    }
}

impl Material {
    pub fn name(&self) -> &str {
        return &self.name;
    }
}
