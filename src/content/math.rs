use crate::content::Content;
use bevy::ecs::component::Component;

/// A component for processing math.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub enum Math<C: Content + ?Sized = str> {
    Inline(C::Output),
    Block(C::Output),
}
