use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, pbr::PbrPlugin, prelude::*,
    render::mesh::skinning::SkinnedMeshInverseBindposes, state::app::StatesPlugin,
};

pub fn init(app: &mut App) {
    app.add_plugins(StatesPlugin);
    app.add_plugins(AssetPlugin::default());
    app.init_asset::<Scene>();
    app.init_asset::<Shader>();
    app.init_asset::<Mesh>();
    app.init_asset::<Image>();
    app.init_asset::<AudioSource>();
    app.init_asset::<SkinnedMeshInverseBindposes>();
    app.add_plugins(PbrPlugin::default());
    app.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
    );
}
