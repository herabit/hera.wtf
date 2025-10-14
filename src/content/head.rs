use chrono::{DateTime, Utc};
use maud::{Markup, Render, html};
use serde::{Deserialize, Serialize};
use url::Url;

/// OpenGraph Profile.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct OgProfile {
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
}

/// OpenGraph types.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum OgType {
    #[default]
    Website,

    Article {
        #[serde(default)]
        published: Option<DateTime<Utc>>,

        #[serde(default)]
        modified: Option<DateTime<Utc>>,

        #[serde(default)]
        expiration: Option<DateTime<Utc>>,

        #[serde(default)]
        section: Option<String>,

        #[serde(default)]
        author: Vec<OgProfile>,

        #[serde(default)]
        tags: Vec<String>,
    },
}

impl Render for OgType {
    fn render(&self) -> Markup {
        match self {
            OgType::Website => html! {
                meta
                    property = "og:type"
                    content = "website";
            },
            OgType::Article {
                published,
                modified,
                expiration,
                section,
                author,
                tags,
            } => html! {
                meta
                    property = "og:type"
                    content = "article";

                @if let Some(published) = published {
                    meta
                        property = "og:article:published_time"
                        content = (published.to_rfc3339());
                }

                @if let Some(modified) = modified {
                    meta
                        property = "og:article:modified_time"
                        content = (modified.to_rfc3339());
                }

                @if let Some(expiration) = expiration {
                    meta
                        property = "og:article:expiration_time"
                        content = (expiration.to_rfc3339());
                }

                @for author in author {
                    @if let Some(first_name) = &author.first_name {
                        meta
                            property = "og:article:author:first_name"
                            content = (first_name);
                    }

                    @if let Some(last_name) = &author.last_name {
                        meta
                            property = "og:article:author:last_name"
                            content = (last_name);
                    }

                    @if let Some(username) = &author.username {
                        meta
                            property = "og:article:author:username"
                            content = (username);
                    }
                }

                @if let Some(section) = section {
                    meta
                        property = "og:article:section"
                        content = (section);
                }

                @for tag in tags {
                    meta
                        property = "og:article:tag"
                        content = (tag);
                }
            },
        }
    }
}

/// Stuff for generating the `head` tag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Head {
    pub title: String,
    pub description: String,
    pub url: Url,
    pub og_type: OgType,
}

impl Render for Head {
    fn render(&self) -> Markup {
        html! {
            head {
                title { (self.title) }

                meta
                    charset = "utf-8";
                meta
                    name = "title"
                    content = (self.title);
                meta
                    name = "description"
                    content = (self.description);
                meta
                    name = "author"
                    content = "Hera Chamorro";

                // # OpenGraph bullshit
                meta
                    name = "og:url"
                    content = (self.url);
                meta
                    name = "og:title"
                    content = (self.title);
                meta
                    name = "og:description"
                    content = (self.description);
                // TODO: OpenGraph Image
                (self.og_type)

                // // Preload stuff
                // link
                //     rel = "preload"
                //     href = "/styles/main.css"
                //     as="style"
                //     type="mime/css"
                //     crossorigin = "anonymous";
                // link
                //     rel = "preload"
                //     href = "/static/fonts/berkeley-mono/regular.woff2"
                //     as = "font"
                //     type = "font/woff2"
                //     crossorigin = "anonymous";

                // link
                //     rel = "preload"
                //     href = "/static/fonts/berkeley-mono/bold.woff2"
                //     as = "font"
                //     type = "font/woff2"
                //     crossorigin = "anonymous";

                // link
                //     rel = "preload"
                //     href = "/static/fonts/berkeley-mono/italic.woff2"
                //     as = "font"
                //     type = "font/woff2"
                //     crossorigin = "anonymous";

                // link
                //     rel = "preload"
                //     href = "/static/fonts/berkeley-mono/bold-italic.woff2"
                //     as = "font"
                //     type = "font/woff2"
                //     crossorigin = "anonymous";

                link
                    rel = "stylesheet"
                    href = "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/7.0.1/css/all.min.css"
                    integrity = "sha512-2SwdPD6INVrV/lHTZbO2nodKhrnDdJK9/kg2XD1r9uGqPo1cUbujc+IYdlYdEErWNu69gVcYgdxlmVmzTWnetw=="
                    crossorigin = "anonymous"
                    referrerpolicy = "no-referrer";

                // Style stuff
                link
                    rel = "stylesheet"
                    href = "/styles/main.css"
                    crossorigin = "anonymous";
            }
        }
    }
}
