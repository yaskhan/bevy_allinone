use bevy::prelude::*;
use crate::character::CharacterController;
use avian3d::prelude::*;
use crate::input::InputState;
use super::types::*;

pub fn update_ai_movement(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &GlobalTransform,
        &mut AiController,
        &mut AiMovement,
        &mut InputState,
        &CharacterController,
        Option<&mut AiPath>,
    )>,
    mut commands: Commands,
) {
    let _delta = time.delta_secs();

    for (entity, transform, mut ai, mut movement, mut input, _controller, path_opt) in query.iter_mut() {
        if ai.state == AiBehaviorState::Dead {
            input.movement = Vec2::ZERO;
            continue;
        }

        let has_path = path_opt.is_some();
        let mut path_finished = false;
        if let Some(mut path) = path_opt {
            if let Some(next) = path.points.get(path.current_index).copied() {
                movement.destination = Some(next);
            } else {
                path_finished = true;
            }

            if let Some(destination) = movement.destination {
                let current_pos = transform.translation();
                let to_dest = destination - current_pos;
                let horizontal_dist = Vec3::new(to_dest.x, 0.0, to_dest.z).length();

                if horizontal_dist > movement.stop_distance {
                    let move_dir = to_dest.normalize_or_zero();
                    input.movement = Vec2::new(move_dir.x, move_dir.z);

                    // Set speed modifiers based on move type
                    match movement.move_type {
                        AiMovementType::Walk => {
                            input.sprint_pressed = false;
                            input.crouch_pressed = false;
                        }
                        AiMovementType::Run => {
                            input.sprint_pressed = false;
                            input.crouch_pressed = false;
                        }
                        AiMovementType::Sprint => {
                            input.sprint_pressed = true;
                            input.crouch_pressed = false;
                        }
                        AiMovementType::Crouch => {
                            input.sprint_pressed = false;
                            input.crouch_pressed = true;
                        }
                    }
                } else if path.current_index + 1 < path.points.len() {
                    path.current_index += 1;
                    movement.destination = path.points.get(path.current_index).copied();
                } else {
                    path_finished = true;
                }
            }
        }

        if path_finished {
            commands.entity(entity).remove::<AiPath>();
            movement.destination = None;
            input.movement = Vec2::ZERO;
        }

        if has_path {
            continue;
        }

        if let Some(destination) = movement.destination {
            let current_pos = transform.translation();
            let to_dest = destination - current_pos;
            let horizontal_dist = Vec3::new(to_dest.x, 0.0, to_dest.z).length();

            if horizontal_dist > movement.stop_distance {
                let move_dir = to_dest.normalize_or_zero();
                input.movement = Vec2::new(move_dir.x, move_dir.z);

                match movement.move_type {
                    AiMovementType::Walk => {
                        input.sprint_pressed = false;
                        input.crouch_pressed = false;
                    }
                    AiMovementType::Run => {
                        input.sprint_pressed = false;
                        input.crouch_pressed = false;
                    }
                    AiMovementType::Sprint => {
                        input.sprint_pressed = true;
                        input.crouch_pressed = false;
                    }
                    AiMovementType::Crouch => {
                        input.sprint_pressed = false;
                        input.crouch_pressed = true;
                    }
                }
            } else {
                input.movement = Vec2::ZERO;
            }
        }
    }
}

pub fn update_ai_avoidance(
    spatial_query: SpatialQuery,
    mut query: Query<(&GlobalTransform, &AiAvoidanceSettings, &mut InputState)>,
) {
    for (transform, settings, mut input) in query.iter_mut() {
        if !settings.enabled {
            continue;
        }

        let move_dir = Vec3::new(input.movement.x, 0.0, input.movement.y);
        if move_dir.length_squared() < 0.001 {
            continue;
        }

        let origin = transform.translation() + Vec3::Y * 0.5;
        let dir = move_dir.normalize();
        let filter = SpatialQueryFilter::default();

        let hit = spatial_query.cast_ray(origin, Dir3::new(dir).unwrap_or(Dir3::Z), settings.ray_distance, true, &filter);
        if hit.is_none() {
            continue;
        }

        let right = transform.right();
        let left = -right;

        let right_hit = spatial_query.cast_ray(
            origin,
            Dir3::new(right).unwrap_or(Dir3::X),
            settings.side_ray_distance,
            true,
            &filter,
        );
        let left_hit = spatial_query.cast_ray(
            origin,
            Dir3::new(left).unwrap_or(Dir3::NEG_X),
            settings.side_ray_distance,
            true,
            &filter,
        );

        let steer_dir = if right_hit.is_none() && left_hit.is_some() {
            right
        } else if left_hit.is_none() && right_hit.is_some() {
            left
        } else if right_hit.is_none() && left_hit.is_none() {
            right
        } else {
            Vec3::ZERO
        };

        if steer_dir != Vec3::ZERO {
            let adjusted = (dir + steer_dir * settings.steer_strength).normalize_or_zero();
            input.movement = Vec2::new(adjusted.x, adjusted.z);
        }
    }
}
