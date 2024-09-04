mod bone_assigner;
mod bones;

use bone_assigner::{find_armature, BONE_TREE};

use bevy::{
    ecs::component::{ComponentHooks, StorageType},
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
            if let Some(armature_id) = find_armature(entity_id, &world) {
                BONE_TREE.apply(armature_id, &mut world);
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bones::*;

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
        check_bone_name::<ArmL>(&mut app, "Arm.L");
        check_bone_name::<ArmR>(&mut app, "Arm.R");
        check_bone_name::<ForearmL>(&mut app, "Forearm.L");
        check_bone_name::<ForearmR>(&mut app, "Forearm.R");
        check_bone_name::<HandL>(&mut app, "Hand.L");
        check_bone_name::<HandR>(&mut app, "Hand.R");
        check_bone_name::<ThighL>(&mut app, "Thigh.L");
        check_bone_name::<ThighR>(&mut app, "Thigh.R");
        check_bone_name::<LegL>(&mut app, "Leg.L");
        check_bone_name::<LegR>(&mut app, "Leg.R");
        check_bone_name::<FootL>(&mut app, "Foot.L");
        check_bone_name::<FootR>(&mut app, "Foot.R");
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

        let arm_l = app.world_mut().spawn(Name::new("Arm.L")).id();
        let forearm_l = app.world_mut().spawn(Name::new("Forearm.L")).id();
        let hand_l = app.world_mut().spawn(Name::new("Hand.L")).id();
        let arm_r = app.world_mut().spawn(Name::new("Arm.R")).id();
        let forearm_r = app.world_mut().spawn(Name::new("Forearm.R")).id();
        let hand_r = app.world_mut().spawn(Name::new("Hand.R")).id();
        app.world_mut().commands().entity(chest).add_child(arm_l);
        app.world_mut()
            .commands()
            .entity(arm_l)
            .add_child(forearm_l);
        app.world_mut()
            .commands()
            .entity(forearm_l)
            .add_child(hand_l);
        app.world_mut().commands().entity(chest).add_child(arm_r);
        app.world_mut()
            .commands()
            .entity(arm_r)
            .add_child(forearm_r);
        app.world_mut()
            .commands()
            .entity(forearm_r)
            .add_child(hand_r);

        let thigh_l = app.world_mut().spawn(Name::new("Thigh.L")).id();
        let leg_l = app.world_mut().spawn(Name::new("Leg.L")).id();
        let foot_l = app.world_mut().spawn(Name::new("Foot.L")).id();
        let thigh_r = app.world_mut().spawn(Name::new("Thigh.R")).id();
        let leg_r = app.world_mut().spawn(Name::new("Leg.R")).id();
        let foot_r = app.world_mut().spawn(Name::new("Foot.R")).id();
        app.world_mut().commands().entity(hips).add_child(thigh_l);
        app.world_mut().commands().entity(thigh_l).add_child(leg_l);
        app.world_mut().commands().entity(leg_l).add_child(foot_l);
        app.world_mut().commands().entity(hips).add_child(thigh_r);
        app.world_mut().commands().entity(thigh_r).add_child(leg_r);
        app.world_mut().commands().entity(leg_r).add_child(foot_r);

        root
    }

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(AvatarPlugin);
        app
    }
}
