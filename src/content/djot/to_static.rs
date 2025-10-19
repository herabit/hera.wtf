//! Tools for expanding lifetimes.
//!
//! This is not ideal, as it does mean allocating, and often enough leaking memory,
//! however, the strings that are leaked seldom long, and we do not expect this to
//! be a long running program.
//!
//! Is it ideal? No. However I don't feel like writing our own djot parser, nor
//! do I feel like forking `jotdown`.

use jotdown::{AttributeKind, AttributeValue, Attributes, Container, Event};
use std::{borrow::Cow, mem};

pub fn event<'a>(e: Event<'a>) -> Event<'static> {
    use Event as E;

    match e {
        E::Start(c, a) => E::Start(container(c), attributes(a)),
        E::End(c) => E::End(container(c)),
        E::Str(s) => E::Str(match s {
            Cow::Borrowed(s) => Cow::Owned(s.into()),
            Cow::Owned(s) => Cow::Owned(s),
        }),
        // Ideal? No. Works? Yes.
        E::FootnoteReference(s) => E::FootnoteReference(Box::leak(Box::from(s))),
        E::Symbol(s) => E::Symbol(match s {
            Cow::Borrowed(s) => Cow::Owned(s.into()),
            Cow::Owned(s) => Cow::Owned(s),
        }),
        E::LeftSingleQuote => E::LeftSingleQuote,
        E::RightSingleQuote => E::RightSingleQuote,
        E::LeftDoubleQuote => E::LeftDoubleQuote,
        E::RightDoubleQuote => E::RightDoubleQuote,
        E::Ellipsis => E::Ellipsis,
        E::EnDash => E::EnDash,
        E::EmDash => E::EmDash,
        E::NonBreakingSpace => E::NonBreakingSpace,
        E::Softbreak => E::Softbreak,
        E::Hardbreak => E::Hardbreak,
        E::Escape => E::Escape,
        E::Blankline => E::Blankline,
        E::ThematicBreak(a) => E::ThematicBreak(attributes(a)),
        E::Attributes(a) => E::Attributes(attributes(a)),
    }
}

pub fn container<'a>(c: Container<'a>) -> Container<'static> {
    use Container as C;

    match c {
        C::Blockquote => C::Blockquote,
        C::List { kind, tight } => C::List { kind, tight },
        C::ListItem => C::ListItem,
        C::TaskListItem { checked } => C::TaskListItem { checked },
        C::DescriptionList => C::DescriptionList,
        C::DescriptionDetails => C::DescriptionDetails,
        C::Footnote { label } => C::Footnote {
            // Ideal? No. Works? Yes.
            label: Box::leak(Box::from(label)),
        },
        C::Table => C::Table,
        C::TableRow { head } => C::TableRow { head },
        C::Section { id } => C::Section {
            id: match id {
                Cow::Borrowed(id) => Cow::Owned(id.into()),
                Cow::Owned(id) => Cow::Owned(id),
            },
        },
        C::Div { class } => C::Div {
            // Ideal? No. Works? Yes.
            class: Box::leak(Box::from(class)),
        },
        C::Paragraph => C::Paragraph,
        C::Heading {
            level,
            has_section,
            id,
        } => C::Heading {
            level,
            has_section,
            id: match id {
                Cow::Borrowed(id) => Cow::Owned(id.into()),
                Cow::Owned(id) => Cow::Owned(id),
            },
        },
        C::TableCell { alignment, head } => C::TableCell { alignment, head },
        C::Caption => C::Caption,
        C::DescriptionTerm => C::DescriptionTerm,
        C::LinkDefinition { label } => C::LinkDefinition {
            // Ideal? No. Works? Yes.
            label: Box::leak(Box::from(label)),
        },
        C::RawBlock { format } => C::RawBlock {
            // Ideal? No. Works? Yes.
            format: Box::leak(Box::from(format)),
        },
        C::CodeBlock { language } => C::CodeBlock {
            // Ideal? No. Works? Yes.
            language: Box::leak(Box::from(language)),
        },
        C::Span => C::Span,
        C::Link(link, link_type) => C::Link(
            match link {
                Cow::Borrowed(link) => Cow::Owned(link.into()),
                Cow::Owned(link) => Cow::Owned(link),
            },
            link_type,
        ),
        C::Image(link, span_link_type) => C::Image(
            match link {
                Cow::Borrowed(link) => Cow::Owned(link.into()),
                Cow::Owned(link) => Cow::Owned(link),
            },
            span_link_type,
        ),
        C::Verbatim => C::Verbatim,
        C::Math { display } => C::Math { display },
        C::RawInline { format } => C::RawInline {
            // Ideal? No. Works? Yes.
            format: Box::leak(Box::from(format)),
        },
        C::Subscript => C::Subscript,
        C::Superscript => C::Superscript,
        C::Insert => C::Insert,
        C::Delete => C::Delete,
        C::Strong => C::Strong,
        C::Emphasis => C::Emphasis,
        C::Mark => C::Mark,
    }
}

pub fn attribute_value<'a>(v: AttributeValue<'a>) -> AttributeValue<'static> {
    // Ideal? No. Works? Yes.
    v.parts().collect::<String>().replace("\\", "\\\\").into()
}

pub fn attribute_kind<'a>(k: AttributeKind<'a>) -> AttributeKind<'static> {
    use AttributeKind as K;

    match k {
        K::Class => K::Class,
        K::Id => K::Id,
        K::Pair { key } => K::Pair {
            // Ideal? No. Works? Yes.
            key: Box::leak(Box::from(key)),
        },
        K::Comment => K::Comment,
    }
}

pub fn attributes<'a>(mut a: Attributes<'a>) -> Attributes<'static> {
    let buffer = mem::take(&mut *a);

    // `a` now contains an empty vec, so no need to run any destructor.
    mem::forget(a);

    // Ideal? No. Works? Yes.
    buffer
        .into_iter()
        .map(|(kind, value)| (attribute_kind(kind), attribute_value(value)))
        .collect()
}
