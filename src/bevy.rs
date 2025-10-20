use std::cmp::Ordering;

use bevy::ecs::{bundle::Bundle, system::EntityCommands};

/// Helper type that is used for inserting or removing something... Usually
/// some bundle from an entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modify<T> {
    /// Remove the `T`.
    Remove,
    /// Insert the `T`.
    Insert(T),
}

impl<T> Modify<T> {
    /// Create a new [`Modify`] inserting `value` if `cond` is [`true`].
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn insert_if(cond: bool, value: T) -> Modify<T> {
        if cond {
            Modify::Insert(value)
        } else {
            Modify::Remove
        }
    }

    /// Create a new [`Modify`] from an [`Option`].
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn insert_some(option: Option<T>) -> Modify<T> {
        match option {
            Some(value) => Modify::Insert(value),
            None => Modify::Remove,
        }
    }

    /// Create a new [`Modify`] from a [`Result`].
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn insert_ok<E>(result: Result<T, E>) -> Modify<T> {
        match result {
            Ok(value) => Modify::Insert(value),
            Err(..) => Modify::Remove,
        }
    }
}

impl<T: PartialOrd> PartialOrd for Modify<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Modify::Insert(lhs), Modify::Insert(rhs)) => lhs.partial_cmp(rhs),
            (Modify::Insert(..), Modify::Remove) => Some(Ordering::Greater),
            (Modify::Remove, Modify::Insert(..)) => Some(Ordering::Less),
            (Modify::Remove, Modify::Remove) => Some(Ordering::Equal),
        }
    }
}

impl<T: Ord> Ord for Modify<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Modify::Insert(lhs), Modify::Insert(rhs)) => lhs.cmp(rhs),
            (Modify::Insert(..), Modify::Remove) => Ordering::Greater,
            (Modify::Remove, Modify::Insert(..)) => Ordering::Less,
            (Modify::Remove, Modify::Remove) => Ordering::Equal,
        }
    }
}

/// Extension trait for [`EntityCommands`].
pub trait EntityCommandsExt {
    /// Given some [`Modify`], add or remove the bundle `M::Item` according to
    /// it's value.
    #[track_caller]
    fn modify<M>(&mut self, modify: M) -> &mut Self
    where
        M: IntoModify<Item: Bundle>;

    /// Given some [`Modify`], add or remove the bundle `M::Item` according to
    /// it's value.
    ///
    /// This is the "silent" variation of [`modify`].
    #[track_caller]
    fn try_modify<M>(&mut self, modify: M) -> &mut Self
    where
        M: IntoModify<Item: Bundle>;
}

impl EntityCommandsExt for EntityCommands<'_> {
    #[inline]
    fn modify<M>(&mut self, modify: M) -> &mut Self
    where
        M: IntoModify<Item: Bundle>,
    {
        match modify.into_modify() {
            Modify::Insert(bundle) => self.insert(bundle),
            Modify::Remove => self.remove::<M::Item>(),
        }
    }

    #[inline]
    fn try_modify<M>(&mut self, modify: M) -> &mut Self
    where
        M: IntoModify<Item: Bundle>,
    {
        match modify.into_modify() {
            Modify::Insert(bundle) => self.try_insert(bundle),
            Modify::Remove => self.try_remove::<M::Item>(),
        }
    }
}

/// Types that can be converted to a [`Modify`].
pub trait IntoModify {
    type Item: Sized;

    #[must_use]
    fn into_modify(self) -> Modify<Self::Item>;
}

impl<T> IntoModify for Modify<T> {
    type Item = T;

    #[inline]
    #[track_caller]
    fn into_modify(self) -> Modify<T> {
        self
    }
}

impl<T> IntoModify for (bool, T) {
    type Item = T;

    #[inline]
    #[track_caller]
    fn into_modify(self) -> Modify<Self::Item> {
        let (cond, value) = self;

        Modify::insert_if(cond, value)
    }
}

impl<T> IntoModify for Option<T> {
    type Item = T;

    #[inline]
    #[track_caller]
    fn into_modify(self) -> Modify<Self::Item> {
        Modify::insert_some(self)
    }
}

impl<T, E> IntoModify for Result<T, E> {
    type Item = T;

    #[inline]
    #[track_caller]
    fn into_modify(self) -> Modify<Self::Item> {
        Modify::insert_ok(self)
    }
}
