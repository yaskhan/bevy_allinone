use bevy::prelude::*;
use crate::combat::Health;
use crate::input::InputState;
use super::types::*;

pub fn update_target_marking(
    mut camera_query: Query<(&CameraController, &CameraState, &mut CameraTargetState, &Transform)>,
    target_query: Query<(Entity, &GlobalTransform, &Health)>,
) {
    for (controller, _state, mut target_state, transform) in camera_query.iter_mut() {
        if !controller.target_lock.enabled {
            target_state.marked_target = None;
            continue;
        }

        // If currently locking, we don't scan for new marks unless we want "adjacent" marking
        // For now, keep it simple.
        
        let camera_pos = transform.translation;
        let camera_forward = transform.forward();
        
        // Find best target near screen center (forward vector)
        let mut best_target = None;
        let mut min_score = f32::MAX;

        for (entity, target_gt, health) in target_query.iter() {
            if health.current <= 0.0 { continue; }
            
            let target_pos = target_gt.translation();
            let to_target = target_pos - camera_pos;
            let dist = to_target.length();
            
            if dist > controller.target_lock.max_distance { continue; }
            
            let to_target_dir = to_target / dist;
            let dot = camera_forward.dot(to_target_dir);
            
            // Convert to angle from center
            let angle = dot.acos().to_degrees();
            
            if angle > controller.target_lock.fov_threshold { continue; }
            
            // Score based on distance and angle (prefer center)
            let score = dist * 0.5 + angle * 2.0; 
            
            if score < min_score {
                min_score = score;
                best_target = Some(entity);
            }
        }
        
        target_state.marked_target = best_target;
    }
}

pub fn update_target_lock(
    time: Res<Time>,
    input: Res<InputState>,
    mut camera_query: Query<(&CameraController, &mut CameraState, &mut CameraTargetState)>,
    target_gt_query: Query<&GlobalTransform>,
) {
    let dt = time.delta_secs();

    for (controller, mut state, mut target_state) in camera_query.iter_mut() {
        // Toggle Lock
        if input.lock_on_pressed {
            if target_state.locked_target.is_some() {
                target_state.locked_target = None;
                target_state.is_locking = false;
            } else if let Some(mark) = target_state.marked_target {
                target_state.locked_target = Some(mark);
                target_state.is_locking = true;
            }
        }

        // Maintain Lock
        if let Some(locked_ent) = target_state.locked_target {
            if let Ok(target_gt) = target_gt_query.get(locked_ent) {
                let target_pos = target_gt.translation();
                let pivot_pos = state.current_pivot;
                
                let dir = (target_pos - pivot_pos).normalize();
                
                // Calculate target yaw/pitch
                let target_yaw = dir.x.atan2(dir.z).to_degrees();
                let target_pitch = (-dir.y).asin().to_degrees();
                
                // Smoothly rotate towards target
                state.yaw = state.yaw + (target_yaw - state.yaw) * controller.target_lock.lock_smooth_speed * dt;
                state.pitch = state.pitch + (target_pitch - state.pitch) * controller.target_lock.lock_smooth_speed * dt;
                
                // Check if target is still in range/view
                let dist = (target_pos - pivot_pos).length();
                if dist > controller.target_lock.max_distance + 5.0 {
                    target_state.locked_target = None;
                    target_state.is_locking = false;
                }
            } else {
                target_state.locked_target = None;
                target_state.is_locking = false;
            }
        }
    }
}
