use bevy::prelude::*;
use super::types::*;
use crate::character::Player;

/// System to find and cache head/neck bones in the hierarchy.
pub fn find_head_bones(
    mut query: Query<(Entity, &mut HeadTrack)>,
    children_query: Query<&Children>,
    names: Query<&Name>,
) {
    for (entity, mut head_track) in query.iter_mut() {
        if head_track.head_bone.is_some() && head_track.neck_bone.is_some() {
            continue;
        }

        // Search hierarchy
        let mut stack: Vec<Entity> = vec![entity];
        while let Some(current) = stack.pop() {
            if let Ok(name) = names.get(current) {
                let name_str = name.as_str().to_lowercase();
                if name_str.contains("head") && head_track.head_bone.is_none() {
                    head_track.head_bone = Some(current);
                    info!("Head bone found: {:?} for entity {:?}", current, entity);
                } else if name_str.contains("neck") && head_track.neck_bone.is_none() {
                    head_track.neck_bone = Some(current);
                    info!("Neck bone found: {:?} for entity {:?}", current, entity);
                }
            }

            if let Ok(children) = children_query.get(current) {
                for i in 0..children.len() {
                    stack.push(children[i]);
                }
            }
        }
    }
}

/// Main system to update head tracking rotations.
pub fn update_head_track(
    time: Res<Time>,
    mut head_track_query: Query<(&mut HeadTrack, &GlobalTransform)>,
    targets_query: Query<(Entity, &GlobalTransform, &HeadTrackTarget)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::camera::CameraController>>,
    mut transforms: Query<&mut Transform>,
) {
    let dt = time.delta_secs();
    
    // Get camera info for default look direction
    let mut camera_forward = Dir3::Z;
    if let Some((_camera, camera_xf)) = camera_query.iter().next() {
        camera_forward = camera_xf.forward();
    }

    for (mut head_track, character_xf) in head_track_query.iter_mut() {
        if !head_track.enabled {
            continue;
        }

        let char_pos = character_xf.translation();
        let char_forward = character_xf.forward();
        let char_up = character_xf.up();

        // 1. Pick Target
        let mut best_target_pos = char_pos + *char_forward * 10.0; // Default: look forward
        let mut target_found = false;

        if head_track.look_in_camera_direction {
            best_target_pos = char_pos + *camera_forward * 10.0;
            target_found = true;
        }

        // Search for specific targets
        let mut closest_dist = f32::MAX;
        for (target_entity, target_xf, target_cfg) in targets_query.iter() {
            if !target_cfg.enabled { continue; }
            
            let pos = target_xf.translation();
            let dist = char_pos.distance(pos);
            
            if dist < target_cfg.min_distance && dist < closest_dist {
                closest_dist = dist;
                best_target_pos = pos;
                head_track.active_target = Some(target_entity);
                target_found = true;
            }
        }

        if !target_found {
            head_track.active_target = None;
        }

        // 2. Weights Update
        let target_head_weight = if target_found { head_track.head_weight } else { 0.0 };
        let target_body_weight = if target_found { head_track.body_weight } else { 0.0 };
        
        head_track.current_head_weight = f32::lerp(
            head_track.current_head_weight, 
            target_head_weight, 
            head_track.weight_change_speed * dt
        );
        head_track.current_body_weight = f32::lerp(
            head_track.current_body_weight, 
            target_body_weight, 
            head_track.weight_change_speed * dt
        );

        // 3. Apply Rotations to Bones
        if head_track.current_head_weight > 0.01 {
            let target_dir = (best_target_pos - char_pos).normalize_or_zero();
            
            // Calculate relative rotation to character forward
            // This is a simplification; would handle local range constraints properly
            let look_at_quat = Quat::from_rotation_arc(*char_forward, target_dir);
            
            // Apply weight
            let final_rot = Quat::IDENTITY.slerp(look_at_quat, head_track.current_head_weight);

            if let Some(head_entity) = head_track.head_bone {
                if let Ok(mut transform) = transforms.get_mut(head_entity) {
                    transform.rotation = transform.rotation.slerp(final_rot, head_track.rotation_speed * dt);
                }
            }
            
            // Neck gets half weight usually
            if let Some(neck_entity) = head_track.neck_bone {
                if let Ok(mut transform) = transforms.get_mut(neck_entity) {
                    let neck_rot = Quat::IDENTITY.slerp(look_at_quat, head_track.current_body_weight);
                    transform.rotation = transform.rotation.slerp(neck_rot, head_track.rotation_speed * dt);
                }
            }
        }
    }
}
