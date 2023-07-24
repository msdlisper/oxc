use std::{
    hash::BuildHasherDefault,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use dashmap::DashMap;
use rustc_hash::FxHasher;

use crate::{package_json::PackageJson, FileMetadata, FileSystem, ResolveError};

pub struct Cache<Fs> {
    pub(crate) fs: Fs,
    cache: DashMap<Box<Path>, Arc<CacheValue>, BuildHasherDefault<FxHasher>>,
}

impl<Fs: FileSystem> Default for Cache<Fs> {
    fn default() -> Self {
        Self { fs: Fs::default(), cache: DashMap::default() }
    }
}

impl<Fs: FileSystem> Cache<Fs> {
    pub fn new(fs: Fs) -> Self {
        Self { fs, ..Self::default() }
    }

    /// # Panics
    ///
    /// * Path is file but does not have a parent
    pub fn dirname(&self, cache_value: &Arc<CacheValue>) -> Arc<CacheValue> {
        Arc::clone(if cache_value.is_file(&self.fs) {
            cache_value.parent.as_ref().unwrap()
        } else {
            cache_value
        })
    }

    pub fn value(&self, path: &Path) -> Arc<CacheValue> {
        if let Some(cache_entry) = self.cache.get(path) {
            return Arc::clone(cache_entry.value());
        }
        let parent = path.parent().map(|p| self.value(p));
        let data = Arc::new(CacheValue::new(path.to_path_buf().into_boxed_path(), parent));
        self.cache.insert(path.to_path_buf().into_boxed_path(), Arc::clone(&data));
        data
    }
}

#[derive(Debug, Clone)]
pub struct CacheValue {
    path: Box<Path>,
    parent: Option<Arc<CacheValue>>,
    meta: OnceLock<Option<FileMetadata>>,
    symlink: OnceLock<Option<PathBuf>>,
    package_json: OnceLock<Option<Result<Arc<PackageJson>, ResolveError>>>,
}

impl CacheValue {
    fn new(path: Box<Path>, parent: Option<Arc<Self>>) -> Self {
        Self {
            path,
            parent,
            meta: OnceLock::new(),
            symlink: OnceLock::new(),
            package_json: OnceLock::new(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.path.to_path_buf()
    }

    fn meta<Fs: FileSystem>(&self, fs: &Fs) -> Option<FileMetadata> {
        *self.meta.get_or_init(|| fs.metadata(&self.path).ok())
    }

    pub fn is_file<Fs: FileSystem>(&self, fs: &Fs) -> bool {
        self.meta(fs).is_some_and(|meta| meta.is_file)
    }

    pub fn is_dir<Fs: FileSystem>(&self, fs: &Fs) -> bool {
        self.meta(fs).is_some_and(|meta| meta.is_dir)
    }

    pub fn symlink<Fs: FileSystem>(&self, fs: &Fs) -> Option<PathBuf> {
        self.symlink.get_or_init(|| fs.canonicalize(&self.path).ok()).clone()
    }

    /// Find package.json of a path by traversing parent directories.
    ///
    /// # Errors
    ///
    /// * [ResolveError::JSON]
    pub fn find_package_json<Fs: FileSystem>(
        &self,
        fs: &Fs,
    ) -> Result<Option<Arc<PackageJson>>, ResolveError> {
        let mut cache_value = self;
        // Go up a directory when querying a file, this avoids a file read from example.js/package.json
        if cache_value.is_file(fs) {
            if let Some(cv) = &cache_value.parent {
                cache_value = cv.as_ref();
            }
        }
        let mut cache_value = Some(cache_value);
        while let Some(cv) = cache_value {
            if let Some(package_json) = cv.package_json(fs).transpose()? {
                return Ok(Some(Arc::clone(&package_json)));
            }
            cache_value = cv.parent.as_deref();
        }
        Ok(None)
    }

    /// Get package.json of the given path.
    ///
    /// # Errors
    ///
    /// * [ResolveError::JSON]
    pub fn package_json<Fs: FileSystem>(
        &self,
        fs: &Fs,
    ) -> Option<Result<Arc<PackageJson>, ResolveError>> {
        // Change to `get_or_try_init` once it is stable
        self.package_json
            .get_or_init(|| {
                let package_json_path = self.path.join("package.json");
                fs.read_to_string(&package_json_path).ok().map(|package_json_string| {
                    PackageJson::parse(package_json_path.clone(), &package_json_string)
                        .map(Arc::new)
                        .map_err(|error| {
                            ResolveError::from_serde_json_error(package_json_path, &error)
                        })
                })
            })
            .clone()
    }
}