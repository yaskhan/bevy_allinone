use bevy::prelude::*;
use crate::input::InputState;
use crate::character::CharacterController;
use super::types::*;

use avian3d::prelude::*;

/// System to handle AI detection of targets with Perception checks
pub fn update_ai_perception(
    mut ai_query: Query<(Entity, &GlobalTransform, &mut AiController, &AiPerception, Option<&CharacterFaction>)>,
    target_query: Query<(Entity, &GlobalTransform, Option<&CharacterFaction>), With<CharacterController>>,
    faction_system: Res<FactionSystem>,
    spatial_query: SpatialQuery,
) {
    for (entity, transform, mut ai, perception, ai_faction) in ai_query.iter_mut() {
        if ai.state == AiBehaviorState::Flee { continue; }

        let mut closest_target = None;
        let mut min_dist = perception.vision_range;
        let current_pos = transform.translation();
        let forward = transform.forward();

        let ai_faction_name = ai_faction.map(|f| f.name.as_str()).unwrap_or("Default");

        for (target_entity, target_transform, target_faction) in target_query.iter() {
            if target_entity == entity { continue; }

            // Faction Check
            let target_faction_name = target_faction.map(|f| f.name.as_str()).unwrap_or("Default");
            let relation = faction_system.get_relation(ai_faction_name, target_faction_name);
            
            if relation != FactionRelation::Enemy {
                continue;
            }

            let target_pos = target_transform.translation();
            let to_target = target_pos - current_pos;
            let dist = to_target.length();

            if dist > perception.vision_range {
                continue;
            }

            // check FOV
            let dir_to_target = to_target.normalize();
            let angle = forward.angle_between(dir_to_target).to_degrees();
            
            if angle > perception.fov / 2.0 {
                continue;
            }

            // Raycast check for line of sight
            let origin = current_pos + Vec3::Y * 1.5; 
            let target_eye = target_pos + Vec3::Y * 1.5;
            let direction_vec = (target_eye - origin).normalize();
            let distance = (target_eye - origin).length();
            
            let Ok(direction) = Dir3::new(direction_vec) else { continue };

            let filter = SpatialQueryFilter::from_excluded_entities([entity]);
            
            if let Some(hit) = spatial_query.cast_ray(origin, direction, distance, true, &filter) {
                if hit.entity != target_entity {
                     continue;
                }
            }

            // Visible
            if dist < min_dist {
                min_dist = dist;
                closest_target = Some(target_entity);
            }
        }

        if let Some(target) = closest_target {
            ai.target = Some(target);
            if min_dist <= ai.attack_range {
                ai.state = AiBehaviorState::Attack;
            } else {
                ai.state = AiBehaviorState::Chase;
            }
        } else {
             ai.target = None;
             if ai.state == AiBehaviorState::Chase || ai.state == AiBehaviorState::Attack {
                 ai.state = AiBehaviorState::Idle; 
             }
        }
    }
}

/// Old simple detection system (deprecated or fallback)
pub fn ai_detection_system(
    mut query: Query<(&GlobalTransform, &mut AiController)>,
    target_query: Query<(Entity, &GlobalTransform), (With<CharacterController>, Without<AiController>)>,
) {
    for (transform, mut ai) in query.iter_mut() {
        if ai.state == AiBehaviorState::Flee { continue; }

        let mut closest_target = None;
        let mut min_dist = ai.detection_range;
        let current_pos = transform.translation();

        // Simple distance-based detection
        for (target_entity, target_transform) in target_query.iter() {
            let dist = current_pos.distance(target_transform.translation());
            if dist < min_dist {
                min_dist = dist;
                closest_target = Some(target_entity);
            }
        }

        if let Some(target) = closest_target {
            ai.target = Some(target);
            if min_dist <= ai.attack_range {
                ai.state = AiBehaviorState::Attack;
            } else {
                ai.state = AiBehaviorState::Chase;
            }
        } else {
            ai.target = None;
            if ai.patrol_path.is_empty() {
                ai.state = AiBehaviorState::Idle;
            } else {
                ai.state = AiBehaviorState::Patrol;
            }
        }
    }
}


