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

        commands.entity(entity).insert(InputContents(contents));
    }

    Ok(())
}

/// Marker component for pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Page;

/// A resource containing the path to start searching for pages in.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Resource)]
pub struct SearchPath(pub PathBuf);

/// The path to the input file for a page.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct InputPath(pub PathBuf);

/// The contents of the input file for a page.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct InputContents(pub String);
