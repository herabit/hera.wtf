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
    content::{InputContents, Page, front_matter::FrontMatter},
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

/// Loads the page matters.
pub fn load_page_matter(
    query: Query<(Entity, &InputContents), (With<Page>, Changed<InputContents>)>,
    mut commands: Commands,
) -> Result<()> {
    for (entity, InputContents(contents)) in query.iter() {
        let front_matter = FrontMatter::parse(&contents).unwrap_or(FrontMatter {
            content: "",
            language: Some("toml"),
            rest: &contents,
        });

        let PageMatter {
            title,
            description,
            slug,
            keywords,
            draft,
            created,
            modified,
        } = match front_matter.language {
            Some("toml") => toml::from_str(front_matter.content)?,
            _ => continue,
        };

        commands
            .entity(entity)
            .insert((Title(title), Description(description), Keywords(keywords)))
            .modify((draft, Draft))
            .modify(slug.map(Slug))
            .modify(created.map(Created))
            .modify(modified.map(Modified));
    }

    Ok(())
}

pub fn print_titles(query: Query<&Title>) {
    for Title(title) in query {
        println!("{title}")
    }
}

// /// Metadata for a page's series, if any.
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize, Serialize)]
// pub struct PageSeries {
//     pub series: String,
//     pub entry: usize,
// }
