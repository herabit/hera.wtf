use chrono::{DateTime, Utc};

use crate::content::head::{OgProfile, OgType};

pub mod djot;
pub mod head;

pub fn hello_world() -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        html lang = "en" {
            (head::Head {
                title: "Hello, World!".into(),
                description: "Lol".into(),
                url: "https://hera.wtf/".try_into().unwrap(),
                og_type: OgType::Article {
                    published: Some(Utc::now()),
                    modified: Some(Utc::now()),
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

            body {
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
            }
        }
    }
}
