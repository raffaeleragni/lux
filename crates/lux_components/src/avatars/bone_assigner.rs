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
    let arm_l = BonePair {
        name: "Arm.L",
        compo: Bone::<ArmL>::default(),
    };
    let forearm_l = BonePair {
        name: "Forearm.L",
        compo: Bone::<ForearmL>::default(),
    };
    let hand_l = BonePair {
        name: "Hand.L",
        compo: Bone::<HandL>::default(),
    };
    let arm_r = BonePair {
        name: "Arm.R",
        compo: Bone::<ArmR>::default(),
    };
    let forearm_r = BonePair {
        name: "Forearm.R",
        compo: Bone::<ForearmR>::default(),
    };
    let hand_r = BonePair {
        name: "Hand.R",
        compo: Bone::<HandR>::default(),
    };
    let thigh_l = BonePair {
        name: "Thigh.L",
        compo: Bone::<ThighL>::default(),
    };
    let leg_l = BonePair {
        name: "Leg.L",
        compo: Bone::<LegL>::default(),
    };
    let foot_l = BonePair {
        name: "Foot.L",
        compo: Bone::<FootL>::default(),
    };
    let thigh_r = BonePair {
        name: "Thigh.R",
        compo: Bone::<ThighR>::default(),
    };
    let leg_r = BonePair {
        name: "Leg.R",
        compo: Bone::<LegR>::default(),
    };
    let foot_r = BonePair {
        name: "Foot.R",
        compo: Bone::<FootR>::default(),
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
