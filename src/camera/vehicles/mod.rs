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
    mut query: Query<(&mut CameraController, &mut CameraState, &mut VehicleCameraController)>,
    vehicle_query: Query<&GlobalTransform>,
) {
    let dt = time.delta_secs();

    for (mut controller, mut state, mut vehicle_cam) in query.iter_mut() {
        let Some(vehicle_ent) = vehicle_cam.vehicle_target else { continue };
        let Ok(vehicle_gt) = vehicle_query.get(vehicle_ent) else { continue };

        // 1. Rotation Damping (Chase camera)
        // Align camera yaw with vehicle yaw if not manually rotated
        let vehicle_rot = vehicle_gt.compute_transform().rotation;
        let (v_yaw, _, _) = vehicle_rot.to_euler(EulerRot::YXZ);
        let v_yaw_deg = v_yaw.to_degrees();

        let wrap_diff = (v_yaw_deg - state.yaw + 180.0) % 360.0 - 180.0;
        let rot_alpha = 1.0 - (-vehicle_cam.rotation_damping * dt).exp();
        state.yaw += wrap_diff * rot_alpha;

        // 2. Boost Distance Offset
        // Interpolate boost offset
        let boost_target = if vehicle_cam.current_boost_offset > 0.01 { vehicle_cam.boost_distance_offset } else { 0.0 };
        vehicle_cam.current_boost_offset = vehicle_cam.current_boost_offset + (boost_target - vehicle_cam.current_boost_offset) * vehicle_cam.boost_fade_speed * dt;
        
        controller.distance = controller.base_distance + vehicle_cam.current_boost_offset;

        // 3. First Person Offset
        if vehicle_cam.is_first_person {
            controller.mode = CameraMode::FirstPerson;
            // Apply fp_offset to pivot in state_offsets or here
        }
    }
}
