use bevy::prelude::*;
use crate::combat::Health;
use crate::input::InputState;
use super::types::*;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct LookAtPoint {
    pub position: Vec3,
    pub speed: f32,
    pub active: bool,
}

pub fn update_target_marking(
    mut camera_query: Query<(&CameraController, &CameraState, &mut CameraTargetState, &Transform)>,
    target_query: Query<(Entity, &GlobalTransform, &Health, Option<&Name>)>,
) {
    for (controller, _state, mut target_state, transform) in camera_query.iter_mut() {
        if !controller.target_lock.enabled {
            target_state.marked_target = None;
            continue;
        }

        let camera_pos = transform.translation;
        let camera_forward = transform.forward();
        
        // Find best target near screen center
        let mut best_target = None;
        let mut min_score = f32::MAX;

        for (entity, target_gt, health, name) in target_query.iter() {
            if health.current <= 0.0 { continue; }
            
            // Optional: Filter by tag/name
            // For now, let's assume we filter by distance and angle
            
            let target_pos = target_gt.translation();
            let to_target = target_pos - camera_pos;
            let dist = to_target.length();
            
            if dist > controller.target_lock.max_distance { continue; }
            
            let to_target_dir = to_target / dist;
            let dot = camera_forward.dot(to_target_dir);
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
    mut camera_query: Query<(&CameraController, &mut CameraState, &mut CameraTargetState, Option<&LookAtPoint>)>,
    target_gt_query: Query<&GlobalTransform>,
) {
    let dt = time.delta_secs();

    for (controller, mut state, mut target_state, look_at_point) in camera_query.iter_mut() {
        // Look At Point Logic (High Priority)
        if let Some(look_at) = look_at_point {
            if look_at.active {
                let dir = (look_at.position - state.current_pivot).normalize();
                let target_yaw = dir.x.atan2(dir.z).to_degrees();
                let target_pitch = (-dir.y).asin().to_degrees();
                
                let alpha = 1.0 - (-look_at.speed * dt).exp();
                state.yaw = state.yaw + (target_yaw - state.yaw) * alpha;
                state.pitch = state.pitch + (target_pitch - state.pitch) * alpha;
                continue; // Skip target lock if LookAtPoint is active
            }
        }

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
                let target_yaw = dir.x.atan2(dir.z).to_degrees();
                let target_pitch = (-dir.y).asin().to_degrees();
                
                let rot_alpha = 1.0 - (-controller.target_lock.lock_smooth_speed * dt).exp();
                state.yaw = state.yaw + (target_yaw - state.yaw) * rot_alpha;
                state.pitch = state.pitch + (target_pitch - state.pitch) * rot_alpha;
                
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
