use std::fs;
use std::io::Read;

use url::Url;

use crate::hallways::util;

#[derive(Debug)]
pub enum Error {
    HTTP(Box<ureq::Error>),
    IO(std::io::Error),
    URLJoin(url::ParseError),
    InvalidScheme,
}

pub fn fetch(base_url: &Url, href: &str) -> Result<Vec<u8>, Error> {
    let url = base_url.join(href).map_err(Error::URLJoin)?;
    util::log(util::log::Level::Debug, format!("Loading asset: {url}"));

    return match url.scheme() {
        "http" | "https" => {
            let response = ureq::get(url.as_str())
                .call()
                .map_err(|err| Error::HTTP(Box::new(err)))?;
            let mut data = Vec::new();
            response
                .into_reader()
                .read_to_end(&mut data)
                .map_err(Error::IO)?;
            Ok(data)
        }
        "file" => {
            let path = url.to_file_path().map_err(|_| {
                Error::IO(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "invalid file URL path",
                ))
            })?;
            let data = fs::read(&path).map_err(Error::IO)?;
            Ok(data)
        }
        _ => Err(Error::InvalidScheme),
    };
}
