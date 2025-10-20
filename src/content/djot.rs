use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Commands, Query},
};
use jotdown::{Event, Parser};

use crate::content::{
    Input,
    page::{Page, PageOffset},
};

pub mod to_static;

/// A list of djot events so that we don't need to reparse events when processing a page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Component)]
pub struct DjotEvents(pub Vec<Event<'static>>);

pub fn parse_events(
    query: Query<(Entity, &Input<String>, &PageOffset), (With<Page>, Changed<Input<String>>)>,
    mut commands: Commands,
) {
    for (entity, Input(contents), &PageOffset(offset)) in query {
        let Some(contents) = contents.get(offset..) else {
            continue;
        };

        let events = Parser::new(contents)
            .skip_while(|event| matches!(event, Event::Blankline))
            .map(to_static::event)
            .collect();

        commands.entity(entity).insert(DjotEvents(events));
    }
}
