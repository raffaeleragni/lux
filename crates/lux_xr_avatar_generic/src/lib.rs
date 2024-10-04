mod avatar_assigner;

use bevy::prelude::*;

pub fn init(app: &mut App) {
    avatar_assigner::init(app);
}
