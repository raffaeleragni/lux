pub mod bones;

mod bone_assigner;

use bevy_mod_inverse_kinematics::InverseKinematicsPlugin;

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
        app.add_plugins(InverseKinematicsPlugin);
    }
}

#[derive(Reflect)]
#[reflect(Component)]
pub struct Avatar;

impl Component for Avatar {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, id, _component_id| {
            bone_assigner::apply(id, &mut world);
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

        check_bone_name::<Root>(&mut app, "Armature");
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

        check_target_name::<Head>(&mut app, "Head");
        check_target_name::<HandL>(&mut app, "Hand.L");
        check_target_name::<HandR>(&mut app, "Hand.R");
        check_target_name::<FootL>(&mut app, "Foot.L");
        check_target_name::<FootR>(&mut app, "Foot.R");
    }

    fn check_bone_name<T: 'static + Bones + Send + Sync>(app: &mut App, name: &'static str) {
        let mut q = app.world_mut().query_filtered::<&Name, With<Bone<T>>>();
        let found = q
            .iter(app.world())
            .next()
            .unwrap_or_else(|| panic!("Expected Bone<> with Name: {}", name));
        assert_eq!(
            found,
            &Name::new(name),
            "While checking Bone<> for {}",
            name
        );
    }

    fn check_target_name<T: 'static + Bones + Send + Sync>(app: &mut App, name: &'static str) {
        let mut q = app
            .world_mut()
            .query_filtered::<(Entity, &Name), With<Bone<T>>>();
        let found = q.iter(app.world()).next().unwrap();
        assert_eq!(found.1, &Name::new(name), "bone not found");
        // cannot have a target in the same bone of the bone, must be another entity
        assert!(
            app.world().entity(found.0).get::<Target<T>>().is_none(),
            "target is in bone"
        );
        let mut q = app.world_mut().query_filtered::<Entity, With<Target<T>>>();
        let found = q.iter(app.world()).next();
        assert!(found.is_some(), "target not found, name {}", name);
    }

    fn add_armature(app: &mut App) -> Entity {
        let root = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Root")))
            .id();
        let arma = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Armature")))
            .id();
        let hips = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Hips")))
            .id();
        let spine = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Spine")))
            .id();
        let chest = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Chest")))
            .id();
        let neck = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Neck")))
            .id();
        let head = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Head")))
            .id();
        app.world_mut().commands().entity(root).add_child(arma);
        app.world_mut().commands().entity(arma).add_child(hips);
        app.world_mut().commands().entity(hips).add_child(spine);
        app.world_mut().commands().entity(spine).add_child(chest);
        app.world_mut().commands().entity(chest).add_child(neck);
        app.world_mut().commands().entity(neck).add_child(head);

        let arm_l = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Arm.L")))
            .id();
        let forearm_l = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Forearm.L")))
            .id();
        let hand_l = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Hand.L")))
            .id();
        let arm_r = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Arm.R")))
            .id();
        let forearm_r = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Forearm.R")))
            .id();
        let hand_r = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Hand.R")))
            .id();
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

        let thigh_l = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Thigh.L")))
            .id();
        let leg_l = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Leg.L")))
            .id();
        let foot_l = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Foot.L")))
            .id();
        let thigh_r = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Thigh.R")))
            .id();
        let leg_r = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Leg.R")))
            .id();
        let foot_r = app
            .world_mut()
            .spawn((GlobalTransform::default(), Name::new("Foot.R")))
            .id();
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
