use std::{
    collections::{HashMap, hash_map::Entry},
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    str::Utf8Error,
};

use bevy::ecs::{component::Component, resource::Resource};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use typst::{
    diag::{FileError, FileResult},
    syntax::FileId,
};
use typst_kit::{download::ProgressSink, package::PackageStorage};

/// A mapping of [`FileId`]s to file entries.
#[derive(Debug, Default, Component, Resource)]
pub struct Files(Mutex<HashMap<FileId, FileEntry>>);

impl Files {
    /// Attempt to resolve a file.
    pub fn resolve(
        root: &Path,
        package_storage: &PackageStorage,
        id: FileId,
    ) -> FileResult<PathBuf> {
        let package_root = id
            .package()
            .map(|spec| package_storage.prepare_package(spec, &mut ProgressSink))
            .transpose()?;
        let root = package_root.as_deref().unwrap_or(root);

        id.vpath().resolve(root).ok_or(FileError::AccessDenied)
    }

    /// Attempt to resolve and load a file.
    pub fn load(
        &self,
        root: &Path,
        package_storage: &PackageStorage,
        id: FileId,
    ) -> MappedMutexGuard<'_, FileEntry> {
        let read_file = |path: PathBuf| {
            fs::read(&*path).map_err(|err| {
                if let ErrorKind::IsADirectory = err.kind() {
                    FileError::IsDirectory
                } else {
                    FileError::from_io(err, &*path)
                }
            })
        };

        MutexGuard::map(self.0.lock(), |files| match files.entry(id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(
                Files::resolve(root, package_storage, id)
                    .and_then(read_file)
                    .map(FileData::new),
            ),
        })
    }

    /// Attempt to resolve and load a file as binary.
    pub fn load_binary(
        &self,
        root: &Path,
        package_storage: &PackageStorage,
        id: FileId,
    ) -> FileEntry {
        match &*self.load(root, package_storage, id) {
            Ok(FileData::Text(text)) => Ok(FileData::Binary(text.as_bytes().to_owned())),
            Ok(FileData::Binary(binary)) => Ok(FileData::Binary(binary.as_slice().to_owned())),
            Err(err) => Err(err.clone()),
        }
    }

    /// Attempt to resolve and load a file as text.
    pub fn load_text(
        &self,
        root: &Path,
        package_storage: &PackageStorage,
        id: FileId,
    ) -> FileEntry {
        match &*self.load(root, package_storage, id) {
            Ok(FileData::Text(text)) => Ok(FileData::Text(text.clone())),
            Ok(FileData::Binary(..)) => Err(FileError::InvalidUtf8),
            // Ok(FileData::Binary(binary)) => Ok(FileData::Text(str::from_utf8(binary)?.to_owned())),
            Err(err) => Err(err.clone()),
        }
    }

    /// Get a reference to the inner map.
    ///
    /// This should be avoided.
    #[inline]
    pub fn inner(&self) -> &Mutex<HashMap<FileId, FileEntry>> {
        &self.0
    }

    /// Get a mutable reference to the inner map.
    ///
    /// This should be avoided.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut Mutex<HashMap<FileId, FileEntry>> {
        &mut self.0
    }
}

impl From<Mutex<HashMap<FileId, FileEntry>>> for Files {
    #[inline]
    fn from(value: Mutex<HashMap<FileId, FileEntry>>) -> Self {
        Files(value)
    }
}

impl From<HashMap<FileId, FileEntry>> for Files {
    #[inline]
    fn from(value: HashMap<FileId, FileEntry>) -> Self {
        Files(Mutex::new(value))
    }
}

impl From<Files> for Mutex<HashMap<FileId, FileEntry>> {
    #[inline]
    fn from(value: Files) -> Self {
        value.0
    }
}

impl From<Files> for HashMap<FileId, FileEntry> {
    #[inline]
    fn from(value: Files) -> Self {
        value.0.into_inner()
    }
}

/// An entry in [`Files`].
pub type FileEntry = FileResult<FileData>;

/// An enum differentiating the various kinds of file data we care about.
#[derive(Debug, Clone)]
pub enum FileData {
    Text(String),
    Binary(Vec<u8>),
}

impl FileData {
    /// Create a new [`FileData`] from some buffer.
    #[inline]
    pub fn new(buffer: Vec<u8>) -> FileData {
        match String::from_utf8(buffer) {
            Ok(text) => FileData::Text(text),
            Err(err) => FileData::Binary(err.into_bytes()),
        }
    }

    /// Get the inner [`str`] if `self` is a [`FileData::Text`].
    #[inline]
    pub const fn as_str(&self) -> Option<&str> {
        match self {
            FileData::Text(text) => Some(text.as_str()),
            FileData::Binary(..) => None,
        }
    }

    /// Get the inner [`str`] if `self` is valid UTF-8.
    #[inline]
    pub const fn to_str(&self) -> Result<&str, Utf8Error> {
        match self {
            FileData::Text(text) => Ok(text.as_str()),
            FileData::Binary(binary) => str::from_utf8(binary.as_slice()),
        }
    }

    /// Get the inner [`str`] mutably if `self` is a [`FileData::Text`].
    #[inline]
    pub const fn as_str_mut(&mut self) -> Option<&mut str> {
        match self {
            FileData::Text(text) => Some(text.as_mut_str()),
            FileData::Binary(..) => None,
        }
    }

    /// Get the inner [`str`] mutably if `self` is valid UTF-8.
    #[inline]
    pub const fn to_str_mut(&mut self) -> Result<&mut str, Utf8Error> {
        match self {
            FileData::Text(text) => Ok(text.as_mut_str()),
            FileData::Binary(binary) => str::from_utf8_mut(binary.as_mut_slice()),
        }
    }
}
