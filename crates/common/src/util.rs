use std::{ops::Deref, sync::Arc};

pub enum Ptr<T: 'static> {
    Static(&'static T),
    Atomic(Arc<T>),
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(static_ref) => Self::Static(static_ref),
            Self::Atomic(arc) => Self::Atomic(arc.clone()),
        }
    }
}

impl<T> AsRef<T> for Ptr<T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Static(static_ref) => static_ref,
            Self::Atomic(arc) => arc.as_ref(),
        }
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Static(static_ref) => static_ref,
            Self::Atomic(arc) => arc.as_ref(),
        }
    }
}
