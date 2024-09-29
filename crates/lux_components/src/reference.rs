use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct ComponentEntityRef<T> {
    p: PhantomData<T>,
    pub entity_id: Entity,
}

impl<T> ComponentEntityRef<T> {
    pub fn new(entity_id: Entity) -> Self {
        ComponentEntityRef {
            p: PhantomData::<T>,
            entity_id,
        }
    }
}
