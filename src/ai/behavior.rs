use bevy::prelude::*;
use crate::character::CharacterController;
use crate::input::InputState;
use super::types::*;

pub fn update_ai_behavior(
    time: Res<Time>,
    mut ai_query: Query<(
        &GlobalTransform,
        &mut AiController,
        &mut CharacterController,
        &mut InputState,
        &mut AiMovement,
    )>,
    target_query: Query<&GlobalTransform>,
    player_query: Query<&GlobalTransform, With<crate::character::Player>>,
    hide_query: Query<&GlobalTransform, With<HidePosition>>,
) {
    let delta = time.delta_secs();

    for (transform, mut ai, _controller, mut input, mut movement) in ai_query.iter_mut() {
        let current_pos = transform.translation();
        
        // Reset input state
        input.movement = Vec2::ZERO;

        match ai.state {
            AiBehaviorState::Idle => {}
            AiBehaviorState::Patrol => {
                if ai.patrol_path.is_empty() {
                    ai.state = AiBehaviorState::Idle;
                    continue;
                }
                if ai.wait_timer > 0.0 {
                    ai.wait_timer -= delta;
                    continue;
                }
                let target_pos = ai.patrol_path[ai.current_waypoint_index % ai.patrol_path.len()];
                let to_target = target_pos - current_pos;
                let horizontal_dist = Vec3::new(to_target.x, 0.0, to_target.z).length();
                if horizontal_dist < 0.5 {
                    ai.current_waypoint_index = (ai.current_waypoint_index + 1) % ai.patrol_path.len();
                    ai.wait_timer = ai.wait_time_between_waypoints;
                } else {
                    let move_dir = to_target.normalize_or_zero();
                    input.movement = Vec2::new(move_dir.x, move_dir.z);
                }
            }
            AiBehaviorState::Chase => {
                if let Some(target_entity) = ai.target {
                    if let Ok(target_transform) = target_query.get(target_entity) {
                        let to_target = target_transform.translation() - current_pos;
                        if to_target.length() <= ai.attack_range {
                            ai.state = AiBehaviorState::Attack;
                        } else {
                            let move_dir = to_target.normalize_or_zero();
                            input.movement = Vec2::new(move_dir.x, move_dir.z);
                            input.sprint_pressed = to_target.length() > 5.0;
                        }
                    } else {
                        ai.target = None;
                        ai.state = AiBehaviorState::Idle;
                    }
                }
            }
            AiBehaviorState::Attack => {
                if let Some(target_entity) = ai.target {
                    if let Ok(target_transform) = target_query.get(target_entity) {
                        let to_target = target_transform.translation() - current_pos;
                        if to_target.length() > ai.attack_range * 1.2 {
                            ai.state = AiBehaviorState::Chase;
                        } else {
                            input.attack_pressed = true;
                        }
                    } else {
                        ai.target = None;
                        ai.state = AiBehaviorState::Idle;
                    }
                }
            }
            AiBehaviorState::Flee => {
                let mut flee_dir = Vec3::ZERO;
                if let Some(target_entity) = ai.target {
                    if let Ok(target_xf) = target_query.get(target_entity) {
                        flee_dir += (current_pos - target_xf.translation()).normalize_or_zero();
                    }
                } else {
                    for player_xf in player_query.iter() {
                        flee_dir += (current_pos - player_xf.translation()).normalize_or_zero();
                    }
                }
                if flee_dir != Vec3::ZERO {
                    let move_dir = flee_dir.normalize();
                    input.movement = Vec2::new(move_dir.x, move_dir.z);
                    input.sprint_pressed = true;
                }
            }
            AiBehaviorState::Follow => {
                for player_xf in player_query.iter() {
                    let to_player = player_xf.translation() - current_pos;
                    if to_player.length() > 3.0 {
                        let move_dir = to_player.normalize_or_zero();
                        input.movement = Vec2::new(move_dir.x, move_dir.z);
                        input.sprint_pressed = to_player.length() > 10.0;
                    }
                }
            }
            AiBehaviorState::Hide => {
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
                    } else {
                        input.crouch_pressed = true;
                    }
                }
            }
            AiBehaviorState::Suspect => {
                ai.suspicion_timer -= delta;
                if ai.suspicion_timer <= 0.0 {
                    ai.state = AiBehaviorState::Idle;
                    movement.destination = None;
                } else if let Some(last_pos) = ai.target_last_position {
                    movement.destination = Some(last_pos);
                    movement.move_type = AiMovementType::Run;
                }
            }
            AiBehaviorState::Wander => {
                if movement.destination.is_none() || movement.destination.unwrap().distance(current_pos) < 1.0 {
                    // Pick "random" point (simplistic for port)
                    let angle = (time.elapsed_secs() * 2.0).sin() * std::f32::consts::PI;
                    let offset = Vec3::new(angle.cos(), 0.0, angle.sin()) * ai.wander_radius;
                    movement.destination = Some(ai.wander_center + offset);
                    movement.move_type = AiMovementType::Walk;
                }
            }
            _ => {}
        }
    }
}

pub fn handle_friend_commands(
    mut friend_query: Query<(&GlobalTransform, &mut AiController, &FriendManager)>,
) {
    for (_xf, mut ai, friend_mgr) in friend_query.iter_mut() {
        match friend_mgr.current_command {
            AiCommand::Follow => ai.state = AiBehaviorState::Follow,
            AiCommand::Wait => {
                ai.state = AiBehaviorState::Idle;
                ai.target = None;
            }
            AiCommand::Hide => ai.state = AiBehaviorState::Hide,
            _ => {}
        }
    }
}

pub fn update_ai_suspicion(
    time: Res<Time>,
    mut query: Query<&mut AiController>,
) {
    let delta = time.delta_secs();
    for mut ai in query.iter_mut() {
        if ai.target.is_none() && (ai.state == AiBehaviorState::Chase || ai.state == AiBehaviorState::Attack) {
            ai.wait_timer -= delta;
            if ai.wait_timer <= 0.0 {
                ai.state = AiBehaviorState::Idle;
            }
        }
    }
}
