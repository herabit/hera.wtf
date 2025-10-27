use std::{
    borrow::Cow,
    collections::{HashMap, hash_map::Entry},
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    str::{FromStr as _, Utf8Error},
};

use bevy::ecs::{component::Component, entity::Entity, resource::Resource, system::Query};
use chrono::{DateTime, Utc};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use typst::{
    Library, World,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source},
    text::Font,
};
use typst::{text::FontBook, utils::LazyHash};
use typst_kit::fonts::{FontSearcher, FontSlot};
use typst_kit::{download::ProgressSink, package::PackageStorage};

use crate::content::Input;

/// A mapping of [`FileId`]s to file entries.
#[derive(Debug, Default, Component, Resource)]
pub struct Files {
    /// The actual mapping of [`FileId`]s to file entries.
    ///
    /// One should avoid using this directly.
    pub inner: Mutex<HashMap<FileId, FileEntry>>,
}

impl Files {
    /// Attempt to resolve a file.
    pub fn resolve(
        root: &Path,
        package_storage: &PackageStorage,
        // inputs: Option<Inputs>,
        id: FileId,
    ) -> FileResult<FileLocation> {
        let vpath = id.vpath();

        // Is this correct? Not *exactly*. Do I care? Not really.``
        if let [b'<', entity @ .., b'>'] = vpath.as_rooted_path().as_os_str().as_encoded_bytes()
            && let Ok(entity) = str::from_utf8(entity)
            && let Ok(entity) = u64::from_str(entity)
        {
            Ok(FileLocation::Entity(Entity::from_bits(entity)))
        } else {
            let package_root = id
                .package()
                .map(|spec| package_storage.prepare_package(spec, &mut ProgressSink))
                .transpose()?;
            let root = package_root.as_deref().unwrap_or(root);

            vpath
                .resolve(root)
                .map(FileLocation::Path)
                .ok_or(FileError::AccessDenied)
        }
    }

    /// Attempt to resolve and load a file.
    pub fn load(
        &self,
        root: &Path,
        package_storage: &PackageStorage,
        id: FileId,
    ) -> MappedMutexGuard<'_, FileEntry> {
        let read_file = |_path: FileLocation| -> FileResult<Vec<u8>> {
            todo!()
            // fs::read(&*path).map_err(|err| {
            //     if let ErrorKind::IsADirectory = err.kind() {
            //         FileError::IsDirectory
            //     } else {
            //         FileError::from_io(err, &*path)
            //     }
            // })
        };

        MutexGuard::map(self.inner.lock(), |files| match files.entry(id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(
                Files::resolve(root, package_storage, id)
                    .and_then(read_file)
                    .map(FileData::new),
            ),
        })
    }

    // /// Attempt to resolve and load a file as binary.
    // pub fn load_binary(
    //     &self,
    //     root: &Path,
    //     package_storage: &PackageStorage,
    //     id: FileId,
    // ) -> FileResult<Vec<u8>> {
    //     match &*self.load(root, package_storage, id) {
    //         Ok(FileData::Text(text)) => Ok(text.as_bytes().to_owned()),
    //         Ok(FileData::Binary(binary)) => Ok(binary.as_slice().to_owned()),
    //         Err(err) => Err(err.clone()),
    //     }
    // }

    // /// Attempt to resolve and load a file as text.
    // pub fn load_text(
    //     &self,
    //     root: &Path,
    //     package_storage: &PackageStorage,
    //     id: FileId,
    // ) -> FileResult<String> {
    //     match &*self.load(root, package_storage, id) {
    //         Ok(FileData::Text(text)) => Ok(text.to_owned()),
    //         Ok(FileData::Binary(binary)) => str::from_utf8(binary)
    //             .map(ToOwned::to_owned)
    //             .map_err(|_| FileError::InvalidUtf8),
    //         Err(err) => Err(err.clone()),
    //     }
    // }
}

/// An entry in [`Files`].
pub type FileEntry = FileResult<FileData>;

