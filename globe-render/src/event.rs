use std::marker::PhantomData;

use bevy::prelude::*;

pub struct Created;
pub struct Updated;
pub struct Deleted;
pub struct Clicked;

#[derive(Event)]
pub struct Event<T, K, D = ()> {
    pub data: D,
    event: PhantomData<T>,
    kind: PhantomData<K>,
}

impl<T, K> Default for Event<T, K, ()> {
    fn default() -> Self {
        Self {
            data: (),
            event: PhantomData,
            kind: PhantomData,
        }
    }
}

impl<T, K, D> From<D> for Event<T, K, D> {
    fn from(value: D) -> Self {
        Self {
            data: value,
            event: PhantomData,
            kind: PhantomData,
        }
    }
}
