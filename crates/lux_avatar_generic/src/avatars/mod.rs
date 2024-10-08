#![allow(clippy::type_complexity)]

pub mod bones;

mod bone_assigner;

use bevy_mod_inverse_kinematics::{IkConstraint, InverseKinematicsPlugin};

use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use bevy_sync::SyncComponent;
use bones::{Bone, FootL, FootR, HandL, HandR, Head, Hips, Root, Target};

use lux_components::{ComponentEntityRef, LocalUser};

#[derive(Default)]
pub(crate) struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.sync_component::<AvatarGeneric>();
        app.add_plugins(InverseKinematicsPlugin);
        app.add_systems(Update, local_user_enters);
        app.add_systems(Update, local_user_exits);
        app.add_systems(
            Update,
            (copy_roation_target_head, orient_hips_to_head).chain(),
        );
    }
}

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct AvatarGeneric {
    distance_hips_to_head: f32,
}

impl Component for AvatarGeneric {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, id, _component_id| {
            bone_assigner::apply(id, &mut world);
        });
    }
}

#[allow(clippy::type_complexity)]
fn local_user_enters(
    mut cmd: Commands,
    q: Query<
        (
            &ComponentEntityRef<Bone<Root>>,
            &ComponentEntityRef<Target<Hips>>,
            &ComponentEntityRef<Target<Head>>,
            &ComponentEntityRef<Target<HandL>>,
            &ComponentEntityRef<Target<HandR>>,
            &ComponentEntityRef<Target<FootL>>,
            &ComponentEntityRef<Target<FootR>>,
        ),
        (With<AvatarGeneric>, Added<LocalUser>),
    >,
) {
    for cer in q.iter() {
        cmd.entity(cer.0.entity_id).try_insert(LocalUser);
        cmd.entity(cer.1.entity_id).try_insert(LocalUser);
        cmd.entity(cer.2.entity_id).try_insert(LocalUser);
        cmd.entity(cer.3.entity_id).try_insert(LocalUser);
        cmd.entity(cer.4.entity_id).try_insert(LocalUser);
        cmd.entity(cer.5.entity_id).try_insert(LocalUser);
        cmd.entity(cer.6.entity_id).try_insert(LocalUser);
    }
}

#[allow(clippy::type_complexity)]
fn local_user_exits(
    mut cmd: Commands,
    mut removed: RemovedComponents<LocalUser>,
    q: Query<
        (
            &ComponentEntityRef<Bone<Root>>,
            &ComponentEntityRef<Target<Hips>>,
            &ComponentEntityRef<Target<Head>>,
            &ComponentEntityRef<Target<HandL>>,
            &ComponentEntityRef<Target<HandR>>,
            &ComponentEntityRef<Target<FootL>>,
            &ComponentEntityRef<Target<FootR>>,
        ),
        With<AvatarGeneric>,
    >,
) {
    for entity in removed.read() {
        let Ok(res) = q.get(entity) else { continue };
        cmd.entity(res.0.entity_id).remove::<LocalUser>();
        cmd.entity(res.1.entity_id).remove::<LocalUser>();
        cmd.entity(res.2.entity_id).remove::<LocalUser>();
        cmd.entity(res.3.entity_id).remove::<LocalUser>();
        cmd.entity(res.4.entity_id).remove::<LocalUser>();
        cmd.entity(res.5.entity_id).remove::<LocalUser>();
        cmd.entity(res.6.entity_id).remove::<LocalUser>();
    }
}

fn orient_hips_to_head(
    av: Query<(
        &AvatarGeneric,
        &ComponentEntityRef<Bone<Hips>>,
        &ComponentEntityRef<Target<Head>>,
    )>,
    mut hips_tf: Query<&mut Transform, With<Bone<Hips>>>,
    head_tf: Query<&Transform, (Without<Bone<Hips>>, With<Target<Head>>)>,
) {
    for (av, hips_id, head_id) in av.iter() {
        if let Ok(head) = head_tf.get(head_id.entity_id) {
            if let Ok(mut hips) = hips_tf.get_mut(hips_id.entity_id) {
                let yonly = Quat::from_euler(
                    EulerRot::YXZ,
                    head.rotation.to_euler(EulerRot::YXZ).0,
                    0.0,
                    0.0,
                );
                hips.rotation = yonly;
                hips.translation = head.translation;
                hips.translation.y -= av.distance_hips_to_head;
            }
        }
    }
}

fn copy_roation_target_head(
    mut q: Query<(&mut Transform, &IkConstraint), With<Bone<Head>>>,
    src: Query<&Transform, (Without<Bone<Head>>, With<Target<Head>>)>,
) {
    for (mut tf, ik) in q.iter_mut() {
        if let Ok(s) = src.get(ik.target) {
            tf.rotation = s.rotation;
        }
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
        app.world_mut()
            .commands()
            .entity(root)
            .try_insert(AvatarGeneric::default());
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

        app.world_mut()
            .commands()
            .entity(root)
            .try_insert(LocalUser);
        app.update();

        assert!(
            compo_has::<Bone<Root>, LocalUser>(&mut app),
            "Bone<Root> missing LocalUser"
        );
        assert!(
            compo_has::<Target<Head>, LocalUser>(&mut app),
            "Target<Head> missing LocalUser"
        );
        assert!(
            compo_has::<Target<HandL>, LocalUser>(&mut app),
            "Target<HandL> missing LocalUser"
        );
        assert!(
            compo_has::<Target<HandR>, LocalUser>(&mut app),
            "Target<HandR> missing LocalUser"
        );
        assert!(
            compo_has::<Target<FootL>, LocalUser>(&mut app),
            "Target<FootR> missing LocalUser"
        );
        assert!(
            compo_has::<Target<FootR>, LocalUser>(&mut app),
            "Target<FootR> missing LocalUser"
        );

        app.world_mut()
            .commands()
            .entity(root)
            .remove::<LocalUser>();
        app.update();

        assert!(
            !compo_has::<Target<Head>, LocalUser>(&mut app),
            "Bone<Root> has LocalUser"
        );
        assert!(
            !compo_has::<Target<Head>, LocalUser>(&mut app),
            "Target<Head> has LocalUser"
        );
        assert!(
            !compo_has::<Target<HandL>, LocalUser>(&mut app),
            "Target<HandL> has LocalUser"
        );
        assert!(
            !compo_has::<Target<HandR>, LocalUser>(&mut app),
            "Target<HandR> has LocalUser"
        );
        assert!(
            !compo_has::<Target<FootL>, LocalUser>(&mut app),
            "Target<FootR> has LocalUser"
        );
        assert!(
            !compo_has::<Target<FootR>, LocalUser>(&mut app),
            "Target<FootR> has LocalUser"
        );
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
        let found_eid_target = q
            .iter(app.world())
            .next()
            .unwrap_or_else(|| panic!("target not found, name {}", name));
        // Every Avatar entity has references to the targets
        let mut q = app
            .world_mut()
            .query_filtered::<&ComponentEntityRef<Target<T>>, With<AvatarGeneric>>();
        let found_eid = q
            .iter(app.world())
            .next()
            .unwrap_or_else(|| panic!("Found no ComponentEntityRef for Target name: {}", name));
        assert_eq!(
            found_eid_target, found_eid.entity_id,
            "ComponentEntityRef for Target name: {}, wrong entity id: {}",
            name, found_eid_target
        );
    }

    fn compo_has<FROM: Component, TO: Component>(app: &mut App) -> bool {
        let mut q = app
            .world_mut()
            .query_filtered::<Entity, (With<FROM>, With<TO>)>();
        q.iter(app.world()).next().is_some()
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
