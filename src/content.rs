use chrono::Utc;
use head::{OgProfile, OgType};

/// Process front matter.
pub mod front_matter;
/// Stuff for generating head stuff.
pub mod head;

pub fn hello_world() -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        html
            lang = "en"
        {
            @let now = Utc::now();
            (
            head::Head {
                title: "Hello, World!".into(),
                description: "Lol".into(),
                url: "https://hera.wtf/".try_into().unwrap(),
                og_type: OgType::Article {
                    published: Some(now),
                    modified: Some(now),
                    expiration: None,
                    section: None,
                    author: vec![
                        OgProfile {
                            first_name: Some("Hera".into()),
                            last_name: Some("Chamorro".into()),
                            username: Some("herabit".into()),
                        },

                    ],
                    tags: vec!["test".into()],
                }
            })

            body
                class = "theme-ef-dark-hard"
            {
                h1 {
                    "Hello, World!"
                }

                p {
                    "Hello I'm @hera.wtf, a dumbass lil' baby boober doober moober skoober."

                    br;
                    br;

                    "Some call me insane..."

                    br;
                    br;

                    "Little do they know that's actually quite apt of an assertion."
                }

                br;
            }
        }
    }
}
