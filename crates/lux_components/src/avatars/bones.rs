use std::marker::PhantomData;

use bevy::prelude::*;

pub trait Bones: Default + Sized + Send + Sync + Clone {}

#[derive(Default, Clone)]
pub struct Root;
impl Bones for Root {}

#[derive(Default, Clone)]
pub struct Hips;
impl Bones for Hips {}

#[derive(Default, Clone)]
pub struct Spine;
impl Bones for Spine {}

#[derive(Default, Clone)]
pub struct Chest;
impl Bones for Chest {}

#[derive(Default, Clone)]
pub struct Neck;
impl Bones for Neck {}

#[derive(Default, Clone)]
pub struct Head;
impl Bones for Head {}

#[derive(Default, Clone, Debug, Component)]
pub struct Bone<T: Bones> {
    b: PhantomData<T>,
}
