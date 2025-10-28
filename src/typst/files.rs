use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    io::ErrorKind,
    mem,
    path::{Path, PathBuf},
    str::Utf8Error,
};

use bevy::ecs::{component::Component, entity::Entity, resource::Resource};
use parking_lot::Mutex;
use typst::{
    diag::{FileError, FileResult},
    syntax::FileId,
};
use typst_kit::{download::ProgressSink, package::PackageStorage};

use crate::util;

#[derive(Debug, PartialEq, Eq, Hash, Component, Resource)]
pub enum FileData {
    Text(Cow<'static, str>),
    Binary(Cow<'static, [u8]>),
}

impl FileData {
    /// Creates a new [`TypstFile`] from some input buffer.
    ///
    /// If the input buffer is UTF-8 the resulting value is of
    /// the [`TypstFile::Text`] variant. Otherwise, it defaults
    /// to [`TypstFile::Binary`].
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new(binary: Cow<'static, [u8]>) -> FileData {
        match str::from_utf8(&binary) {
            Ok(..) => {
                // SAFETY: Above we confirm that `binary` is valid UTF-8.
                let text = match binary {
                    Cow::Borrowed(s) => Cow::Borrowed(unsafe { str::from_utf8_unchecked(s) }),
                    Cow::Owned(s) => Cow::Owned(unsafe { String::from_utf8_unchecked(s) }),
                };

                FileData::Text(text)
            }
            Err(_) => FileData::Binary(binary),
        }
    }

    /// Attempts to get a string buffer, consuming `self`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn into_text(self) -> Result<Cow<'static, str>, (Cow<'static, [u8]>, Utf8Error)> {
        let binary = match self {
            FileData::Text(text) => return Ok(text),
            FileData::Binary(binary) => binary,
        };

        match str::from_utf8(&*binary) {
            // SAFETY: If this branch is hit, then we know `binary` is valid UTF-8.
            Ok(..) => Ok(match binary {
                Cow::Borrowed(s) => Cow::Borrowed(unsafe { str::from_utf8_unchecked(s) }),
                Cow::Owned(s) => Cow::Owned(unsafe { String::from_utf8_unchecked(s) }),
            }),
            Err(err) => Err((binary, err)),
        }
    }

    /// Gets a binary buffer from `self`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn into_binary(self) -> Cow<'static, [u8]> {
        match self {
            FileData::Binary(binary) => binary,
            FileData::Text(text) => match text {
                Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
                Cow::Owned(s) => Cow::Owned(s.into_bytes()),
            },
        }
    }

    /// Attempts to convert `self` into a text buffer, returning a reference to
    /// the underyling buffer.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn make_text(
        &mut self,
    ) -> Result<&mut Cow<'static, str>, (&mut Cow<'static, [u8]>, Utf8Error)> {
        match mem::take(self).into_text() {
            Ok(text) => {
                util::write(self, FileData::Text(text));

                match self {
                    FileData::Text(text) => Ok(text),
                    FileData::Binary(..) => unreachable!(),
                }
            }
            Err((binary, err)) => {
                util::write(self, FileData::Binary(binary));

                match self {
                    FileData::Binary(binary) => Err((binary, err)),
                    FileData::Text(..) => unreachable!(),
                }
            }
        }
    }

    /// Same as [`FileData::make_text`] except no validation is performed *at all*.
    ///
    /// # Safety
    ///
    /// The caller needs to ensure that if `self` is a binary buffer, that the
    /// binary buffer is valid UTF-8. Failure to do so is *undefined behavior*.
    #[inline]
    #[must_use]
    #[track_caller]
    pub unsafe fn make_text_unchecked(&mut self) -> &mut Cow<'static, str> {
        let text = match mem::take(self) {
            FileData::Text(text) => text,
            // SAFETY: The caller ensures `binary` is valid UTF-8.
            FileData::Binary(binary) => match binary {
                Cow::Borrowed(s) => Cow::Borrowed(unsafe { str::from_utf8_unchecked(s) }),
                Cow::Owned(s) => Cow::Owned(unsafe { String::from_utf8_unchecked(s) }),
            },
        };

        util::write(self, FileData::Text(text));

        match self {
            FileData::Text(text) => text,
            FileData::Binary(..) => unreachable!(),
        }
    }

    /// Convert this buffer into a binary buffer, returning a reference to the underlying
    /// buffer.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn make_binary(&mut self) -> &mut Cow<'static, [u8]> {
        let binary = mem::take(self).into_binary();

        util::write(self, FileData::Binary(binary));

        match self {
            FileData::Binary(binary) => binary,
            FileData::Text(..) => unreachable!(),
        }
    }

    /// Clear the buffer, setting the length to zero.
    #[inline]
    #[track_caller]
    pub fn clear(&mut self) {
        match self {
            FileData::Text(Cow::Borrowed(text)) => *text = &text[..0],
            FileData::Text(Cow::Owned(text)) => text.clear(),
            FileData::Binary(Cow::Borrowed(binary)) => *binary = &binary[..0],
            FileData::Binary(Cow::Owned(binary)) => binary.clear(),
        }
    }
}

