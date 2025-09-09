use std::{
    collections::BTreeMap,
    env,
    io::{Cursor, ErrorKind},
    mem::forget,
    path::{Path, PathBuf},
    process::Output,
};

use anyhow::{Context, bail, ensure};
use bytes::{BufMut, Bytes, BytesMut};
use reqwest::Client;
use zip::ZipArchive;

pub async fn get_token() -> anyhow::Result<String> {
    let tokens = env::vars().filter_map(|(key, value)| {
        let keys = &["GH_TOKEN", "GITHUB_TOKEN"];

        if keys.contains(&&*key) {
            Some(value)
        } else {
            None
        }
    });

    let mut token = if let Some(token) = { tokens }.next() {
        token
    } else {
        let Output { status, stdout, .. } = tokio::process::Command::new("gh")
            .args(&["auth", "token"])
            .output()
            .await
            .context("could not spawn `gh`")?;

        if !status.success() {
            if let Some(code) = status.code() {
                bail!("`gh` returned with exit code {code}")
            } else {
                bail!("`gh` was terminated by a signal")
            }
        }

        String::try_from(stdout).context("the token provided by `gh` is not UTF-8")?
    };

    // SAFETY: This should be obvious
    unsafe {
        let new_len = token.trim_ascii_end().len();
        token.as_mut_vec().set_len(new_len);
    }

    Ok(token)
}

/// Downloads the secrets for this repo, it'll be a zip file.
pub async fn download_secrets(client: &Client, token: &str) -> anyhow::Result<Bytes> {
    let req = client
        .get("https://api.github.com/repos/herabit/secrets.hera.wtf/zipball/main")
        .header("Authorization", &*format!("Bearer {token}"))
        .header("X-Github-Api-Version", "2022-11-28")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1",
        )
        .build()
        .context("failed to build secrets request")?;

    let resp = client
        .execute(req)
        .await
        .context("downloading secrets from github")?;

    ensure!(
        resp.status().is_success(),
        "failed to download secrets from github: status {}",
        resp.status()
    );

    let body = resp
        .bytes()
        .await
        .context("downloading secrets from github")?;

    Ok(body)
}

/// Processes the zip file.
pub async fn process_secrets(secrets_zip: Bytes) -> anyhow::Result<BTreeMap<PathBuf, BytesMut>> {
    tokio::task::spawn_blocking(move || {
        let mut archive = ZipArchive::new(Cursor::new(secrets_zip))
            .context("failed to open the secrets archive")?;

        let mut files = BTreeMap::new();

        // archive.root_dir(filter)

        for index in 0..archive.len() {
            let mut file = archive
                .by_index(index)
                .with_context(|| format!("opening file #{index} in the secrets zip"))?;

            if !file.is_file() {
                continue;
            }

            let name = file
                .enclosed_name()
                .with_context(|| format!("file #{index} has an unsafe name"))?;

            // There's a fucking root folder and I do not feel like dealing with the oddities of the zip file format,
            // I really do not know how to use this library, lol.
            let name = {
                let mut comps = name.components();
                comps.next();

                let new = comps.as_path().to_path_buf();

                drop(name);

                new
            };

            ensure!(
                !files.contains_key(&name),
                "{name:?} (file #{index}) is a duplicate file name"
            );
            // ensure!(
            //     file.is_file(),
            //     "{name:?} (file #{index}) is not actually a file"
            // );

            let mut bytes = BytesMut::with_capacity(file.size().try_into().unwrap_or(0)).writer();

            std::io::copy(&mut file, &mut bytes)
                .with_context(|| format!("{name:?} (file #{index}) failed to read"))?;

            let prev = files.insert(name, bytes.into_inner());

            // We return early if `name` is already in the tree, so just forget the prev as we know
            // it to always be None.
            //
            // Likely not really needed but, wtv.
            forget(prev);
        }

        Ok(files)
    })
    .await
    .context("failed to join secret processing thread")?
}

/// This will get the secrets with defaults.
pub async fn get_secrets() -> anyhow::Result<BTreeMap<PathBuf, BytesMut>> {
    let token = get_token().await?;
    let client = Client::new();
    let data = download_secrets(&client, &*token).await?;

    let file_tree = process_secrets(data).await?;

    Ok(file_tree)
}

/// This will download and store all of the secrets into the secrets folder.
pub async fn install_secrets<P, B, I>(install_to: &Path, iter: I) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    B: AsRef<[u8]>,
    I: IntoIterator<Item = (P, B)>,
{
    let mut buffer = PathBuf::with_capacity(install_to.as_os_str().len());

    for (path, data) in iter {
        let path = {
            buffer.clear();

            buffer.push(install_to);
            buffer.push(path.as_ref());

            &*buffer
        };

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.with_context(|| {
                format!("creating the parent directory for {:?}", path.display())
            })?;
        }

        tokio::fs::write(path, data.as_ref())
            .await
            .with_context(|| format!("writing contents to {:?}", path.display()))?;

        // We want to create gitkeep files.
        while buffer.pop() && Some(&*buffer) != install_to.parent() {
            buffer.push(".gitkeep");

            let result = tokio::fs::File::create_new(&*buffer).await;

            if let Err(err) = result
                && err.kind() != ErrorKind::AlreadyExists
            {
                return Err(err)
                    .with_context(|| format!("writing contents to {:?}", buffer.display()));
            }

            buffer.pop();
        }
    }

    Ok(())
}
