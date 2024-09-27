use std::sync::LazyLock;

use crate::avatars::bones::*;
use bevy::{ecs::world::DeferredWorld, prelude::*};
use bevy_mod_inverse_kinematics::IkConstraint;

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
    target: Option<Target<T>>,
}

impl<T: Bones> BoneApplier for BonePair<T> {
    fn apply(&self, parent_id: Entity, world: &mut DeferredWorld) -> Option<Entity> {
        if let Some(found_id) = find_by_name_in_childs(&self.name.into(), parent_id, world) {
            let tf = world.get::<GlobalTransform>(found_id)?;
            let pos = tf.translation();
            let mut cmds = world.commands();
            cmds.entity(found_id).insert(self.compo.clone());
            if let Some(target) = self.target.as_ref() {
                let etid = cmds
                    .spawn((
                        SpatialBundle {
                            transform: Transform{
                                translation: pos,
                                ..default()
                            },
                            ..default()
                        },
                        target.clone(),
                    ))
                    .id();
                cmds.entity(found_id).insert(IkConstraint {
                    chain_length: 2,
                    iterations: 20,
                    target: etid,
                    pole_target: None,
                    pole_angle: 0.0,
                    enabled: true,
                });
            }
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
        target: None,
    };
    let spine = BonePair {
        name: "Spine",
        compo: Bone::<Spine>::default(),
        target: None,
    };
    let chest = BonePair {
        name: "Chest",
        compo: Bone::<Chest>::default(),
        target: None,
    };
    let neck = BonePair {
        name: "Neck",
        compo: Bone::<Neck>::default(),
        target: None,
    };
    let head = BonePair {
        name: "Head",
        compo: Bone::<Head>::default(),
        target: Some(Target::<Head>::default()),
    };
    let arm_l = BonePair {
        name: "Arm.L",
        compo: Bone::<ArmL>::default(),
        target: None,
    };
    let forearm_l = BonePair {
        name: "Forearm.L",
        compo: Bone::<ForearmL>::default(),
        target: None,
    };
    let hand_l = BonePair {
        name: "Hand.L",
        compo: Bone::<HandL>::default(),
        target: Some(Target::<HandL>::default()),
    };
    let arm_r = BonePair {
        name: "Arm.R",
        compo: Bone::<ArmR>::default(),
        target: None,
    };
    let forearm_r = BonePair {
        name: "Forearm.R",
        compo: Bone::<ForearmR>::default(),
        target: None,
    };
    let hand_r = BonePair {
        name: "Hand.R",
        compo: Bone::<HandR>::default(),
        target: Some(Target::<HandR>::default()),
    };
    let thigh_l = BonePair {
        name: "Thigh.L",
        compo: Bone::<ThighL>::default(),
        target: None,
    };
    let leg_l = BonePair {
        name: "Leg.L",
        compo: Bone::<LegL>::default(),
        target: None,
    };
    let foot_l = BonePair {
        name: "Foot.L",
        compo: Bone::<FootL>::default(),
        target: Some(Target::<FootL>::default()),
    };
    let thigh_r = BonePair {
        name: "Thigh.R",
        compo: Bone::<ThighR>::default(),
        target: None,
    };
    let leg_r = BonePair {
        name: "Leg.R",
        compo: Bone::<LegR>::default(),
        target: None,
    };
    let foot_r = BonePair {
        name: "Foot.R",
        compo: Bone::<FootR>::default(),
        target: Some(Target::<FootR>::default()),
    };
    BoneTree {
        applier: Box::new(hips),
        children: vec![
            BoneTree {
                applier: Box::new(thigh_l),
                children: vec![BoneTree {
                    applier: Box::new(leg_l),
                    children: vec![BoneTree {
                        applier: Box::new(foot_l),
                        children: vec![],
                    }],
                }],
            },
            BoneTree {
                applier: Box::new(thigh_r),
                children: vec![BoneTree {
                    applier: Box::new(leg_r),
                    children: vec![BoneTree {
                        applier: Box::new(foot_r),
                        children: vec![],
                    }],
                }],
            },
            BoneTree {
                applier: Box::new(spine),
                children: vec![BoneTree {
                    applier: Box::new(chest),
                    children: vec![
                        BoneTree {
                            applier: Box::new(neck),
                            children: vec![BoneTree {
                                applier: Box::new(head),
                                children: vec![],
                            }],
                        },
                        BoneTree {
                            applier: Box::new(arm_l),
                            children: vec![BoneTree {
                                applier: Box::new(forearm_l),
                                children: vec![BoneTree {
                                    applier: Box::new(hand_l),
                                    children: vec![],
                                }],
                            }],
                        },
                        BoneTree {
                            applier: Box::new(arm_r),
                            children: vec![BoneTree {
                                applier: Box::new(forearm_r),
                                children: vec![BoneTree {
                                    applier: Box::new(hand_r),
                                    children: vec![],
                                }],
                            }],
                        },
                    ],
                }],
            },
        ],
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
