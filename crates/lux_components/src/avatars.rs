use std::{marker::PhantomData, sync::LazyLock};

use bevy::{
    ecs::{
        component::{ComponentHooks, StorageType},
        world::DeferredWorld,
    },
    prelude::*,
};
use bevy_sync::SyncComponent;

#[derive(Default)]
pub(crate) struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.sync_component::<Avatar>();
    }
}

#[derive(Reflect)]
pub struct Avatar;

impl Component for Avatar {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity_id, _component_id| {
            let target = "Armature".into();
            let Some(armature_id) = find_by_name_in_childs(&target, entity_id, &world) else {
                warn!("Could not find Armature");
                return;
            };
            BONE_TREE.apply(armature_id, &mut world);
        });
    }
}

struct BoneTree {
    applier: Box<dyn BoneApplier + Send + Sync>,
    children: Vec<BoneTree>,
}

impl BoneTree {
    fn apply(&self, parent_id: Entity, world: &mut DeferredWorld) {
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

static BONE_TREE: LazyLock<BoneTree> = LazyLock::new(|| {
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

trait Bones: Default + Sized + Send + Sync + Clone {}

#[derive(Default, Clone)]
struct Root;
impl Bones for Root {}

#[derive(Default, Clone)]
struct Hips;
impl Bones for Hips {}

#[derive(Default, Clone)]
struct Spine;
impl Bones for Spine {}

#[derive(Default, Clone)]
struct Chest;
impl Bones for Chest {}

#[derive(Default, Clone)]
struct Neck;
impl Bones for Neck {}

#[derive(Default, Clone)]
struct Head;
impl Bones for Head {}

#[derive(Default, Clone, Debug, Component)]
struct Bone<T: Bones> {
    b: PhantomData<T>,
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut app = app();
        let root = add_armature(&mut app);
        app.update();
        app.world_mut().commands().entity(root).insert(Avatar);
        app.update();

        check_bone_name::<Hips>(&mut app, "Hips");
        check_bone_name::<Spine>(&mut app, "Spine");
        check_bone_name::<Chest>(&mut app, "Chest");
        check_bone_name::<Neck>(&mut app, "Neck");
        check_bone_name::<Head>(&mut app, "Head");
    }

    fn check_bone_name<T: 'static + Bones + Send + Sync>(app: &mut App, name: &'static str) {
        let mut q = app.world_mut().query_filtered::<&Name, With<Bone<T>>>();
        let found = q.iter(app.world()).next().unwrap();
        assert_eq!(found, &Name::new(name));
    }

    fn add_armature(app: &mut App) -> Entity {
        let root = app.world_mut().spawn(Name::new("Root")).id();
        let arma = app.world_mut().spawn(Name::new("Armature")).id();
        let hips = app.world_mut().spawn(Name::new("Hips")).id();
        let spine = app.world_mut().spawn(Name::new("Spine")).id();
        let chest = app.world_mut().spawn(Name::new("Chest")).id();
        let neck = app.world_mut().spawn(Name::new("Neck")).id();
        let head = app.world_mut().spawn(Name::new("Head")).id();
        app.world_mut().commands().entity(root).add_child(arma);
        app.world_mut().commands().entity(arma).add_child(hips);
        app.world_mut().commands().entity(hips).add_child(spine);
        app.world_mut().commands().entity(spine).add_child(chest);
        app.world_mut().commands().entity(chest).add_child(neck);
        app.world_mut().commands().entity(neck).add_child(head);
        root
    }

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(AvatarPlugin);
        app
    }
}
