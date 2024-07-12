use bevy::{
    app::{App, Startup, Update},
    ecs::{
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    math::vec3,
    time::Time,
    transform::components::Transform,
};
use bevy_mod_openxr::{helper_traits::ToQuat, resources::OxrViews};
use bevy_mod_xr::session::XrTrackingRoot;
use bevy_xr_utils::xr_utils_actions::{
    ActiveSet, XRUtilsAction, XRUtilsActionSet, XRUtilsActionState, XRUtilsActionSystemSet,
    XRUtilsBinding,
};

pub fn init(app: &mut App) {
    app.add_systems(
        Startup,
        create_action_entities.before(XRUtilsActionSystemSet::CreateEvents),
    )
    .add_systems(Update, handle_flight_input);
}

fn create_action_entities(mut commands: Commands) {
    let set = commands
        .spawn((
            XRUtilsActionSet {
                name: "flight".into(),
                pretty_name: "pretty flight set".into(),
                priority: u32::MIN,
            },
            ActiveSet,
        ))
        .id();
    let action = commands
        .spawn((XRUtilsAction {
            action_name: "flight_input".into(),
            localized_name: "flight_input_localized".into(),
            action_type: bevy_mod_xr::actions::ActionType::Vector,
        },))
        .id();
    let binding_index = commands
        .spawn(XRUtilsBinding {
            profile: "/interaction_profiles/valve/index_controller".into(),
            binding: "/user/hand/right/input/thumbstick".into(),
        })
        .id();
    let binding_touch = commands
        .spawn(XRUtilsBinding {
            profile: "/interaction_profiles/oculus/touch_controller".into(),
            binding: "/user/hand/right/input/thumbstick".into(),
        })
        .id();
    commands.entity(action).add_child(binding_index);
    commands.entity(action).add_child(binding_touch);
    commands.entity(set).add_child(action);
}

fn handle_flight_input(
    action_query: Query<&XRUtilsActionState>,
    mut oxr_root: Query<&mut Transform, With<XrTrackingRoot>>,
    time: Res<Time>,
    views: ResMut<OxrViews>,
) {
    for state in action_query.iter() {
        if let XRUtilsActionState::Vector(vector_state) = state {
            let input_vector = vec3(
                vector_state.current_state[0],
                0.0,
                -vector_state.current_state[1],
            );
            let speed = 5.0;
            let root = oxr_root.get_single_mut();
            if let Ok(mut root_position) = root {
                let view = views.first();
                match view {
                    Some(v) => {
                        let reference_quat = v.pose.orientation.to_quat();
                        let locomotion_vector = reference_quat.mul_vec3(input_vector);

                        root_position.translation +=
                            locomotion_vector * speed * time.delta_seconds();
                    }
                    None => return,
                }
            }
        }
    }
}
