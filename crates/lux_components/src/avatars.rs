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
            let Some(armature_id) = find_by_name_in_childs(&target, entity_id, &world) else {
                return;
            };
            let target = "Hips".into();
            if let Some(hips_id) = find_by_name_in_childs(&target, armature_id, &world) {
                world
                    .commands()
                    .entity(hips_id)
                    .insert(Bone::<Hips>::default());
                let target = "Spine".into();
                if let Some(spine_id) = find_by_name_in_childs(&target, hips_id, &world) {
                    world
                        .commands()
                        .entity(spine_id)
                        .insert(Bone::<Spine>::default());
                    let target = "Neck".into();
                    if let Some(neck_id) = find_by_name_in_childs(&target, spine_id, &world) {
                        world
                            .commands()
                            .entity(neck_id)
                            .insert(Bone::<Neck>::default());
                        let target = "Head".into();
                        if let Some(head_id) = find_by_name_in_childs(&target, neck_id, &world) {
                            world
                                .commands()
                                .entity(head_id)
                                .insert(Bone::<Head>::default());
                        }
                    }
                }
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
        let neck = app.world_mut().spawn(Name::new("Neck")).id();
        let head = app.world_mut().spawn(Name::new("Head")).id();
        app.world_mut().commands().entity(root).add_child(arma);
        app.world_mut().commands().entity(arma).add_child(hips);
        app.world_mut().commands().entity(hips).add_child(spine);
        app.world_mut().commands().entity(spine).add_child(neck);
        app.world_mut().commands().entity(neck).add_child(head);
        root
    }

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(AvatarPlugin);
        app
    }
}