/// System to handle commands from FriendManager
pub fn handle_friend_commands(
    mut friend_query: Query<(&GlobalTransform, &mut AiController, &FriendManager)>,
    player_query: Query<&GlobalTransform, With<crate::character::Player>>, 
) {
    let Ok(player_xf) = player_query.iter().next().ok_or(()) else { return };
    let _player_pos = player_xf.translation();

    for (_transform, mut ai, friend_mgr) in friend_query.iter_mut() {
        match friend_mgr.current_command {
            AiCommand::Follow => {
                ai.state = AiBehaviorState::Follow;
            }
            AiCommand::Wait => {
                ai.state = AiBehaviorState::Idle;
                ai.target = None;
            }
            AiCommand::Attack => {
                // Allow normal AI logic to find and attack enemies
            }
            AiCommand::Hide => {
                ai.state = AiBehaviorState::Hide;
            }
        }
    }
}

/// System to handle AI movement and state transitions
pub fn update_ai_behavior(
    time: Res<Time>,
    mut ai_query: Query<(
        &GlobalTransform,
        &mut AiController,
        &mut CharacterController,
        &mut InputState,
    )>,
    target_query: Query<&GlobalTransform>,
    player_query: Query<&GlobalTransform, With<crate::character::Player>>,
    hide_query: Query<&GlobalTransform, With<HidePosition>>,
) {
    let delta = time.delta_secs();

    for (transform, mut ai, mut _controller, mut input) in ai_query.iter_mut() {
        let current_pos = transform.translation();

        // Reset input state for AI
        input.movement = Vec2::ZERO;
        input.jump_pressed = false;
        input.sprint_pressed = false;
        input.attack_pressed = false;
        input.crouch_pressed = false;

        match ai.state {
            AiBehaviorState::Idle => {
                // Do nothing
            }
            AiBehaviorState::Patrol => {
                if ai.patrol_path.is_empty() {
                    ai.state = AiBehaviorState::Idle;
                    continue;
                }

                if ai.wait_timer > 0.0 {
                    ai.wait_timer -= delta;
                    continue;
                }

                let target_pos = ai.patrol_path[ai.current_waypoint_index];
                let to_target = target_pos - current_pos;
                let horizontal_dist = Vec3::new(to_target.x, 0.0, to_target.z).length();

                if horizontal_dist < 0.5 {
                    ai.current_waypoint_index = (ai.current_waypoint_index + 1) % ai.patrol_path.len();
                    ai.wait_timer = ai.wait_time_between_waypoints;
                } else {
                    let move_dir = to_target.normalize_or_zero();
                    input.movement = Vec2::new(move_dir.x, move_dir.z);
                    input.sprint_pressed = false; // Patrol is slow
                }
            }
            AiBehaviorState::Chase => {
                if let Some(target_entity) = ai.target {
                    if let Ok(target_transform) = target_query.get(target_entity) {
                        let target_pos = target_transform.translation();
                        let current_pos = transform.translation();
                        let to_target = target_pos - current_pos;
                        let dist = to_target.length();

                        if dist <= ai.attack_range {
                            ai.state = AiBehaviorState::Attack;
                        } else {
                            let move_dir = to_target.normalize_or_zero();
                            input.movement = Vec2::new(move_dir.x, move_dir.z);
                            input.sprint_pressed = dist > 5.0; // Sprint if far
                        }
                    } else {
                        ai.target = None;
                        ai.state = AiBehaviorState::Patrol;
                    }
                }
            }
            AiBehaviorState::Attack => {
                if let Some(target_entity) = ai.target {
                    if let Ok(target_transform) = target_query.get(target_entity) {
                        let target_pos = target_transform.translation();
                        let current_pos = transform.translation();
                        let to_target = target_pos - current_pos;
                        let dist = to_target.length();

                        if dist > ai.attack_range * 1.2 { // Add some buffer
                            ai.state = AiBehaviorState::Chase;
                        } else {
                            // Stop and attack
                            input.movement = Vec2::ZERO;
                            input.attack_pressed = true;
                            
                            let _move_dir = to_target.normalize_or_zero();
                        }
                    } else {
                        ai.target = None;
                        ai.state = AiBehaviorState::Patrol;
                    }
                }
            }
            AiBehaviorState::Flee => {
                // Basic flee logic: run away from closest target/player
                let mut flee_dir = Vec3::ZERO;
                if let Some(target_entity) = ai.target {
                    if let Ok(target_xf) = target_query.get(target_entity) {
                        flee_dir += (current_pos - target_xf.translation()).normalize_or_zero();
                    }
                } else if let Some(player_xf) = player_query.iter().next() {
                    flee_dir += (current_pos - player_xf.translation()).normalize_or_zero();
                }

                if flee_dir != Vec3::ZERO {
                    let move_dir = flee_dir.normalize();
                    input.movement = Vec2::new(move_dir.x, move_dir.z);
                    input.sprint_pressed = true;
                } else {
                    ai.state = AiBehaviorState::Idle;
                }
            }
            AiBehaviorState::Follow => {
                if let Some(player_xf) = player_query.iter().next() {
                    let player_pos = player_xf.translation();
                    let current_pos = transform.translation();
                    let dist = current_pos.distance(player_pos);
                    
                    if dist > 3.0 { // Follow distance
                        let to_player = player_pos - current_pos;
                        let move_dir = to_player.normalize_or_zero();
                        input.movement = Vec2::new(move_dir.x, move_dir.z);
                        input.sprint_pressed = dist > 10.0;
                    }
                }
            }
            AiBehaviorState::Hide => {
                // Find nearest hide position
                let mut best_hide = None;
                let mut min_hide_dist = f32::MAX;
                
                for hide_xf in hide_query.iter() {
                    let dist = current_pos.distance(hide_xf.translation());
                    if dist < min_hide_dist {
                        min_hide_dist = dist;
                        best_hide = Some(hide_xf.translation());
                    }
                }
                
                if let Some(hide_pos) = best_hide {
                    if min_hide_dist > 0.5 {
                        let to_hide = hide_pos - current_pos;
                        let move_dir = to_hide.normalize_or_zero();
                        input.movement = Vec2::new(move_dir.x, move_dir.z);
                        input.sprint_pressed = min_hide_dist > 5.0;
                    } else {
                        // Already at hide position
                        input.movement = Vec2::ZERO;
                        input.crouch_pressed = true;
                    }
                } else {
                    // No hide positions found, fallback to flee or idle
                    ai.state = AiBehaviorState::Flee;
                }
            }
        }
    }
}

