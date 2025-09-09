use anyhow::{Context, Result, bail};
use std::{borrow::Cow, collections::VecDeque, ffi::OsStr, fs::FileType, path::Path};
use tokio::fs;

pub async fn copy_files(from: &Path, to: &Path) -> Result<()> {
    let mut dir_reader = fs::read_dir(from)
        .await
        .with_context(|| format!("failed to read dir {:?}", from.display()))?;

    fs::create_dir_all(to).await?;

    while let Some(entry) = dir_reader.next_entry().await.transpose() {
        let entry = entry.with_context(|| format!("reading child in dir {:?}", from.display()))?;

        // let file_type = entry
        //     .file_type()
        //     .await
        //     .with_context(|| format!("reading file type for {:?}", entry.path().display()))?;

        let entry_path = entry.path();

        if entry_path.file_name() == Some(OsStr::new(".gitkeep")) {
            continue;
        }

        let output_path = to.join(entry_path.file_name().context("failed to get file name")?);
        let metadata = fs::metadata(&*entry_path)
            .await
            .context("getting metadata")?;

        if metadata.is_dir() {
            // Recurse!
            Box::pin(copy_files(&*entry_path, &*output_path)).await?
        } else if metadata.is_file() {
            let entry_path = entry.path();

            fs::copy(&*entry_path, &*output_path)
                .await
                .map(|_| ())
                .context("failed to copy file")?
        } else {
            bail!("unknown file type");
        }
    }

    Ok(())
}
