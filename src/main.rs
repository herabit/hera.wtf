use bevy::{
    app::{App, Startup},
    ecs::schedule::IntoScheduleConfigs,
};

/// Tools relating to content.
pub mod content;

/// Tools for Bevy.
pub mod bevy_util;

pub fn main() {
    App::new()
        .add_systems(
            Startup,
            (
                content::page::find,
                content::page::read,
                content::page::load_matter,
                content::page::parse_djot,
                content::page::print_djot,
            )
                .chain(),
        )
        .run();
}
