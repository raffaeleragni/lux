use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_sync::SyncComponent;

#[derive(Component, Reflect)]
pub struct Avatar;

#[derive(Default)]
pub(crate) struct AvatarPlugin;

enum Bones {
    Root,
    Hips,
    Spine,
    Neck,
    Head,
}

#[derive(Default)]
struct Bone<Bones> {
    b: PhantomData<Bones>,
}

impl Plugin for AvatarPlugin {
    fn build(&self, _app: &mut App) {
        // app.sync_component::<Avatar>();
    }
}

fn find_by_name_in_childs(
    target: &Name,
    start: Entity,
    q_name: &Query<&Name>,
    q_child: &Query<&Children>,
) -> Option<Entity> {
    if let Ok(q) = q_child.get(start) {
        for child in q {
            let Ok(name) = q_name.get(*child) else {
                continue;
            };
            if name == target {
                return Some(*child);
            }
        }
    }
    None
    // find Hips
    // top becomes Root
    // then walk down...
    // find the others and take from the remaining
    // stop when the remaining is 0
    // or all have been visited
}
