use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike, Utc};
use typst::{
    Library, World,
    diag::FileResult,
    foundations::{Bytes, Datetime as TypstDateTime},
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};

/// Our world thingy for typst.
#[derive(Clone)]
pub struct Elysium {
    pub now: DateTime<Utc>,
    pub library: LazyHash<Library>,
    pub fonts: LazyHash<FontBook>,
    pub main: FileId,
}

impl World for Elysium {
    fn library(&self) -> &LazyHash<Library> {
        todo!()
    }

    fn book(&self) -> &LazyHash<FontBook> {
        todo!()
    }

    fn main(&self) -> FileId {
        todo!()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        todo!()
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        todo!()
    }

    fn font(&self, index: usize) -> Option<Font> {
        todo!()
    }

    fn today(&self, offset: Option<i64>) -> Option<TypstDateTime> {
        let now = match offset {
            None => self.now.with_timezone(&Local).fixed_offset(),
            Some(offset) => {
                let offset = i32::try_from(offset).ok()?.checked_mul(60 * 60)?;
                let offset = FixedOffset::east_opt(offset)?;

                self.now.with_timezone(&offset)
            }
        };

        TypstDateTime::from_ymd_hms(
            now.year(),
            now.month().try_into().ok()?,
            now.day().try_into().ok()?,
            now.hour().try_into().ok()?,
            now.minute().try_into().ok()?,
            now.second().try_into().ok()?,
        )
    }
}
