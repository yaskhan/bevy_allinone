use bevy::prelude::*;
use crate::camera::types::*;

pub struct CameraVehiclesPlugin;

impl Plugin for CameraVehiclesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VehicleCameraController>()
           .add_systems(Update, update_vehicle_camera);
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VehicleCameraController {
    pub vehicle_target: Option<Entity>,
    pub rotation_damping: f32,
    pub boost_distance_offset: f32,
    pub current_boost_offset: f32,
    pub boost_fade_speed: f32,
    pub is_first_person: bool,
    pub fp_offset: Vec3,
}

impl Default for VehicleCameraController {
    fn default() -> Self {
        Self {
            vehicle_target: None,
            rotation_damping: 3.0,
            boost_distance_offset: 2.0,
            current_boost_offset: 0.0,
            boost_fade_speed: 5.0,
            is_first_person: false,
            fp_offset: Vec3::new(0.0, 1.2, 0.5),
        }
    }
}

pub fn update_vehicle_camera(
    time: Res<Time>,
    mut query: Query<(&mut CameraController, &mut CameraState, &VehicleCameraController)>,
    vehicle_query: Query<&GlobalTransform>,
) {
    let dt = time.delta_secs();

    for (mut controller, mut state, vehicle_cam) in query.iter_mut() {
        if controller.mode != CameraMode::ThirdPerson && controller.mode != CameraMode::FirstPerson {
            // Only apply vehicle logic in these modes if intended
            // Or many vehicle cameras use a specific 'Vehicle' mode?
            // For now, let's assume it hooks into the existing modes.
        }

        let Some(vehicle_ent) = vehicle_cam.vehicle_target else { continue };
        let Ok(vehicle_xf) = vehicle_query.get(vehicle_ent) else { continue };

        // Vehicle camera logic:
        // 1. Follow rotation more strictly (Damping)
        // 2. Adjust distance based on boost
        
        let alpha = 1.0 - (-vehicle_cam.rotation_damping * dt).exp();
        
        // We want state.yaw/pitch to align with vehicle forward if not manually overridden
        // But CameraState is usually driven by input.
        // For vehicles, we often want "Auto-Center"
        
        // Placeholder for auto-centering logic
        // let vehicle_rot = vehicle_xf.compute_transform().rotation;
        // let (v_yaw, v_pitch, _) = vehicle_rot.to_euler(EulerRot::YXZ);
        // state.yaw = lerp_angle(state.yaw, v_yaw, alpha);
    }
}
