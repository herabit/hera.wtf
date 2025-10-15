use bevy::{
    app::{App, Startup},
    ecs::schedule::IntoScheduleConfigs,
};

pub mod content;

pub fn main() {
    App::new()
        .insert_resource(content::SearchPath(".".into()))
        .add_systems(
            Startup,
            (content::find_pages, content::read_page_contents).chain(),
        )
        .run();
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
// pub struct Page;

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
// pub struct SourcePath(pub PathBuf);

// // #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
