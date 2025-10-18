use std::{
    borrow::Cow,
    ffi::{CStr, OsStr},
    path::Path,
};

use bevy::ecs::component::Component;

// /// Stuff for rendering djot.
// pub mod djot;
/// process front matter.
pub mod front_matter;
// /// Stuff for generating head stuff.
// pub mod head;
/// Stuff for pages.
pub mod page;

/// Trait for types that can be used with [`Input`] or [`Output`].
pub trait Content: 'static {
    type Output: 'static + Sized;
}

impl<T: Sized + 'static> Content for T {
    type Output = T;
}

impl<T: Sized + 'static + Clone> Content for [T] {
    type Output = Cow<'static, [T]>;
}

impl Content for Path {
    type Output = Cow<'static, Path>;
}

impl Content for OsStr {
    type Output = Cow<'static, OsStr>;
}

impl Content for CStr {
    type Output = Cow<'static, CStr>;
}

/// A marker for things that are considered inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
#[repr(transparent)]
pub struct Input<C: Content + ?Sized>(pub C::Output);

/// A marker for things that are considered outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
#[repr(transparent)]
pub struct Output<C: Content + ?Sized>(pub C::Output);

/// The contents of some entity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Component)]
pub struct Contents(pub String);