/// System to draw AI vision cones using Gizmos
pub fn draw_ai_vision_cones(
    mut gizmos: Gizmos,
    query: Query<(&GlobalTransform, &AiController, &AiPerception, &AiVisionVisualizer)>,
) {
    for (transform, ai, perception, visualizer) in query.iter() {
        if !visualizer.active { continue; }

        let color = if ai.state == AiBehaviorState::Attack || ai.state == AiBehaviorState::Chase {
            visualizer.alert_color
        } else {
            visualizer.normal_color
        };

        let pos = transform.translation() + Vec3::Y * 1.5; // Eye height
        let forward = transform.forward();
        
        let half_fov = perception.fov.to_radians() / 2.0;
        let range = perception.vision_range;

        // Rotate forward vector by +/- half_fov around Y axis
        let left_dir = Quat::from_rotation_y(half_fov) * forward;
        let right_dir = Quat::from_rotation_y(-half_fov) * forward;

        gizmos.ray(pos, left_dir * range, color);
        gizmos.ray(pos, right_dir * range, color);
        
        // Draw connection arc (approximate)
        let segments = 10;
        let angle_step = perception.fov.to_radians() / segments as f32;
        for i in 0..segments {
            let angle = -half_fov + i as f32 * angle_step;
            let next_angle = -half_fov + (i + 1) as f32 * angle_step;
            
            let p1_dir = Quat::from_rotation_y(angle) * forward;
            let p2_dir = Quat::from_rotation_y(next_angle) * forward;
            
            gizmos.line(pos + p1_dir * range, pos + p2_dir * range, color);
        }
    }
}

/// System to visualize AI state (placeholder for icons)
pub fn update_ai_state_visuals(
    mut gizmos: Gizmos,
    query: Query<(&GlobalTransform, &AiController, &AiStateVisuals)>,
) {
    for (transform, ai, visuals) in query.iter() {
        if !visuals.show_state_icons { continue; }

        let pos = transform.translation() + visuals.icon_offset;
        
        match ai.state {
            AiBehaviorState::Idle => {
                // Draw "Zzz" equivalent - Blue Sphere
                gizmos.sphere(pos, 0.2, Color::srgb(0.0, 0.0, 1.0));
            }
            AiBehaviorState::Chase => {
                // "!" - Yellow Sphere
                gizmos.sphere(pos, 0.2, Color::srgb(1.0, 1.0, 0.0));
            }
            AiBehaviorState::Attack => {
                // "!!" - Red Sphere
                gizmos.sphere(pos, 0.3, Color::srgb(1.0, 0.0, 0.0));
            }
            AiBehaviorState::Patrol => {
                // Green small sphere
                gizmos.sphere(pos, 0.1, Color::srgb(0.0, 1.0, 0.0));
            }
            AiBehaviorState::Follow => {
                // Cyan sphere
                 gizmos.sphere(pos, 0.15, Color::srgb(0.0, 1.0, 1.0));
            }
            _ => {}
        }
    }
}
