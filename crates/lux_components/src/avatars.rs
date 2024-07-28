use std::marker::PhantomData;

use bevy::{
    ecs::{
        component::{ComponentHooks, StorageType},
        world::DeferredWorld,
    },
    prelude::*,
};

#[derive(Reflect)]
pub struct Avatar;

impl Component for Avatar {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity_id, _component_id| {
            let target = "Armature".into();
            let Some(armature_id) = find_by_name_in_childs2(&target, entity_id, &world) else {
                return;
            };
            let target = "Hips".into();
            if let Some(hips_id) = find_by_name_in_childs2(&target, armature_id, &world) {
                world
                    .commands()
                    .entity(hips_id)
                    .insert(Bone::<Hips>::default());
            }
        });
    }
}

#[derive(Default)]
pub(crate) struct AvatarPlugin;

trait Bones {}

#[derive(Default)]
struct Root;
impl Bones for Root {}

#[derive(Default)]
struct Hips;
impl Bones for Hips {}

#[derive(Default)]
struct Spine;
impl Bones for Spine {}

#[derive(Default)]
struct Neck;
impl Bones for Neck {}

#[derive(Default)]
struct Head;
impl Bones for Head {}

#[derive(Default, Component)]
struct Bone<T: Bones> {
    b: PhantomData<T>,
}

impl Plugin for AvatarPlugin {
    fn build(&self, _app: &mut App) {
        // app.sync_component::<Avatar>();
    }
}

fn find_by_name_in_childs2(target: &Name, start: Entity, world: &DeferredWorld) -> Option<Entity> {
    let mut queue = vec![start];
    while !queue.is_empty() {
        let next = queue.pop()?;
        let Some(childs) = world.entity(next).get::<Children>() else {
            continue;
        };
        for child in childs {
            queue.push(*child);
            let Some(name) = world.entity(*child).get::<Name>() else {
                continue;
            };
            if name == target {
                return Some(*child);
            }
        }
    }
    None
}