impl Default for FileData {
    #[inline]
    fn default() -> Self {
        FileData::Text("".into())
    }
}

impl Clone for FileData {
    fn clone(&self) -> Self {
        match self {
            Self::Text(text) => Self::Text(text.clone()),
            Self::Binary(binary) => Self::Binary(binary.clone()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match source {
            FileData::Text(src) => {
                // NOTE: We're setting the length to zero, allowing us to safely
                //       treat the same allocation as a UTF-8 buffer.
                self.clear();

                // SAFETY: See above.
                let dest = unsafe { self.make_text_unchecked() };

                dest.clone_from(src)
            }
            FileData::Binary(binary) => self.make_binary().clone_from(binary),
        }
    }
}

/// A location where a typst file is stored.
#[derive(Debug, Clone, Copy)]
pub enum FileLoc<'a> {
    /// A file location on disk.
    Disk(&'a Path),
    /// A file attached to an entity...
    Entity(Entity),
}

/// A typst file entry that is lazily loaded.
#[derive(Debug, Default)]
pub struct FileEntry {
    accessed: bool,
    result: Option<FileResult<FileData>>,
}

impl FileEntry {
    #[inline]
    pub fn accessed(&self) -> bool {
        self.accessed
    }

    pub fn get_or_init(
        &mut self,
        loc: FileLoc,
        entity_lookup: &mut dyn FnMut(Entity) -> Option<FileData>,
    ) -> &mut FileResult<FileData> {
        if mem::replace(&mut self.accessed, true) && self.result.is_some() {
            self.result.as_mut().unwrap()
        } else {
            self.result.insert(match loc {
                FileLoc::Disk(path) => {
                    fs::read(path)
                        .map(Cow::Owned)
                        .map(FileData::new)
                        .map_err(|err| {
                            if let ErrorKind::IsADirectory = err.kind() {
                                FileError::IsDirectory
                            } else {
                                FileError::from_io(err, path)
                            }
                        })
                }
                FileLoc::Entity(entity) => entity_lookup(entity).ok_or(FileError::AccessDenied),
            })
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.accessed = false;
    }
}

/// A lookup

/// The storage for typst files.
#[derive(Debug, Default, Component, Resource)]
pub struct Files(pub Mutex<HashMap<FileId, FileData>>);

impl Files {
    pub fn resolve<'l>(
        id: FileId,
        root: &Path,
        package_storage: &PackageStorage,
        entity_lookup: &mut dyn FnMut(FileId) -> Option<Entity>,
        buf: &'l mut PathBuf,
    ) -> FileResult<FileLoc<'l>> {
        entity_lookup(id)
            .map(FileLoc::Entity)
            .map(Ok)
            .unwrap_or_else(|| {
                let package_root = id
                    .package()
                    .map(|spec| package_storage.prepare_package(spec, &mut ProgressSink))
                    .transpose()?;

                let root = package_root.as_deref().unwrap_or(root);

                id.vpath()
                    .resolve(root)
                    .map(|path| {
                        *buf = path;
                        &**buf
                    })
                    .map(FileLoc::Disk)
                    .ok_or(FileError::AccessDenied)
            })
    }
}
