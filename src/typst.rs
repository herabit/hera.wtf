pub mod files;

// use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike, Utc};
// use parking_lot::{MappedMutexGuard as MappedGuard, Mutex, MutexGuard};
// use std::{
//     borrow::Cow,
//     collections::{HashMap, hash_map::Entry},
//     fs,
//     io::ErrorKind,
//     path::{Path, PathBuf},
//     str::Utf8Error,
// };
// use typst::{
//     Library, World,
//     diag::{FileError, FileResult},
//     foundations::{Bytes, Datetime as TypstDateTime},
//     syntax::{FileId, Source},
//     text::{Font, FontBook},
//     utils::LazyHash,
// };
// use typst_kit::{download::ProgressSink, fonts::FontSlot, package::PackageStorage};

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
