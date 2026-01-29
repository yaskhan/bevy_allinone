use bevy::prelude::*;

mod types;
mod follow;
mod waypoints;
mod collision;
mod fov;
mod shake;
mod bob;
mod state_offsets;
mod collision_lean;
mod lock;
mod zones;

pub use types::*;
pub use follow::*;
pub use waypoints::*;
pub use collision::*;
pub use fov::*;
pub use shake::*;
pub use bob::*;
pub use state_offsets::*;
pub use collision_lean::*;
pub use lock::*;
pub use zones::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ShakeQueue>()
            .register_type::<CameraController>()
            .register_type::<CameraState>()
            .register_type::<CameraWaypoint>()
            .register_type::<CameraWaypointTrack>()
            .register_type::<CameraWaypointFollower>()
            .register_type::<CameraShakeInstance>()
            .register_type::<CameraBobState>()
            .register_type::<CameraTargetState>()
            .register_type::<CameraZone>()
            .register_type::<CameraZoneTracker>()
            .add_systems(Update, (
                update_camera_state_offsets,
                update_target_marking,
                update_target_lock,
                update_camera_zones,
                apply_camera_zone_settings,
                update_camera_rotation,
            ).chain())
            .add_systems(Update, (
                update_camera_shake,
                update_camera_bob,
                update_camera_lean_collision,
                update_camera_follow,
                update_camera_waypoint_follow,
                handle_camera_collision,
                update_camera_fov,
                handle_camera_mode_switch,
            ).chain());
    }
}

pub fn spawn_camera(
    commands: &mut Commands,
    target: Entity,
) -> Entity {
    commands.spawn((
        Camera3d::default(),
        CameraController {
            follow_target: Some(target),
            ..default()
        },
        CameraState {
            current_distance: 4.0,
            ..default()
        },
        CameraBobState::default(),
        CameraTargetState::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ))
    .id()
}
