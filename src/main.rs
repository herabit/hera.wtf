use ::bevy::{
    app::{App, Startup},
    ecs::{component::Component, resource::Resource, schedule::IntoScheduleConfigs},
};
use chrono::{DateTime, Utc};

/// Tools for Bevy.
pub mod bevy;
/// Tools relating to content.
pub mod content;
/// Tools for typst.
pub mod typst;

pub fn main() {
    App::new()
        .init_resource::<Now>()
        .add_systems(
            Startup,
            (
                content::page::find,
                content::page::read,
                content::page::load_matter,
                content::djot::parse_events,
            )
                .chain(),
        )
        .run();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Resource)]
pub struct Now(pub DateTime<Utc>);

impl Default for Now {
    #[inline]
    fn default() -> Self {
        Now(Utc::now())
    }
}
