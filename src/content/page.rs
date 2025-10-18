use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::{Context as _, anyhow};
use bevy::ecs::{
    component::Component,
    entity::Entity,
    error::Result,
    query::{Changed, With},
    system::{Commands, Query},
};

use serde::{Deserialize, Serialize};

use crate::{
    bevy_util::EntityCommandsExt as _,
    content::{Input, front_matter::FrontMatter},
};

/// A struct for storing the metadata for pages.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize, Serialize)]
pub struct PageMatter {
    /// The title of the page.
    #[serde(default)]
    pub title: String,
    /// The description of the page.
    #[serde(default)]
    pub description: String,
    /// The slug of the page.
    #[serde(default)]
    pub slug: Option<String>,
    /// A list of keywords.
    #[serde(default)]
    pub keywords: Vec<String>,
    /// Whether the page is considered a draft.
    #[serde(default)]
    pub draft: bool,
    /// When the page was originally created.
    #[serde(default)]
    pub created: Option<chrono::NaiveDate>,
    /// When the page was last modified.
    #[serde(default)]
    pub modified: Option<chrono::NaiveDate>,
}

/// A title component.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Title(pub String);

/// A description component.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Description(pub String);

/// A slug component.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Slug(pub String);

/// Keyword list component.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Keywords(pub Vec<String>);

/// Draft component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Draft;

/// Created time component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Created(pub chrono::NaiveDate);

/// Created time component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Modified(pub chrono::NaiveDate);

/// Marker component for pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Page;

/// Page content offset component.
///
/// This details how many bytes from the start of the input contents of a page,
/// that the actual body starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct PageOffset(pub usize);

pub fn find(mut commands: Commands) -> Result<()> {
    fn find_pages_inner(root: &Path, paths: &mut Vec<PathBuf>) -> anyhow::Result<()> {
        let mut read_dir =
            std::fs::read_dir(root).with_context(|| format!("reading {:?}", root.display()))?;

        while let Some(entry) = read_dir.next().transpose()? {
            let path = entry.path();
            let file_type = entry
                .file_type()
                .with_context(|| format!("getting metadata for {:?}", root.display()))?;

            if file_type.is_dir() {
                find_pages_inner(&*path, paths)?;
            } else if file_type.is_file() && path.extension() == Some(OsStr::new("dj")) {
                paths.push(path);
            }
        }

        Ok(())
    }

    let mut file_paths = Vec::new();

    find_pages_inner(Path::new("."), &mut file_paths)?;

    commands.spawn_batch(
        file_paths
            .into_iter()
            .map(|path| (Page, Input::<Path>(path.into()))),
    );

    Ok(())
}

pub fn read(
    query: Query<(Entity, &Input<Path>), (With<Page>, Changed<Input<Path>>)>,
    mut commands: Commands,
) -> Result<()> {
    for (entity, Input(path)) in query {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("reading {:?}", path.display()))?;

        commands.entity(entity).insert(Input::<String>(contents));
    }

    Ok(())
}

pub fn load_matter(
    query: Query<(Entity, &Input<String>, &Input<Path>), (With<Page>, Changed<Input<String>>)>,
    mut commands: Commands,
) -> Result<()> {
    for (entity, Input(contents), Input(path)) in query {
        let front_matter = FrontMatter::parse(contents).ok();
        let page_offset = match &front_matter {
            Some(front_matter) => contents.len() - front_matter.rest.len(),
            None => 0,
        };

        let PageMatter {
            title,
            description,
            slug,
            keywords,
            draft,
            created,
            modified,
        } = match &front_matter {
            None => Default::default(),
            Some(FrontMatter {
                language: Some("toml") | None,
                content,
                ..
            }) => toml::from_str(content)
                .with_context(|| format!("parsing {:?} as toml", path.display()))?,
            Some(FrontMatter {
                language: Some(language),
                ..
            }) => {
                return Err(
                    anyhow!("unknown language, {language:?} in {:?}", path.display()).into(),
                );
            }
        };

        commands
            .entity(entity)
            .insert((
                Title(title),
                Description(description),
                Keywords(keywords),
                PageOffset(page_offset),
            ))
            .modify((draft, Draft))
            .modify(slug.map(Slug))
            .modify(created.map(Created))
            .modify(modified.map(Modified));
    }

    Ok(())
}
pub fn print_titles(query: Query<(&Title, &Modified)>) {
    for (Title(title), Modified(modified)) in query {
        println!("{title}: {modified}")
    }
}

// pub fn print_titles(query: Query<(&Title, &Modified)>) {
//     for (Title(title), Modified(modified)) in query {
//         println!("{title}: {modified}")
//     }
// }

// // pub fn process_djot(query: Query<(Entity, &PageOffset, &InputContents)>, _commands: Commands) {
// //     for (_entity, &PageOffset(offset), InputContents(contents)) in query {
// //         let Some(contents) = contents.get(offset..) else {
// //             continue;
// //         };

// //         let mut parser = jotdown::Parser::new(contents);

// //         todo!()
// //     }
// // }

// // /// Metadata for a page's series, if any.
// // #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize, Serialize)]
// // pub struct PageSeries {
// //     pub series: String,
// //     pub entry: usize,
// // }
