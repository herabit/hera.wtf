#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Djot<T: ?Sized>(pub T);

// pub fn render_djot(djot: &str, output: &mut String) {
//     let mut events = jotdown::
// }
