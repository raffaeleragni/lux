use std::sync::LazyLock;

use crate::avatars::bones::*;
use bevy::{ecs::world::DeferredWorld, prelude::*};

pub struct BoneTree {
    applier: Box<dyn BoneApplier + Send + Sync>,
    children: Vec<BoneTree>,
}

impl BoneTree {
    pub fn apply(&self, parent_id: Entity, world: &mut DeferredWorld) {
        if let Some(child_id) = self.applier.apply(parent_id, world) {
            for child in self.children.iter() {
                child.apply(child_id, world);
            }
        }
    }
}

trait BoneApplier {
    fn apply(&self, parent_id: Entity, world: &mut DeferredWorld) -> Option<Entity>;
}

struct BonePair<T: Bones + 'static> {
    name: &'static str,
    compo: Bone<T>,
}

impl<T: Bones> BoneApplier for BonePair<T> {
    fn apply(&self, parent_id: Entity, world: &mut DeferredWorld) -> Option<Entity> {
        if let Some(found_id) = find_by_name_in_childs(&self.name.into(), parent_id, world) {
            world.commands().entity(found_id).insert(self.compo.clone());
            return Some(found_id);
        }
        warn!("Could not find bone: {}", self.name);
        None
    }
}
pub fn find_armature(entity_id: Entity, world: &DeferredWorld) -> Option<Entity> {
    let target = "Armature".into();
    let Some(armature_id) = find_by_name_in_childs(&target, entity_id, world) else {
        warn!("Could not find Armature");
        return None;
    };
    Some(armature_id)
}

pub static BONE_TREE: LazyLock<BoneTree> = LazyLock::new(|| {
    let hips = BonePair {
        name: "Hips",
        compo: Bone::<Hips>::default(),
    };
    let spine = BonePair {
        name: "Spine",
        compo: Bone::<Spine>::default(),
    };
    let chest = BonePair {
        name: "Chest",
        compo: Bone::<Chest>::default(),
    };
    let neck = BonePair {
        name: "Neck",
        compo: Bone::<Neck>::default(),
    };
    let head = BonePair {
        name: "Head",
        compo: Bone::<Head>::default(),
    };
    BoneTree {
        applier: Box::new(hips),
        children: vec![BoneTree {
            applier: Box::new(spine),
            children: vec![BoneTree {
                applier: Box::new(chest),
                children: vec![BoneTree {
                    applier: Box::new(neck),
                    children: vec![BoneTree {
                        applier: Box::new(head),
                        children: vec![],
                    }],
                }],
            }],
        }],
    }
});

fn find_by_name_in_childs(target: &Name, start: Entity, world: &DeferredWorld) -> Option<Entity> {
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
