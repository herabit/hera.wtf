// use std::borrow::Cow;

// pub trait EventTypes {
//     type Math<'a>
//     where
//         Self: 'a;
// }

// impl EventTypes for () {
//     type Math<'a> = Cow<'a, str>;
// }

use std::borrow::Cow;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Math {
    Inline(Cow<'static, str>),
    Block(Cow<'static, str>),
}
