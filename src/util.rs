use std::{
    mem::forget,
    path::Path,
    ptr::{dangling, dangling_mut},
};

use anyhow::{Context, Result, bail};
use tokio::fs::{self, DirEntry};

use crate::build::copy_files;

#[inline(always)]
#[must_use]
#[track_caller]
pub const fn conjure<'a, T>(x: T) -> &'a T {
    const { assert!(size_of::<T>() == 0) };

    forget(x);

    // SAFETY: We know that `T` is a ZST and that we had a valid `T` before.
    //         We "forgot" it.
    unsafe { &*dangling::<T>() }
}

#[inline(always)]
#[must_use]
#[track_caller]
pub const fn conjure_mut<'a, T>(x: T) -> &'a mut T {
    const { assert!(size_of::<T>() == 0) };

    forget(x);

    // SAFETY: We know that `T` is a ZST and that we had a valid `T` before.
    //         We "forgot" it.
    unsafe { &mut *dangling_mut::<T>() }
}

/// Copies all files and folders from `source` to `destination`.
pub async fn copy_folder(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination).await.with_context(|| {
        format!(
            "creating directory and parents for {:?}",
            destination.display()
        )
    })?;

    let mut entries = fs::read_dir(source)
        .await
        .with_context(|| format!("reading {:?}", source.display()))?;

    while let Some(result) = entries.next_entry().await.transpose() {
        let entry = result.with_context(|| format!("getting entry from {:?}", source.display()))?;
        let entry_path = entry.path();
        let entry_file_name = entry_path.file_name().unwrap();

        if Path::new(entry_file_name).starts_with(".") {
            continue;
        }

        let kind = entry
            .file_type()
            .await
            .with_context(|| format!("getting file kind for {:?}", entry_path.display()))?;

        let output_path = destination.join(entry_file_name);

        if kind.is_dir() {
            Box::pin(copy_folder(&*entry_path, &*output_path))
                .await
                .with_context(|| {
                    format!(
                        "copying {:?} to {:?}",
                        entry_path.display(),
                        output_path.display()
                    )
                })?;
        } else if kind.is_file() {
            fs::copy(&*entry_path, &*output_path)
                .await
                .with_context(|| {
                    format!(
                        "copying {:?} to {:?}",
                        entry_path.display(),
                        output_path.display()
                    )
                })?;
        } else {
            bail!("unknown kind for path {:?}", entry.path().display());
        }
    }

    Ok(())
}

#[test]
fn lol() {
    println!("{}", size_of_val(&copy_files("".as_ref(), "".as_ref())));
}
