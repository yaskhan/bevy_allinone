use bevy::prelude::*;
use crate::camera::{CameraController, CameraMode};
use super::types::*;

/// Process camera events triggered by actions
pub fn process_camera_events(
    mut camera_queue: ResMut<CameraEventQueue>,
    mut camera_query: Query<&mut CameraController>,
    action_query: Query<&ActionSystem>,
) {
    for event in camera_queue.0.drain(..) {
        // Find cameras following this player
        for mut camera in camera_query.iter_mut() {
            if camera.follow_target == Some(event.player_entity) {
                match event.event_type {
                    CameraEventType::SetCameraMode { ref mode } => {
                        camera.mode = match mode.as_str() {
                            "FirstPerson" => CameraMode::FirstPerson,
                            "Locked" => CameraMode::Locked,
                            "SideScroller" => CameraMode::SideScroller,
                            "TopDown" => CameraMode::TopDown,
                            _ => CameraMode::ThirdPerson,
                        };
                        info!("Action set camera mode to: {}", mode);
                    }
                    CameraEventType::SetCameraState { ref state_name } => {
                        camera.current_state_name = state_name.clone();
                        info!("Action set camera state to: {}", state_name);
                    }
                    CameraEventType::Shake { intensity, duration, frequency } => {
                        // This would typically interface with a ShakeQueue resource
                        // For now we log it, or we could add back the ShakeQueue if available
                        info!("Action triggered camera shake: intensity={}, duration={}", intensity, duration);
                    }
                    CameraEventType::Zoom { target_fov, duration } => {
                         info!("Action triggered camera zoom: target_fov={}, duration={}", target_fov, duration);
                         // FOV zoom logic usually requires state.fov_override
                    }
                    CameraEventType::None => {}
                    _ => {}
                }
            }
        }
    }
}
