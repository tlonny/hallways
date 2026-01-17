use rodio::decoder::DecoderError;

use crate::gltf;
use crate::gpu::buffer::vertex;

use super::manifest;
use super::manifest::fetch::Error as ManifestFetchError;
use super::material;
use super::model::BuildError;
use super::portal;

#[derive(Debug)]
pub enum LoadError {
    Manifest(manifest::LoadError),
    Mesh(gltf::Error),
    Material(material::LoadError),
    Model(BuildError),
    Portal(portal::LoadError),
    Track(DecoderError),
}

fn fetch_error_fmt(
    f: &mut std::fmt::Formatter<'_>,
    context: &str,
    err: &ManifestFetchError,
) -> std::fmt::Result {
    return match err {
        ManifestFetchError::HTTP(err) => {
            write!(f, "failed to load {}: http fetch error ({})", context, err)
        }
        ManifestFetchError::IO(err) => {
            write!(f, "failed to load {}: io fetch error ({})", context, err)
        }
        ManifestFetchError::URLJoin(err) => {
            write!(f, "failed to load {}: URL join error ({})", context, err)
        }
        ManifestFetchError::InvalidScheme => {
            write!(f, "failed to load {}: invalid URL scheme", context)
        }
    };
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            LoadError::Manifest(err) => match err {
                manifest::LoadError::Fetch(fetch_err) => {
                    fetch_error_fmt(f, "level manifest", fetch_err)
                }
                manifest::LoadError::UTF8(err) => {
                    write!(f, "failed to parse level manifest UTF-8: {}", err)
                }
                manifest::LoadError::Decode(err) => {
                    write!(f, "failed to decode level manifest JSON: {}", err)
                }
                manifest::LoadError::TooManyPortals => {
                    write!(f, "failed to parse level manifest: too many portals")
                }
                manifest::LoadError::InvalidVersion => {
                    write!(
                        f,
                        "failed to parse level manifest: invalid manifest version"
                    )
                }
            },
            LoadError::Mesh(gltf_err) => match gltf_err {
                gltf::Error::Decode => {
                    write!(f, "failed to load level mesh: GLTF decode error")
                }
                gltf::Error::NoScene => {
                    write!(f, "failed to load level mesh: no scene in GLTF")
                }
                gltf::Error::MultipleScenes => {
                    write!(f, "failed to load level mesh: multiple scenes in GLTF")
                }
                gltf::Error::MeshNotIndexed => {
                    write!(f, "failed to load level mesh: mesh is not indexed")
                }
                gltf::Error::MaterialUnnamed => {
                    write!(f, "failed to load level mesh: material is unnamed")
                }
            },
            LoadError::Material(err) => match err {
                material::LoadError::ImageDecode(err) => {
                    write!(
                        f,
                        "failed to load level materials: image decode error ({})",
                        err
                    )
                }
                material::LoadError::TextureBucketMissing { width, height } => {
                    write!(
                        f,
                        "failed to load level materials: texture bucket missing for {}x{}",
                        width, height
                    )
                }
                material::LoadError::TextureBucketFull { width, height } => {
                    write!(
                        f,
                        "failed to load level materials: texture bucket full for {}x{}",
                        width, height
                    )
                }
                material::LoadError::MaterialIndex(err) => {
                    write!(
                        f,
                        "failed to load level materials: material index write error ({:?})",
                        err
                    )
                }
            },
            LoadError::Model(err) => match err {
                BuildError::MaterialIXMissing => {
                    write!(
                        f,
                        "failed to build level model: material index missing on vertex"
                    )
                }
                BuildError::MaterialConfigMissing => {
                    write!(
                        f,
                        "failed to build level model: material config missing for index"
                    )
                }
                BuildError::Write(err) => match err {
                    vertex::level::WriteError::CapacityExceeded => {
                        write!(
                            f,
                            "failed to build level model: vertex buffer capacity exceeded"
                        )
                    }
                },
            },
            LoadError::Portal(err) => match err {
                portal::LoadError::UrlJoin(parse_err) => {
                    write!(
                        f,
                        "failed to load level portals: URL join error ({})",
                        parse_err
                    )
                }
                portal::LoadError::Gltf(gltf_err) => {
                    write!(
                        f,
                        "failed to load level portals: GLTF decode error ({:?})",
                        gltf_err
                    )
                }
                portal::LoadError::Geometry(geometry_err) => {
                    write!(
                        f,
                        "failed to load level portals: portal geometry decode error ({:?})",
                        geometry_err
                    )
                }
                portal::LoadError::Write(write_err) => match write_err {
                    vertex::portal::WriteError::CapacityExceeded => {
                        write!(
                            f,
                            "failed to load level portals: vertex buffer capacity exceeded"
                        )
                    }
                },
            },
            LoadError::Track(err) => {
                write!(f, "failed to load level track: decode error ({})", err)
            }
        };
    }
}
