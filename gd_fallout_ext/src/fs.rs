use std::{
    io::{BufRead, Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

use ini::Ini;
use log::info;

pub mod dat;
pub mod inifile;
pub mod stdfs;

#[derive(Clone, Debug)]
pub struct Metadata {
    len: u64,
}

impl Metadata {
    pub fn len(&self) -> u64 {
        self.len
    }
}

pub struct FileSystem {
    providers: Vec<Box<dyn Provider>>,
    properties_providers: Vec<Box<dyn PropertiesProvider>>,
}

impl FileSystem {
    pub fn new(args: &clap::ArgMatches) -> Self {
        let mut result = FileSystem {
            providers: Vec::new(),
            properties_providers: Vec::new(),
        };
        result.setup_file_system(Path::new(args.value_of("RESOURCE_DIR").unwrap()));
        return result;
    }

    fn setup_file_system(&mut self, root_dir: &Path) {
        info!("Using resources dir: {}", root_dir.display());

        let mut dat_files = Vec::new();
        let mut ini_files = Vec::new();

        for file in &["fallout2.cfg", "f2_res.ini", "ddraw.ini"] {
            let path: PathBuf = [root_dir, Path::new(file)].iter().collect();
            if path.is_file() {
                info!("Found {}", file);
                ini_files.push(path);
            }
        }

        // Add patchXXX.dat files.
        for i in 0..999 {
            let file = format!("patch{:03}.dat", i);
            let path: PathBuf = [root_dir, Path::new(&file)].iter().collect();
            if path.is_file() {
                info!("Found {}", file);
                dat_files.push(path)
            } else {
                break;
            }
        }
        dat_files.reverse();

        for file in &["master.dat", "critter.dat"] {
            let path: PathBuf = [root_dir, Path::new(file)].iter().collect();
            if path.is_file() {
                info!("Found {}", file);
                dat_files.push(path);
            }
        }

        for ini_file in ini_files.iter() {
            self.register_properties_provider(inifile::new_provider(ini_file).unwrap());
        }

        let data_dir: PathBuf = [root_dir, Path::new("data")].iter().collect();
        if data_dir.is_dir() {
            info!("Found `data` dir");
            self.register_provider(stdfs::new_provider(data_dir).unwrap());
        }

        for dat_file in dat_files.iter().rev() {
            self.register_provider(dat::v2::new_provider(dat_file).unwrap());
        }
    }

    pub fn register_provider(&mut self, provider: Box<dyn Provider>) {
        self.providers.push(provider);
    }

    pub fn register_properties_provider(&mut self, provider: Box<dyn PropertiesProvider>) {
        self.properties_providers.push(provider);
    }

    pub fn properties(&self, path: &str) -> Result<Box<&Ini>> {
        let ref this = self;
        let path = path;
        let mut error: Option<Error> = None;
        for provider in &this.properties_providers {
            match provider.as_ref().reader(path) {
                Ok(r) => return Ok(r),
                Err(e) => {
                    if e.kind() == ErrorKind::NotFound {
                        error = None;
                        continue;
                    }
                    if error.is_none() {
                        error = Some(e);
                    }
                    break;
                }
            }
        }
        Err(error.unwrap_or_else(|| {
            Error::new(ErrorKind::NotFound, format!("file not found: {}", path))
        }))
    }

    pub fn reader(&self, path: &str) -> Result<Box<dyn BufRead + Send>> {
        self.find_provider(path, |p| p.reader(path))
    }

    pub fn metadata(&self, path: &str) -> Result<Metadata> {
        self.find_provider(path, |p| p.metadata(path))
    }

    fn find_provider<T>(&self, path: &str, f: impl Fn(&dyn Provider) -> Result<T>) -> Result<T> {
        let mut error: Option<Error> = None;
        for provider in &self.providers {
            match f(provider.as_ref()) {
                Ok(r) => return Ok(r),
                Err(e) => {
                    if e.kind() == ErrorKind::NotFound {
                        error = None;
                        continue;
                    }
                    if error.is_none() {
                        error = Some(e);
                    }
                    break;
                }
            }
        }
        Err(error.unwrap_or_else(|| {
            Error::new(ErrorKind::NotFound, format!("file not found: {}", path))
        }))
    }

    pub fn exists(&self, path: &str) -> bool {
        self.metadata(path).is_ok()
    }
}

pub trait Provider {
    fn reader(&self, path: &str) -> Result<Box<dyn BufRead + Send>>;
    fn metadata(&self, path: &str) -> Result<Metadata>;
}

pub trait PropertiesProvider {
    fn reader(&self, path: &str) -> Result<Box<&Ini>>;
    fn metadata(&self, path: &str) -> Result<Metadata>;
}
