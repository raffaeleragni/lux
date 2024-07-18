use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use bevy_sync::SyncExclude;
use std::marker::PhantomData;

/// This component marks that a component is controlled by another component
/// This will allow to span or control how many controlling components there
/// are and if they are zero or 1+, to delete or sidespawn a SyncExclude<C>.
/// This will not be synched as it is a local only blocker for sending data.
#[derive(Default)]
pub struct ControlledBy<C: Component + Default, F: Component> {
    c: PhantomData<C>,
    f: PhantomData<F>,
}

#[derive(Component)]
struct ControlledByCounter {
    counter: usize,
}

impl<C: Component + Default, F: Component> Component for ControlledBy<C, F> {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity_id, _component_id| {
            let entity_mut = &mut world.entity_mut(entity_id);
            let counter = entity_mut.get_mut::<ControlledByCounter>();
            if counter.map(|mut c| c.counter += 1).is_none() {
                world
                    .commands()
                    .entity(entity_id)
                    .insert(ControlledByCounter { counter: 1 });
            }
            world
                .commands()
                .entity(entity_id)
                .insert(SyncExclude::<C>::default());
        });
        hooks.on_remove(|mut world, entity_id, _component_id| {
            let entity_mut = &mut world.entity_mut(entity_id);
            let counter = entity_mut.get_mut::<ControlledByCounter>();
            let counter = counter
                .filter(|c| c.counter > 0)
                .map(|mut c| {
                    c.counter -= 1;
                    c.counter
                })
                .unwrap_or(0);
            if counter == 0 {
                world
                    .commands()
                    .entity(entity_id)
                    .remove::<ControlledByCounter>()
                    .remove::<SyncExclude<C>>();
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::render::primitives::Aabb;
    use bevy_sync::SyncExclude;

    #[test]
    fn test_controlled_by_system_adds_component() {
        let mut app = App::new();
        let spawn = app
            .world_mut()
            .spawn((Name::new(""), ControlledBy::<Name, Aabb>::default()))
            .id();
        app.update();

        let spawn = app.world().entity(spawn);
        assert!(spawn.get::<SyncExclude<Name>>().is_some());
    }

    #[test]
    fn test_controlled_by_system_removes_component() {
        let mut app = App::new();
        let spawn = app
            .world_mut()
            .spawn((Name::new(""), ControlledBy::<Name, Aabb>::default()))
            .id();
        app.update();
        app.world_mut()
            .commands()
            .entity(spawn)
            .remove::<ControlledBy<Name, Aabb>>();
        app.update();

        let spawn = app.world().entity(spawn);
        assert!(spawn.get::<SyncExclude<Name>>().is_none());
    }

    #[ignore = "unimplemented case"]
    #[test]
    fn test_controlled_by_system_not_removed_with_multiple_controlled_by() {
        let mut app = App::new();
        let spawn = app
            .world_mut()
            .spawn((
                Name::new(""),
                ControlledBy::<Name, Aabb>::default(),
                ControlledBy::<Name, Visibility>::default(),
            ))
            .id();
        app.update();
        app.world_mut()
            .commands()
            .entity(spawn)
            .remove::<ControlledBy<Name, Aabb>>();
        app.update();

        let spawn = app.world().entity(spawn);
        assert!(spawn.get::<SyncExclude<Name>>().is_some());
    }

    #[ignore = "unimplemented case"]
    #[test]
    fn test_controlled_by_system_removed_with_multiple_controlled_by() {
        let mut app = App::new();
        let spawn = app
            .world_mut()
            .spawn((
                Name::new(""),
                ControlledBy::<Name, Aabb>::default(),
                ControlledBy::<Name, Visibility>::default(),
            ))
            .id();
        app.update();
        app.world_mut()
            .commands()
            .entity(spawn)
            .remove::<ControlledBy<Name, Aabb>>();
        app.world_mut()
            .commands()
            .entity(spawn)
            .remove::<ControlledBy<Name, Visibility>>();
        app.update();

        let spawn = app.world().entity(spawn);
        assert!(spawn.get::<SyncExclude<Name>>().is_none());
    }
}