/// An enum differentiating the various kinds of file data we care about.
#[derive(Debug, Clone)]
pub enum FileData {
    /// UTF-8 file data.
    Text(String),
    /// File data of unknown encoding.
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

/// A location for some file... This is *either* an entity *or* a file path.
#[derive(Debug, Clone)]
pub enum FileLocation {
    Path(PathBuf),
    Entity(Entity),
}

/// The font storage for typst.
#[derive(Debug, Default, Component, Resource)]
pub struct Fonts {
    /// Metadata about the various fonts.
    pub book: LazyHash<FontBook>,
    /// The list of loaded fonts.
    pub fonts: Vec<FontSlot>,
}

impl Fonts {
    /// Loads the fonts from disk.
    pub fn load() -> Fonts {
        let fonts = FontSearcher::new()
            .include_system_fonts(true)
            .include_embedded_fonts(true)
            .search();

        Fonts {
            book: LazyHash::new(fonts.book),
            fonts: fonts.fonts,
        }
    }
}

/// The root path for typst to load files.
#[derive(Debug, Clone, Component, Resource)]
pub struct Root(pub Cow<'static, Path>);

/// The package storage for typst.
#[derive(Debug, Component, Resource)]
pub struct Packages(pub PackageStorage);

/// The library for typst.
#[derive(Debug, Component, Resource)]
pub struct Lib(pub LazyHash<Library>);

/// The current moment in time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Resource)]
pub struct Now(pub DateTime<Utc>);

/// The file id for something.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Resource)]
pub struct Main(pub FileId);

pub type Inputs<'world, 'state> = Query<'world, 'state, (Entity, &'world Input<str>)>;

/// A typst world.
#[derive(Debug, Clone, Copy)]
pub struct Elysium<'world, 'state> {
    pub root: &'world Path,
    pub now: DateTime<Utc>,
    pub lib: &'world Lib,
    pub main: FileId,
    pub fonts: &'world Fonts,
    pub files: &'world Files,
    pub packages: &'world Packages,
    pub query: Inputs<'world, 'state>,
}

impl<'world, 'state> Elysium<'world, 'state> {
    // pub fn load
}

impl<'world, 'state> World for Elysium<'world, 'state> {
    #[inline]
    fn library(&self) -> &LazyHash<Library> {
        &self.lib.0
    }

    #[inline]
    fn book(&self) -> &LazyHash<FontBook> {
        &self.fonts.book
    }

    #[inline]
    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.files
            .load(self.root, &self.packages.0, id)
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|data| match data {
                FileData::Text(text) => Ok(&**text),
                FileData::Binary(binary) => {
                    str::from_utf8(binary).map_err(|_| FileError::InvalidUtf8)
                }
            })
            .map(|text| Source::new(id, text.to_owned()))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        todo!()
        // self.files.load_file(self.root, &self.packages.0, id).map(|binary| )
    }

    fn font(&self, index: usize) -> Option<Font> {
        todo!()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        todo!()
    }
}

// /// Our world thingy for typst.
// pub struct Elysium {
//     /// The root directory from which we load files from.
//     pub root: Cow<'static, Path>,
//     /// The time of world creation.
//     pub now: DateTime<Utc>,
//     /// The standard library we're using.
//     pub library: LazyHash<Library>,
//     pub main: FileId,
//     pub fonts: Vec<FontSlot>,
//     pub font_book: LazyHash<FontBook>,
//     pub files: Mutex<HashMap<FileId, FileResult<FileData>>>,
//     pub packages: PackageStorage,
// }

// impl Elysium {
//     pub fn resolve(&self, id: FileId) -> FileResult<PathBuf> {
//         let root = id
//             .package()
//             .map(|spec| self.packages.prepare_package(spec, &mut ProgressSink))
//             .transpose()?;

//         let root = root.as_deref().unwrap_or(&self.root);

//         id.vpath().resolve(root).ok_or(FileError::AccessDenied)
//     }

//     #[unsafe(no_mangle)]
//     pub fn load(&self, id: FileId) -> MappedGuard<'_, FileResult<FileData>> {
//         // NOTE: Is this too nested? Yes. Do I care? Not at fucking all.
//         MutexGuard::map(self.files.lock(), |files| match files.entry(id) {
//             Entry::Occupied(entry) => entry.into_mut(),
//             Entry::Vacant(entry) => entry.insert(
//                 self.resolve(id)
//                     .and_then(|path| {
//                         fs::read(&*path).map_err(|err| {
//                             if err.kind() == ErrorKind::IsADirectory {
//                                 FileError::IsDirectory
//                             } else {
//                                 FileError::from_io(err, &*path)
//                             }
//                         })
//                     })
//                     .map(|data| match String::from_utf8(data) {
//                         Ok(string) => FileData::String(string),
//                         Err(err) => FileData::Bytes(err.into_bytes()),
//                     }),
//             ),
//         })
//     }
// }

// impl World for Elysium {
//     fn library(&self) -> &LazyHash<Library> {
//         &self.library
//     }

//     fn book(&self) -> &LazyHash<FontBook> {
//         &self.font_book
//     }

//     fn main(&self) -> FileId {
//         self.main
//     }

//     fn source(&self, id: FileId) -> FileResult<Source> {
//         match &*self.load(id) {
//             Ok(FileData::String(text)) => Ok(Source::new(id, text.clone())),
//             Ok(FileData::Bytes(..)) => Err(FileError::InvalidUtf8),
//             Err(err) => Err(err.clone()),
//         }
//     }

//     fn file(&self, id: FileId) -> FileResult<Bytes> {
//         match &*self.load(id) {
//             Ok(FileData::String(text)) => Ok(Bytes::from_string(text.clone())),
//             Ok(FileData::Bytes(bytes)) => Ok(Bytes::new(bytes.clone())),
//             Err(err) => Err(err.clone()),
//         }
//     }

//     fn font(&self, index: usize) -> Option<Font> {
//         self.fonts.get(index).and_then(FontSlot::get)
//     }

//     fn today(&self, offset: Option<i64>) -> Option<TypstDateTime> {
//         let now = match offset {
//             None => self.now.with_timezone(&Local).fixed_offset(),
//             Some(offset) => {
//                 let offset = i32::try_from(offset).ok()?.checked_mul(60 * 60)?;
//                 let offset = FixedOffset::east_opt(offset)?;

//                 self.now.with_timezone(&offset)
//             }
//         };

//         TypstDateTime::from_ymd_hms(
//             now.year(),
//             now.month().try_into().ok()?,
//             now.day().try_into().ok()?,
//             now.hour().try_into().ok()?,
//             now.minute().try_into().ok()?,
//             now.second().try_into().ok()?,
//         )
//     }
// }

// /// File data schenanigans.
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum FileData {
//     String(String),
//     Bytes(Vec<u8>),
// }

// impl FileData {
//     #[inline]
//     pub const fn as_str(&self) -> Result<&str, Utf8Error> {
//         match self {
//             FileData::String(string) => Ok(string.as_str()),
//             FileData::Bytes(bytes) => str::from_utf8(bytes.as_slice()),
//         }
//     }

//     #[inline]
//     pub const fn as_str_mut(&mut self) -> Result<&mut str, Utf8Error> {
//         match self {
//             FileData::String(string) => Ok(string.as_mut_str()),
//             FileData::Bytes(bytes) => str::from_utf8_mut(bytes.as_mut_slice()),
//         }
//     }
// }
