use std::marker::PhantomData;

use bevy::prelude::*;

pub trait Bones: Default + Sized + Send + Sync + Clone {}

macro_rules! bone {
    ($name:ident) => {
        #[derive(Default, Clone, Debug)]
        pub struct $name;
        impl Bones for $name {}
    };
}

#[derive(Default, Clone, Debug, Component)]
pub struct Bone<T: Bones> {
    b: PhantomData<T>,
}

#[derive(Default, Clone, Debug, Component)]
pub struct Target<T: Bones> {
    b: PhantomData<T>,
}

bone!(Root);
bone!(Hips);
bone!(Spine);
bone!(Chest);
bone!(Neck);
bone!(Head);
bone!(ArmL);
bone!(ArmR);
bone!(ForearmL);
bone!(ForearmR);
bone!(HandL);
bone!(HandR);
bone!(ThighL);
bone!(ThighR);
bone!(LegL);
bone!(LegR);
bone!(FootL);
bone!(FootR);
