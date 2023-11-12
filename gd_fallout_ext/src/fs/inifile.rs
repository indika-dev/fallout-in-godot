use std::{
    borrow::BorrowMut,
    fs::File,
    io::{BufReader, Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

use ini::Ini;
use log::debug;

use super::{Metadata, PropertiesProvider};

const KNOWN_CONFIG_FILES: &'static [&'static str] = &["fallout2.cfg", "f2_res.ini", "ddraw.ini"];

pub fn new_provider<P: AsRef<Path>>(path: P) -> Result<Box<dyn PropertiesProvider>> {
    Ok(Box::new(Inifile::new(path)?))
}

#[derive(Debug)]
struct Inifile {
    name: String,
    file_path: Box<PathBuf>,
    properties: Ini,
    state: State,
}

#[derive(Debug)]
enum State {
    Found,
    Missing,
    Err,
}

impl Inifile {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut result = Inifile {
            name: path
                .as_ref()
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
                .trim()
                .to_string(),
            file_path: Box::new(PathBuf::new()),
            properties: Ini::new(),
            state: State::Missing,
        };
        if File::open(path.as_ref()).is_ok()
            && KNOWN_CONFIG_FILES.contains(
                &path
                    .as_ref()
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
                    .as_str(),
            )
        {
            debug!("loading ini file {}", path.as_ref().display());
            result.file_path.push(path);
            result.properties = Ini::read_from(&mut BufReader::new(
                File::open(result.file_path.as_ref()).unwrap(),
            ))
            .unwrap_or(result.properties);
            result.state = State::Found;
        }
        Ok(result)
    }

    fn file(&self) -> Result<&Ini> {
        match self.state {
            State::Found => return Ok(&self.properties),
            State::Missing => return Err(Error::new(ErrorKind::NotFound, "file not found")),
            State::Err => return Err(Error::new(ErrorKind::NotFound, "file not found")),
        }
    }
}

impl PropertiesProvider for Inifile {
    fn reader(&self, path: &str) -> Result<Box<&Ini>> {
        if path.contains(self.name.as_str()) {
            return Ok(Box::new(&self.properties));
        } else {
            return Err(Error::new(ErrorKind::NotFound, "can't handle file"));
        }
    }

    fn metadata(&self, path: &str) -> Result<Metadata> {
        // let len = self.to_fs_path(path).metadata()?.len();
        if path.contains(self.name.as_str()) {
            Ok(Metadata {
                len: self.properties.len() as u64,
            })
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "can't handle file"))
        }
    }
}

pub fn normalize_path(path: &str) -> String {
    let mut r = String::with_capacity(path.len());

    for c in path.chars() {
        build_normalized_path(&mut r, Some(c));
    }
    build_normalized_path(&mut r, None);

    r
}

pub fn build_normalized_path(path: &mut String, c: Option<char>) {
    if let Some(mut c) = c {
        c = if c == '/' {
            '\\'
        } else {
            c.to_ascii_lowercase()
        };

        path.push(c);
    }

    if path == ".\\" || c.is_none() && path == "." {
        path.truncate(0);
    } else if path.ends_with("\\.\\") {
        let l = path.len();
        path.remove(l - 1);
        path.remove(l - 2);
    }
}
