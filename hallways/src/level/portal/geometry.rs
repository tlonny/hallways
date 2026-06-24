use glam::Vec3;

use crate::hallways::gltf::Vertex;

#[derive(Debug)]
pub enum DecodeError {
    InsufficientVertices,
    DegenerateGeometry,
    NotCoplanar,
    TiltedPortal,
    UnstableAnchor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct Geometry {
    pub center: Vec3,
    pub normal: Vec3,
    pub yaw: f32,
    pub kind: Kind,
}

const EPSILON: f32 = 0.001;
const NORMAL_MATCH_EPSILON: f32 = 0.001;

impl Geometry {
    pub fn decode(vertices: &[Vertex], indices: &[u32]) -> Result<Geometry, DecodeError> {
        if vertices.len() < 3 {
            return Err(DecodeError::InsufficientVertices);
        }

        let mut normal = None;
        for tri in indices.chunks_exact(3) {
            let a = vertices[tri[0] as usize].position;
            let b = vertices[tri[1] as usize].position;
            let c = vertices[tri[2] as usize].position;
            let tri_normal = (b - a).cross(c - a);
            if tri_normal.length() > EPSILON {
                normal = Some(tri_normal.normalize());
                break;
            }
        }
        let normal = normal.ok_or(DecodeError::DegenerateGeometry)?;

        if normal.is_nan() {
            return Err(DecodeError::DegenerateGeometry);
        }

        let anchor = vertices[indices[0] as usize].position;
        for vertex in vertices {
            let dist = (vertex.position - anchor).dot(normal).abs();
            if dist > EPSILON {
                return Err(DecodeError::NotCoplanar);
            }
        }

        let mut center = Vec3::ZERO;
        for vertex in vertices {
            center += vertex.position;
        }
        center /= vertices.len() as f32;

        if normal.y.abs() < EPSILON {
            let yaw = normal.x.atan2(normal.z);
            return Ok(Geometry {
                center,
                normal,
                yaw,
                kind: Kind::Horizontal,
            });
        }
        if (normal.y - 1.0).abs() >= EPSILON && (normal.y + 1.0).abs() >= EPSILON {
            return Err(DecodeError::TiltedPortal);
        }

        let center_to_anchor = anchor - center;
        if center_to_anchor.length() < EPSILON {
            return Err(DecodeError::UnstableAnchor);
        }
        let center_to_anchor = center_to_anchor.normalize();

        let roll = normal
            .dot(Vec3::X.cross(center_to_anchor))
            .atan2(Vec3::X.dot(center_to_anchor));

        return Ok(Geometry {
            center,
            normal,
            yaw: roll,
            kind: Kind::Vertical,
        });
    }

    pub fn matches(&self, other: &Geometry) -> bool {
        return match (self.kind, other.kind) {
            (Kind::Horizontal, Kind::Horizontal) => true,
            (Kind::Vertical, Kind::Vertical) => {
                (self.normal - other.normal).length() <= NORMAL_MATCH_EPSILON
            }
            _ => false,
        };
    }
}
