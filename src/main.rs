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
        .insert_resource(content::SearchPath(".".into()))
        .add_systems(
            Startup,
            (
                content::find_pages,
                content::read_page_contents,
                content::page::load_page_matter,
                content::page::print_titles,
            )
                .chain(),
        )
        .run();
}
