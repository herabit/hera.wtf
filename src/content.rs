use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::Context;
use bevy::ecs::{
    component::Component,
    entity::Entity,
    error::Result,
    query::{With, Without},
    resource::Resource,
    system::{Commands, Query, Res},
};
use chrono::Utc;
use head::{OgProfile, OgType};

/// Stuff for rendering djot.
pub mod djot;
/// Process front matter.
pub mod front_matter;
/// Stuff for generating head stuff.
pub mod head;
/// Stuff for pages.
pub mod page;

pub fn find_pages(root: Res<SearchPath>, mut commands: Commands) -> Result<()> {
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

    find_pages_inner(&*root.0, &mut file_paths)?;

    commands.spawn_batch(file_paths.into_iter().map(|path| (Page, InputPath(path))));

    Ok(())
}

pub fn read_page_contents(
    query: Query<(Entity, &InputPath), (With<Page>, Without<InputContents>)>,
    mut commands: Commands,
) -> Result<()> {
    for (entity, InputPath(path)) in query.iter() {
        let contents = std::fs::read_to_string(&**path)
            .with_context(|| format!("reading {:?}", path.display()))?;

        println!("{:?}", path.display());

        commands.entity(entity).insert(InputContents(contents));
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Page;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Resource)]
pub struct SearchPath(pub PathBuf);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct InputPath(pub PathBuf);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct InputContents(pub String);

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
// pub struct Title(pub String);

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
// pub struct Description(pub String);

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Resource)]
// pub struct SeriesMap()

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
// pub struct Series(pub Entity);
