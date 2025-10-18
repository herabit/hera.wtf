#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FrontMatter<'a> {
    /// The matter that was read from the frontmatter.
    pub content: &'a str,
    /// The language of the frontmatter.
    pub language: Option<&'a str>,
    /// The rest of the document.
    pub rest: &'a str,
}

impl<'a> FrontMatter<'a> {
    /// Extracts the front matter from a provided string.
    pub fn parse(input: &'a str) -> Result<Self, &'static str> {
        // NOTE: `\r` is handled by the later `language.trim()`.
        let (start, input) = input.split_once("\n").unwrap_or((input, ""));

        let marker_len @ 3.. = start.bytes().take_while(|b| *b == b'-').count() else {
            return Err("no marker");
        };

        let (marker, language) = match start.split_at_checked(marker_len).ok_or("invalid marker")? {
            (marker, "") => (marker, None),
            (marker, language) => (marker, Some(language)),
        };

        let (content, rest) = input
            .match_indices(marker)
            .map(|(index, _)| index)
            .filter_map(|index| {
                let (content, rest) = input.split_at_checked(index)?;

                let content = match content {
                    "" => content,
                    mut content => {
                        content = content.strip_suffix("\n")?;

                        content.strip_suffix("\r").unwrap_or(content)
                    }
                };

                let rest = match rest.get(marker.len()..)? {
                    rest @ "" => rest,
                    mut rest => {
                        rest = rest.strip_prefix("\r").unwrap_or(rest);

                        rest.strip_prefix("\n")?
                    }
                };

                Some((content, rest))
            })
            .next()
            .ok_or("no marker")?;

        Ok(Self {
            content,
            language,
            rest,
        })
    }
}
