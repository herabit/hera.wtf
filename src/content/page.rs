use serde::{Deserialize, Serialize};

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
    /// The series of a page, if any.
    #[serde(default)]
    pub series: Option<PageSeries>,
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

/// Metadata for a page's series, if any.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize, Serialize)]
pub struct PageSeries {
    pub series: String,
    pub entry: usize,
}
