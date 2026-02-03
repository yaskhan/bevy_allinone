use bevy::prelude::*;
use crate::input::InputState;
use super::types::*;
use crate::physics::GroundDetection;
use crate::character::types::CharacterMovementState;
use crate::player::ragdoll::{Ragdoll, RagdollState};
use crate::camera::{CameraController, CameraTargetState, CameraBobState};
use crate::weapons::WeaponManager;
use crate::ai::types::AiController;
use crate::grab::types::{Grabber, GrabEventQueue, GrabEvent};
use crate::player::player_modes::PlayerModesSystem;
use avian3d::prelude::Friction;
use bevy::ecs::system::SystemParam;
use crate::character::types::{FootIk, HandIk};
use crate::head_track::types::HeadTrack;
use crate::weapons::WeaponIkState;

#[derive(SystemParam)]
pub struct ActionSystemParams<'w, 's> {
    pub start_queue: ResMut<'w, StartActionEventQueue>,
    pub end_queue: ResMut<'w, EndActionEventQueue>,
    pub player_query: Query<'w, 's, (Entity, &'static mut PlayerActionSystem, &'static mut Transform)>,
    pub action_query: Query<'w, 's, (Entity, &'static mut ActionSystem, &'static GlobalTransform)>,
    pub movement_query: Query<'w, 's, &'static mut CharacterMovementState>,
    pub modes_query: Query<'w, 's, &'static mut PlayerModesSystem>,
    pub weapon_manager_query: Query<'w, 's, &'static mut WeaponManager>,
    pub camera_query: Query<'w, 's, &'static mut CameraController>,
    pub ground_query: Query<'w, 's, &'static GroundDetection>,
    pub ragdoll_query: Query<'w, 's, &'static Ragdoll>,
    pub camera_target_query: Query<'w, 's, &'static CameraTargetState>,
    pub ai_query: Query<'w, 's, (Entity, &'static GlobalTransform, &'static mut AiController)>,
    pub bob_query: Query<'w, 's, &'static mut CameraBobState>,
    pub friction_query: Query<'w, 's, &'static mut Friction>,
    pub grabber_query: Query<'w, 's, &'static mut Grabber>,
    pub foot_ik_query: Query<'w, 's, &'static mut FootIk>,
    pub hand_ik_query: Query<'w, 's, &'static mut HandIk>,
    pub head_track_query: Query<'w, 's, &'static mut HeadTrack>,
    pub weapon_ik_query: Query<'w, 's, &'static mut WeaponIkState>,
    pub grab_events: ResMut<'w, GrabEventQueue>,
    pub time: Res<'w, Time>,
    pub input: Res<'w, InputState>,
}

/// Helper to validate if player meets action requirement conditions
pub fn validate_action_requirements(
    check_on_ground: bool,
    check_crouch_state: bool,
    required_crouch_state: bool,
    check_ragdoll_state: bool,
    required_ragdoll_state: bool,
    check_locked_camera_state: bool,
    required_locked_camera_state: bool,
    check_aiming_state: bool,
    required_aiming_state: bool,
    ground_detection: Option<&GroundDetection>,
    char_movement: Option<&CharacterMovementState>,
    ragdoll: Option<&Ragdoll>,
    camera_target_state: Option<&CameraTargetState>,
    weapon_manager: Option<&WeaponManager>,
) -> bool {
    // Ground check
    if check_on_ground {
        if let Some(gd) = ground_detection {
            if !gd.is_grounded {
                return false;
            }
        }
    }
    
    // Crouch check
    if check_crouch_state {
        if let Some(cms) = char_movement {
            if cms.is_crouching != required_crouch_state {
                return false;
            }
        }
    }
    
    // Ragdoll check
    if check_ragdoll_state {
        if let Some(r) = ragdoll {
            let is_ragdolled = r.current_state == RagdollState::Ragdolled;
            if is_ragdolled != required_ragdoll_state {
                return false;
            }
        }
    }
    
    // Camera lock check
    if check_locked_camera_state {
        if let Some(cts) = camera_target_state {
            if cts.is_locking != required_locked_camera_state {
                return false;
            }
        }
    }
    
    // Aiming check
    if check_aiming_state {
        if let Some(wm) = weapon_manager {
            let is_aiming = wm.aiming_in_third_person || wm.aiming_in_first_person;
            if is_aiming != required_aiming_state {
                return false;
            }
        }
    }
    
    true
}

pub fn update_action_system(
    mut params: ActionSystemParams,
    mut commands: Commands,
) {
    // 1. Handle Start Action
    for event in params.start_queue.0.drain(..) {
        if let Ok((_player_entity, mut player_action, _)) = params.player_query.get_mut(event.player_entity) {
             // Only start if not already active
            if player_action.is_action_active {
                  continue;
            }

            if let Ok((action_entity, action, _action_transform)) = params.action_query.get_mut(event.action_entity) {
                // Set Player State
                player_action.current_action = Some(action_entity);
                player_action.is_action_active = true;
                player_action.action_timer = 0.0;
                player_action.walk_timer = 0.0;
                player_action.walk_state = WalkToTargetState::Idle;
                
                // Initialize event system
                player_action.events_active = false;
                player_action.event_start_time = 0.0;
                
                // Determine initial state
                if action.use_walk_to_target_before_action {
                    player_action.state = ActionState::WalkingToTargetBefore;
                } else if action.use_position_to_adjust_player && action.match_target_transform.is_some() {
                    player_action.state = ActionState::AdjustingTransform;
                } else {
                    player_action.state = ActionState::PlayingAnimation;
                }
                
                // Apply player state control
                let control = &action.player_state_control;
                
                // Save current states if preservation is enabled
                if control.preserve_crouch {
                    // Query actual crouch state from CharacterControllerState
                    if let Ok(char_state) = params.movement_query.get(event.player_entity) {
                        player_action.saved_crouch_state = char_state.is_crouching;
                        info!("Saved crouch state: {}", char_state.is_crouching);
                    }
                }
                if control.preserve_strafe {
                    // Query actual strafe mode from PlayerModesSystem
                    if let Ok(player_modes) = params.modes_query.get(event.player_entity) {
                        // Check if current mode is strafe-related (e.g., contains "Strafe" in name)
                        player_action.saved_strafe_state = player_modes.current_players_mode_name.contains("Strafe");
                        info!("Saved strafe state: {}", player_action.saved_strafe_state);
                    }
                }
                
                // Save weapon states
                if control.preserve_aim_state {
                    if let Ok(weapon_manager) = params.weapon_manager_query.get(event.player_entity) {
                        player_action.saved_aim_state = weapon_manager.aiming_in_third_person;
                        info!("Saved aim state: {}", player_action.saved_aim_state);
                    }
                }
                
                // Apply immediate weapon control
                if control.disable_weapon_input {
                    if let Ok(mut weapon_manager) = params.weapon_manager_query.get_mut(event.player_entity) {
                        weapon_manager.player_currently_busy = true;
                        info!("Weapon input disabled during action");
                    }
                }
                
                if control.force_aim {
                    if let Ok(mut weapon_manager) = params.weapon_manager_query.get_mut(event.player_entity) {
                        weapon_manager.aiming_in_third_person = true;
                        weapon_manager.aiming_in_first_person = true;
                        info!("Forced aim state during action");
                    }
                }
                
                if control.disable_weapon_switching {
                    if let Ok(mut weapon_manager) = params.weapon_manager_query.get_mut(event.player_entity) {
                        player_action.saved_change_keys = weapon_manager.change_weapons_with_keys;
                        player_action.saved_change_wheel = weapon_manager.change_weapons_with_mouse_wheel;
                        player_action.saved_change_number = weapon_manager.change_weapons_with_number_keys;
                        
                        weapon_manager.change_weapons_with_keys = false;
                        weapon_manager.change_weapons_with_mouse_wheel = false;
                        weapon_manager.change_weapons_with_number_keys = false;
                        info!("Weapon switching disabled during action");
                    }
                }
                
                // Camera controls
                for mut camera in params.camera_query.iter_mut() {
                    if camera.follow_target == Some(event.player_entity) {
                        // Save current camera state
                        player_action.saved_camera_mode = format!("{:?}", camera.mode);
                        player_action.saved_camera_state_name = camera.current_state_name.clone();
                        player_action.saved_camera_enabled = camera.enabled;
                        
                        // Apply controls
                        if control.force_third_person {
                            camera.mode = crate::camera::CameraMode::ThirdPerson;
                            info!("Forced third person view");
                        }
                        
                        if let Some(ref state_name) = control.camera_state_name {
                            camera.current_state_name = state_name.clone();
                            info!("Set camera state to: {}", state_name);
                        }
                        
                        if control.pause_camera_rotation {
                            camera.enabled = false;
                            info!("Paused camera rotation/follow");
                        }
                        
                        // Note: disable_camera_zoom is handled by blocking inputs, but we could also 
                        // add a flag to CameraController if needed.
                    }
                }
                
                // Save and Apply IK/Tracking control
                if let Ok(mut foot_ik) = params.foot_ik_query.get_mut(event.player_entity) {
                    player_action.saved_foot_ik_enabled = foot_ik.enabled;
                    if control.foot_ik_pause {
                        foot_ik.enabled = false;
                        info!("Foot IK paused during action");
                    }
                }
                if let Ok(mut hand_ik) = params.hand_ik_query.get_mut(event.player_entity) {
                    player_action.saved_hand_ik_enabled = hand_ik.enabled;
                    if control.hand_ik_pause {
                        hand_ik.enabled = false;
                        info!("Hand IK paused during action");
                    }
                }
                if let Ok(mut head_track) = params.head_track_query.get_mut(event.player_entity) {
                    player_action.saved_head_track_enabled = head_track.enabled;
                    if control.head_track_pause {
                        head_track.enabled = false;
                        info!("Head tracking paused during action");
                    }
                }
                // Weapon IK weight control
                if let Ok(weapon_manager) = params.weapon_manager_query.get(event.player_entity) {
                    for &weapon_ent in &weapon_manager.weapons_list {
                        if let Ok(mut weapon_ik) = params.weapon_ik_query.get_mut(weapon_ent) {
                            player_action.saved_weapon_ik_weight = weapon_ik.weight;
                            if let Some(weight) = control.weapon_ik_weight {
                                weapon_ik.weight = weight;
                                info!("Hand/Weapon IK weight set to {} during action", weight);
                            }
                        }
                    }
                }
                
                info!("Started action: {} with state control (movement={}, rotation={}, input={}, gravity={}, invincible={})",
                    action.action_name, !control.disable_movement, !control.disable_rotation, 
                    !control.disable_input, !control.disable_gravity, control.invincible);
            }
        }
    }
    
    // 2. Update Active Actions
    for (player_entity, mut player_action, mut player_transform) in params.player_query.iter_mut() {
        if !player_action.is_action_active { continue; }
        
        let mut action_ended = false;

        if let Some(action_entity) = player_action.current_action {
             if let Ok((_, action, action_glob_transform)) = params.action_query.get(action_entity) {
                 
                 match player_action.state {
                    ActionState::WalkingToTargetBefore => {
                        // Walk-to-target is handled by update_walk_to_target_system
                        // This state just waits for walk completion
                        if player_action.walk_state == WalkToTargetState::ReachedTarget {
                            // Transition to next state
                            if action.use_position_to_adjust_player && action.match_target_transform.is_some() {
                                player_action.state = ActionState::AdjustingTransform;
                            } else {
                                player_action.state = ActionState::PlayingAnimation;
                            }
                            player_action.action_timer = 0.0;
                            info!("Action System: Reached walk target, transitioning to {:?}", player_action.state);
                        } else if player_action.walk_state == WalkToTargetState::TimedOut {
                            // Timeout - cancel action
                            warn!("Action System: Walk to target timed out, cancelling action");
                            action_ended = true;
                        }
                    }
                    ActionState::AdjustingTransform => {
                         if let Some(target_local) = action.match_target_transform {
                            // Calculate target world transform
                            // Assuming match_target_transform is local to the action entity
                            // If it's absolute, we use it directly. Assuming relative for now as per comment.
                            // But for simplicity, let's treat match_target_transform as an offset from action root
                            
                            let action_rotation = action_glob_transform.compute_transform().rotation;
                            let action_translation = action_glob_transform.translation();
                            
                            let target_rotation = action_rotation * target_local.rotation;
                            let target_translation = action_translation + action_rotation * target_local.translation;
                            
                            // Interpolate
                            let speed = action.adjust_player_position_speed * params.time.delta_secs();
                            
                            let new_pos = player_transform.translation.lerp(target_translation, speed);
                            let new_rot = player_transform.rotation.slerp(target_rotation, speed);
                            
                            player_transform.translation = new_pos;
                            player_transform.rotation = new_rot;
                            
                            let dist = player_transform.translation.distance(target_translation);
                            let angle = player_transform.rotation.angle_between(target_rotation);
                            
                            // Check if reached
                            if dist < 0.05 && angle < 0.1 {
                                player_transform.translation = target_translation;
                                player_transform.rotation = target_rotation;
                                player_action.state = ActionState::PlayingAnimation;
                                player_action.action_timer = 0.0;
                                info!("Action System: Finished adjustment, starting animation");
                            }
                         } else {
                             // Fallback if data missing
                             player_action.state = ActionState::PlayingAnimation;
                         }
                    }
                    ActionState::PlayingAnimation => {
                        if player_action.action_timer == 0.0 {
                            // First frame of action start
                            // AI Pause
                            if action.pause_ai_in_radius {
                                let player_pos = player_transform.translation;
                                for (_, ai_xf, mut ai) in params.ai_query.iter_mut() {
                                    if ai_xf.translation().distance(player_pos) <= action.ai_pause_radius {
                                        ai.is_paused = true;
                                    }
                                }
                            }
                            
                            // Headbob
                            if action.player_state_control.pause_headbob {
                               for mut bob in params.bob_query.iter_mut() {
                                   bob.active = false;
                               }
                            }
                            
                            // Friction
                            if let Some(f_override) = action.player_state_control.friction_override {
                                if let Ok(mut friction) = params.friction_query.get_mut(player_entity) {
                                    player_action.saved_friction = Some(friction.static_coefficient);
                                    friction.static_coefficient = f_override;
                                    friction.dynamic_coefficient = f_override;
                                }
                            }
                            
                            // Drop Grabbed Object
                            if action.player_state_control.drop_held_object {
                                if let Ok(mut grabber) = params.grabber_query.get_mut(player_entity) {
                                    if let Some(held) = grabber.held_object {
                                        params.grab_events.0.push(GrabEvent::Drop(player_entity, held));
                                        grabber.held_object = None;
                                        info!("Action System: Force-dropped held object {:?}", held);
                                    }
                                }
                            }
                        }
                        
                        player_action.action_timer += params.time.delta_secs();
                        
                        // Jump Interruption
                        if action.can_interrupted_by_jump && params.input.jump_pressed {
                            player_action.state = ActionState::Interrupted;
                            info!("Action Interrupted by Jump");
                            // Trigger end-action logic via queue
                            params.end_queue.0.push(EndActionEvent {
                                player_entity,
                                action_entity,
                            });
                            return;
                        }

                        if player_action.action_timer >= action.duration {
                            // Check if we need to walk after action
                            if action.use_walk_to_target_after_action {
                                player_action.state = ActionState::WalkingToTargetAfter;
                                player_action.walk_timer = 0.0;
                                player_action.walk_state = WalkToTargetState::Idle;
                            } else if action.wait_for_input_to_continue {
                                player_action.state = ActionState::WaitingForInput;
                                info!("Action System: Waiting for input to continue");
                            } else if action.stay_in_state_after_finish {
                                player_action.state = ActionState::StayingInState;
                                info!("Action System: Staying in state");
                            } else {
                                player_action.state = ActionState::Finished;
                            }
                        }
                    }
                    ActionState::WalkingToTargetAfter => {
                        // Walk-to-target is handled by update_walk_to_target_system
                        if player_action.walk_state == WalkToTargetState::ReachedTarget {
                            if action.wait_for_input_to_continue {
                                player_action.state = ActionState::WaitingForInput;
                                info!("Action System: Reached walk target, waiting for input");
                            } else if action.stay_in_state_after_finish {
                                player_action.state = ActionState::StayingInState;
                                info!("Action System: Reached walk target, staying in state");
                            } else {
                                player_action.state = ActionState::Finished;
                            }
                        } else if player_action.walk_state == WalkToTargetState::TimedOut {
                            warn!("Action System: Walk after action timed out");
                            player_action.state = ActionState::Finished;
                        }
                    }
                    ActionState::WaitingForInput => {
                        // Check if required input is pressed
                        let input_pressed = match action.input_to_continue.as_str() {
                            "Interact" => params.input.interact_pressed,
                            "Jump" => params.input.jump_pressed,
                            "Fire" => params.input.fire_pressed,
                            "Aim" => params.input.aim_pressed,
                            _ => params.input.interact_pressed, // Default to interact
                        };
                        
                        if input_pressed {
                            if action.stay_in_state_after_finish {
                                player_action.state = ActionState::StayingInState;
                            } else {
                                player_action.state = ActionState::Finished;
                            }
                            info!("Action System: Input received, continuing");
                        }
                    }
                    ActionState::StayingInState => {
                        // This state persists until interrupted by another action or manual stop
                        // No automatic transition to Finished
                    }
                    ActionState::Finished => {
                        action_ended = true;
                    }
                    ActionState::Idle => {
                        // Should not happen if active
                        action_ended = true;
                    }
                 }

             } else {
                 // Action entity missing?
                 action_ended = true;
             }
        } else {
             action_ended = true;
        }
        
        if action_ended {
            params.end_queue.0.push(EndActionEvent {
                player_entity,
                action_entity: player_action.current_action.unwrap_or(Entity::PLACEHOLDER),
            });
        }
    }
    
    // 3. Handle End Action
    for event in params.end_queue.0.drain(..) {
        if let Ok((_ent, mut player_action, player_transform)) = params.player_query.get_mut(event.player_entity) {
            // Validate it's the correct action ending
            let is_matching_action = player_action.current_action == Some(event.action_entity) || event.action_entity == Entity::PLACEHOLDER;
            
            if is_matching_action && player_action.is_action_active {
                // Get action to check if we need to restore states
                let should_restore_states = if let Some(action_entity) = player_action.current_action {
                    if let Ok((_entity, action, _transform)) = params.action_query.get(action_entity) {
                        let control = &action.player_state_control;
                        
                        // AI Resume
                        if action.pause_ai_in_radius {
                            let player_pos = player_transform.translation;
                            for (_, ai_xf, mut ai) in params.ai_query.iter_mut() {
                                if ai_xf.translation().distance(player_pos) <= action.ai_pause_radius {
                                    ai.is_paused = false;
                                }
                            }
                        }
                        
                        // Headbob Resume
                        if control.pause_headbob {
                            for mut bob in params.bob_query.iter_mut() {
                                bob.active = true;
                            }
                        }
                        
                        // Friction Resume
                        if let Some(saved_f) = player_action.saved_friction {
                            if let Ok(mut friction) = params.friction_query.get_mut(event.player_entity) {
                                friction.static_coefficient = saved_f;
                                friction.dynamic_coefficient = saved_f;
                            }
                            player_action.saved_friction = None;
                        }

                        // Destroy action
                        if action.destroy_action_on_end {
                            commands.entity(action_entity).despawn_recursive();
                            info!("Action entity {:?} destroyed on end", action_entity);
                        }

                        // Restore IK/Tracking state
                        if let Ok(mut foot_ik) = params.foot_ik_query.get_mut(event.player_entity) {
                            foot_ik.enabled = player_action.saved_foot_ik_enabled;
                        }
                        if let Ok(mut hand_ik) = params.hand_ik_query.get_mut(event.player_entity) {
                            hand_ik.enabled = player_action.saved_hand_ik_enabled;
                        }
                        if let Ok(mut head_track) = params.head_track_query.get_mut(event.player_entity) {
                            head_track.enabled = player_action.saved_head_track_enabled;
                        }
                        if let Ok(weapon_manager) = params.weapon_manager_query.get(event.player_entity) {
                            for &weapon_ent in &weapon_manager.weapons_list {
                                if let Ok(mut weapon_ik) = params.weapon_ik_query.get_mut(weapon_ent) {
                                    weapon_ik.weight = player_action.saved_weapon_ik_weight;
                                }
                            }
                        }

                        // Restore preserved states
                        if control.preserve_crouch && player_action.saved_crouch_state {
                            // Restore crouch state to CharacterControllerState
                            if let Ok(mut char_state) = params.movement_query.get_mut(event.player_entity) {
                                char_state.is_crouching = true;
                                info!("Restored crouch state to: true");
                            }
                        }
                        if control.preserve_strafe && player_action.saved_strafe_state {
                            // Restore strafe mode to PlayerModesSystem
                            if let Ok(mut player_modes) = params.modes_query.get_mut(event.player_entity) {
                                // Find strafe mode name first
                                let strafe_mode_name = player_modes.player_modes.iter()
                                    .find(|mode| mode.name.contains("Strafe"))
                                    .map(|mode| mode.name.clone());
                                
                                // Then assign it
                                if let Some(mode_name) = strafe_mode_name {
                                    player_modes.current_players_mode_name = mode_name.clone();
                                    info!("Restored strafe mode to: {}", mode_name);
                                }
                            }
                        }
                        
                        // Restore weapon states
                        if control.preserve_aim_state && player_action.saved_aim_state {
                            if let Ok(mut weapon_manager) = params.weapon_manager_query.get_mut(event.player_entity) {
                                weapon_manager.aiming_in_third_person = true;
                                weapon_manager.aiming_in_first_person = true;
                                info!("Restored aim state to: true");
                            }
                        }
                        
                        // Reset busy flag if it was disabled
                        if control.disable_weapon_input {
                            if let Ok(mut weapon_manager) = params.weapon_manager_query.get_mut(event.player_entity) {
                                weapon_manager.player_currently_busy = false;
                                info!("Weapon input re-enabled");
                            }
                        }
                        
                        // Restore switching flags
                        if control.disable_weapon_switching {
                            if let Ok(mut weapon_manager) = params.weapon_manager_query.get_mut(event.player_entity) {
                                weapon_manager.change_weapons_with_keys = player_action.saved_change_keys;
                                weapon_manager.change_weapons_with_mouse_wheel = player_action.saved_change_wheel;
                                weapon_manager.change_weapons_with_number_keys = player_action.saved_change_number;
                                info!("Weapon switching restored");
                            }
                        }
                        
                        // Restore camera state
                        for mut camera in params.camera_query.iter_mut() {
                            if camera.follow_target == Some(event.player_entity) {
                                // Restoration logic: we need to parse the mode string back or just save the enum if possible
                                // For now, let's assume we can restore mode if it was forced
                                if control.force_third_person {
                                    // This is a bit tricky without the enum, but we can match on the saved string
                                    camera.mode = match player_action.saved_camera_mode.as_str() {
                                        "FirstPerson" => crate::camera::CameraMode::FirstPerson,
                                        "Locked" => crate::camera::CameraMode::Locked,
                                        "SideScroller" => crate::camera::CameraMode::SideScroller,
                                        "TopDown" => crate::camera::CameraMode::TopDown,
                                        _ => crate::camera::CameraMode::ThirdPerson,
                                    };
                                }
                                
                                if control.camera_state_name.is_some() {
                                    camera.current_state_name = player_action.saved_camera_state_name.clone();
                                }
                                
                                if control.pause_camera_rotation {
                                    camera.enabled = player_action.saved_camera_enabled;
                                }
                                info!("Camera state restored");
                            }
                        }
                        
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
                
                player_action.is_action_active = false;
                player_action.current_action = None;
                player_action.state = ActionState::Idle;
                
                // Reset saved states
                player_action.saved_crouch_state = false;
                player_action.saved_strafe_state = false;
                
                info!("Ended action (restored states: {})", should_restore_states);
            }
        }
    }
    
    // 4. Input Detection (Simple Proximity Check for now)
    // In a real implementation this would be more optimized or use the existing spatial query system
    if params.input.interact_pressed {
         for (player_entity, player_action, player_transform) in params.player_query.iter() {
             if player_action.is_action_active { continue; }
             
             for (action_entity, action, action_transform) in params.action_query.iter() {
                 if !action.is_active { continue; }
                 
                 let dist = player_transform.translation.distance(action_transform.translation());
                 if dist <= action.min_distance {
                     // Advanced Validation
                     let ground = params.ground_query.get(player_entity).ok();
                     let cms = params.movement_query.get(player_entity).ok();
                     let ragdoll = params.ragdoll_query.get(player_entity).ok();
                     let weapon_manager = params.weapon_manager_query.get(player_entity).ok();
                     
                     // Find active camera for this player
                     let mut camera_target = None;
                     for target_state in params.camera_target_query.iter() {
                         // This is a bit simplified, usually you'd link camera to player
                         camera_target = Some(target_state);
                         break;
                     }

                     if !validate_action_requirements(
                         action.check_on_ground,
                         action.check_crouch_state,
                         action.required_crouch_state,
                         action.check_ragdoll_state,
                         action.required_ragdoll_state,
                         action.check_locked_camera_state,
                         action.required_locked_camera_state,
                         action.check_aiming_state,
                         action.required_aiming_state,
                         ground,
                         cms,
                         ragdoll,
                         camera_target,
                         weapon_manager,
                     ) {
                         continue;
                     }

                     // Check Angle if needed
                     // Simple forward check
                     let to_action = (action_transform.translation() - player_transform.translation).normalize_or_zero();
                     let player_forward = player_transform.forward();
                     
                     let dot = player_forward.dot(to_action);
                     
                     // ~45 degrees is dot > 0.707
                     let angle_threshold = if action.use_min_angle { action.min_angle.to_radians().cos() } else { -1.0 };
                     
                     if dot >= angle_threshold {
                         params.start_queue.0.push(StartActionEvent {
                             player_entity,
                             action_entity,
                         });
                         break; // Only start one action
                     }
                 }
             }
         }
    }
}

/// System to update animator parameters based on player action state
pub fn update_animator_parameters_system(
    mut query: Query<(&PlayerActionSystem, &mut AnimatorParameters)>,
    action_query: Query<&ActionSystem>,
) {
    for (player_action, mut animator_params) in query.iter_mut() {
        if player_action.is_action_active {
            if let Some(action_entity) = player_action.current_action {
                if let Ok(action) = action_query.get(action_entity) {
                    // Set action active flags
                    if action.animation_used_on_upper_body {
                        animator_params.action_active_upper_body = true;
                        if action.disable_regular_action_active_state {
                            animator_params.action_active = false;
                        }
                    } else {
                        animator_params.action_active = true;
                    }
                    
                    // Set action ID
                    if action.use_action_id {
                        animator_params.action_id = action.action_id;
                    }
                }
            }
        } else if player_action.is_action_active && 
                 (player_action.state == ActionState::WaitingForInput || player_action.state == ActionState::StayingInState) {
            // Keep animator flags during waiting/staying states
            // We don't have the action here without querying, but we can assume if it's active we keep flags
            // Refinement: if we need action settings (like upper body), we'd need to query it.
            // For now, let's just make sure they don't reset.
        } else {
            // Reset animator parameters when no action is active
            animator_params.action_active = false;
            animator_params.action_active_upper_body = false;
            animator_params.action_id = 0;
        }
    }
}

/// System to automatically reset sequence indices for custom actions
pub fn update_custom_action_index_reset_system(
    mut query: Query<&mut CustomActionInfo>,
    time: Res<Time>,
) {
    for mut action_info in query.iter_mut() {
        if action_info.reset_index_after_delay && action_info.current_action_index > 0 {
            if time.elapsed_secs() - action_info.last_time_used > action_info.index_reset_delay {
                action_info.current_action_index = 0;
                info!("Action index reset for {}", action_info.name);
            }
        }
    }
}

/// System to apply match target during actions
pub fn apply_match_target_system(
    mut query: Query<(&mut PlayerActionSystem, &mut Transform)>,
    mut action_query: Query<&mut ActionSystem>,
    time: Res<Time>,
) {
    for (player_action, mut player_transform) in query.iter_mut() {
        if !player_action.is_action_active {
            continue;
        }
        
        if let Some(action_entity) = player_action.current_action {
            if let Ok(mut action) = action_query.get_mut(action_entity) {
                if !action.use_match_target {
                    continue;
                }
                
                // Clone the match config to avoid borrow issues
                if let Some(match_config) = action.match_target_config.clone() {
                    if !match_config.enabled {
                        continue;
                    }
                    
                    // Update normalized time (assuming duration-based)
                    let normalized_time = (player_action.action_timer / action.duration).clamp(0.0, 1.0);
                    
                    // Update the config's normalized time
                    if let Some(ref mut config) = action.match_target_config {
                        config.current_normalized_time = normalized_time;
                    }
                    
                    // Check if we're in the match target time window
                    if normalized_time >= match_config.start_time && normalized_time <= match_config.end_time {
                        // Calculate interpolation factor within the window
                        let window_duration = match_config.end_time - match_config.start_time;
                        let window_progress = if window_duration > 0.0 {
                            (normalized_time - match_config.start_time) / window_duration
                        } else {
                            1.0
                        };
                        
                        // Apply position matching with weights
                        let target_pos = match_config.target_position;
                        let current_pos = player_transform.translation;
                        let weighted_target = Vec3::new(
                            current_pos.x + (target_pos.x - current_pos.x) * match_config.position_weight.x,
                            current_pos.y + (target_pos.y - current_pos.y) * match_config.position_weight.y,
                            current_pos.z + (target_pos.z - current_pos.z) * match_config.position_weight.z,
                        );
                        player_transform.translation = player_transform.translation.lerp(weighted_target, window_progress);
                        
                        // Apply rotation matching with weight
                        if match_config.rotation_weight > 0.0 {
                            player_transform.rotation = player_transform.rotation.slerp(
                                match_config.target_rotation,
                                window_progress * match_config.rotation_weight
                            );
                        }
                    }
                }
            }
        }
    }
}

/// System to handle custom action activation requests
pub fn handle_custom_action_activation_system(
    mut activate_queue: ResMut<ActivateCustomActionEventQueue>,
    mut start_action_queue: ResMut<StartActionEventQueue>,
    mut interrupted_queue: ResMut<ActionInterruptedEventQueue>,
    custom_action_manager: Res<CustomActionManager>,
    mut player_query: Query<&mut PlayerActionSystem>,
    mut custom_action_query: Query<&mut CustomActionInfo>,
    action_query: Query<&ActionSystem>,
    ground_query: Query<&GroundDetection>,
    char_movement_query: Query<&CharacterMovementState>,
    ragdoll_query: Query<&Ragdoll>,
    camera_target_query: Query<&CameraTargetState>,
    weapon_manager_query: Query<&WeaponManager>,
    time: Res<Time>,
) {
    for event in activate_queue.0.drain(..) {
        let action_name_lower = event.action_name.to_lowercase();
        
        // Look up action by name
        if let Some(&custom_action_entity) = custom_action_manager.action_lookup.get(&action_name_lower) {
            if let Ok(mut custom_action_info) = custom_action_query.get_mut(custom_action_entity) {
                if !custom_action_info.enabled {
                    continue;
                }
                
                // Advanced Validation
                let ground = ground_query.get(event.player_entity).ok();
                let cms = char_movement_query.get(event.player_entity).ok();
                let ragdoll = ragdoll_query.get(event.player_entity).ok();
                let weapon_manager = weapon_manager_query.get(event.player_entity).ok();
                
                let mut camera_target = None;
                for target_state in camera_target_query.iter() {
                    camera_target = Some(target_state);
                    break;
                }

                if !validate_action_requirements(
                    custom_action_info.check_on_ground,
                    custom_action_info.check_crouch_state,
                    custom_action_info.required_crouch_state,
                    custom_action_info.check_ragdoll_state,
                    custom_action_info.required_ragdoll_state,
                    custom_action_info.check_locked_camera_state,
                    custom_action_info.required_locked_camera_state,
                    custom_action_info.check_aiming_state,
                    custom_action_info.required_aiming_state,
                    ground,
                    cms,
                    ragdoll,
                    camera_target,
                    weapon_manager,
                ) {
                    info!("Action {} failed high-level validation", custom_action_info.name);
                    continue;
                }
                
                // Handle probability
                if custom_action_info.use_probability {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let roll: f32 = rng.gen();
                    if roll > custom_action_info.probability {
                        info!("Action {} failed probability check ({} > {})", custom_action_info.name, roll, custom_action_info.probability);
                        continue;
                    }
                }
                
                // Determine which action entity to use
                let action_entity_to_use = if custom_action_info.use_random_action_list {
                    if custom_action_info.random_action_entities.is_empty() {
                        continue;
                    }
                    
                    if custom_action_info.follow_actions_order {
                        // Use current index (would need to be mutable in real implementation)
                        let index = custom_action_info.current_action_index % custom_action_info.random_action_entities.len();
                        custom_action_info.random_action_entities[index]
                    } else {
                        // Random selection
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        let index = rng.gen_range(0..custom_action_info.random_action_entities.len());
                        custom_action_info.random_action_entities[index]
                    }
                } else {
                    custom_action_info.action_system_entity.unwrap_or(Entity::PLACEHOLDER)
                };
                
                if action_entity_to_use == Entity::PLACEHOLDER {
                    continue;
                }
                
                // Check if we need to interrupt current action
                if let Ok(mut player_action) = player_query.get_mut(event.player_entity) {
                    let mut can_start = true;
                    
                    if player_action.is_action_active {
                        if let Some(current_action_entity) = player_action.current_action {
                            if let Ok(current_action) = action_query.get(current_action_entity) {
                                // Check if current action can be stopped
                                if !current_action.can_stop_previous_action {
                                    // Check if new action can force interrupt
                                    if custom_action_info.can_interrupt_other_actions {
                                        let can_interrupt = if custom_action_info.use_category_to_check_interrupt {
                                            custom_action_info.action_categories_to_interrupt.contains(&current_action.category_name)
                                        } else {
                                            custom_action_info.action_names_to_interrupt.contains(&current_action.action_name)
                                        };
                                        
                                        if can_interrupt || custom_action_info.can_force_interrupt {
                                            // Interrupt the current action
                                            interrupted_queue.0.push(ActionInterruptedEvent {
                                                player_entity: event.player_entity,
                                                interrupted_action_entity: current_action_entity,
                                                new_action_entity: action_entity_to_use,
                                            });
                                            
                                            info!("Action {} interrupted by {}", current_action.action_name, custom_action_info.name);
                                        } else {
                                            // Queue for later
                                            player_action.action_waiting_to_resume = Some(action_entity_to_use);
                                            can_start = false;
                                        }
                                    } else {
                                        // Queue for later
                                        player_action.action_waiting_to_resume = Some(action_entity_to_use);
                                        can_start = false;
                                    }
                                }
                            }
                        }
                    }
                    
                    if can_start {
                    // Update sequencing info
                    custom_action_info.last_time_used = time.elapsed_secs();
                    
                    // Increment index if using random/ordered list
                    if custom_action_info.use_random_action_list && custom_action_info.follow_actions_order {
                        custom_action_info.current_action_index += 1;
                    }

                    // Set category
                        player_action.current_action_category = Some(custom_action_info.category.name.clone());
                        
                        // Start the action
                        start_action_queue.0.push(StartActionEvent {
                            player_entity: event.player_entity,
                            action_entity: action_entity_to_use,
                        });
                        
                        info!("Starting custom action: {}", custom_action_info.name);
                    }
                }
            }
        } else {
            warn!("Custom action '{}' not found in manager", event.action_name);
        }
    }
}

/// System to update custom action manager lookup tables
pub fn update_custom_action_manager_system(
    mut manager: ResMut<CustomActionManager>,
    query: Query<(Entity, &CustomActionInfo), Changed<CustomActionInfo>>,
) {
    for (entity, custom_action_info) in query.iter() {
        let name_lower = custom_action_info.name.to_lowercase();
        
        // Update name lookup
        manager.action_lookup.insert(name_lower, entity);
        
        // Update category lookup
        let category_name = custom_action_info.category.name.clone();
        manager.category_lookup
            .entry(category_name)
            .or_insert_with(Vec::new)
            .push(entity);
    }
}

/// System to handle walk-to-target for actions
pub fn update_walk_to_target_system(
    mut player_query: Query<(&mut PlayerActionSystem, Entity)>,
    action_query: Query<&ActionSystem>,
    mut navmesh_query: Query<&mut crate::player::navmesh_override::NavMeshOverride>,
    mut enable_queue: ResMut<crate::player::navmesh_override::EnableNavMeshOverrideQueue>,
    mut target_queue: ResMut<crate::player::navmesh_override::SetNavMeshTargetQueue>,
) {
    for (mut player_action, player_entity) in player_query.iter_mut() {
        if !player_action.is_action_active {
            continue;
        }
        
        // Only process if in a walking state
        let is_walking_state = matches!(
            player_action.state,
            ActionState::WalkingToTargetBefore | ActionState::WalkingToTargetAfter
        );
        
        if !is_walking_state {
            continue;
        }
        
        if let Some(action_entity) = player_action.current_action {
            if let Ok(action) = action_query.get(action_entity) {
                // Initialize walk if needed
                if player_action.walk_state == WalkToTargetState::Idle {
                    // Enable NavMesh override
                    enable_queue.0.push(crate::player::navmesh_override::EnableNavMeshOverrideEvent {
                        entity: player_entity,
                    });
                    
                    // Set target
                    target_queue.0.push(crate::player::navmesh_override::SetNavMeshTargetEvent {
                        entity: player_entity,
                        target_position: action.walk_target_position,
                        target_entity: action.walk_target_entity,
                    });
                    
                    // Configure NavMesh settings
                    if let Ok(mut navmesh) = navmesh_query.get_mut(player_entity) {
                        navmesh.walk_speed = action.max_walk_speed;
                        navmesh.min_distance = action.min_distance_to_target;
                        navmesh.timeout = action.walk_timeout;
                        navmesh.elapsed_time = 0.0;
                    }
                    
                    player_action.walk_state = WalkToTargetState::Walking;
                    player_action.walk_timer = 0.0;
                    
                    info!("Walk-to-Target: Started walking for action {}", action.action_name);
                }
                
                // Monitor NavMesh status
                if player_action.walk_state == WalkToTargetState::Walking {
                    if let Ok(navmesh) = navmesh_query.get(player_entity) {
                        match navmesh.path_status.as_str() {
                            "Reached" => {
                                player_action.walk_state = WalkToTargetState::ReachedTarget;
                                info!("Walk-to-Target: Reached target");
                            }
                            "TimedOut" => {
                                player_action.walk_state = WalkToTargetState::TimedOut;
                                warn!("Walk-to-Target: Timed out");
                            }
                            _ => {
                                // Still walking
                                player_action.walk_timer += 0.016; // Approximate delta
                            }
                        }
                    }
                }
            }
        }
    }
}


/// System to process timed events during actions
pub fn process_action_events_system(
    mut player_query: Query<(&mut PlayerActionSystem, Entity, &Transform)>,
    mut action_query: Query<(&mut ActionSystem, Option<&Transform>)>,
    state_query: Query<&crate::player::player_state::PlayerStateSystem>,
    modes_query: Query<&crate::player::player_modes::PlayerModesSystem>,
    stats_query: Query<&crate::stats::stats_system::StatsSystem>,
    mut event_queue: ResMut<ActionEventTriggeredQueue>,
    mut remote_queue: ResMut<RemoteActionEventQueue>,
    mut camera_queue: ResMut<CameraEventQueue>,
    mut physics_queue: ResMut<PhysicsEventQueue>,
    mut state_change_queue: ResMut<StateChangeEventQueue>,
    mut weapon_queue: ResMut<WeaponEventQueue>,
    mut power_queue: ResMut<PowerEventQueue>,
    mut parenting_queue: ResMut<ParentingEventQueue>,
    time: Res<Time>,
) {
    for (mut player_action, player_entity, player_transform) in player_query.iter_mut() {
        if !player_action.is_action_active {
            continue;
        }
        
        if let Some(action_entity) = player_action.current_action {
            if let Ok((mut action, action_transform)) = action_query.get_mut(action_entity) {
                if !action.use_event_list || action.event_list.is_empty() {
                    continue;
                }
                
                // Initialize events on first frame
                if !player_action.events_active {
                    player_action.events_active = true;
                    player_action.event_start_time = time.elapsed_secs();
                    
                    // Reset all event triggered flags
                    for event in action.event_list.iter_mut() {
                        event.event_triggered = false;
                    }
                }
                
                let power_multiplier = action.player_state_control.power_drain_multiplier;
                let elapsed = time.elapsed_secs() - player_action.event_start_time;
                
                if action.use_accumulative_delay {
                    // Accumulative mode: Sequential events
                    let mut accumulated_time = 0.0;
                    let action_progress = player_action.action_timer / action.duration;
                    for event in action.event_list.iter_mut() {
                        if !event.event_triggered {
                            // Determine trigger time based on timing mode
                            let should_fire = if event.use_animation_timing {
                                // Animation-based: Check normalized time
                                action_progress >= event.animation_normalized_time
                            } else {
                                // Time-based: Check accumulated delay
                                accumulated_time += event.delay_to_activate;
                                elapsed >= accumulated_time
                            };
                            
                            if should_fire {
                                // Check condition before firing
                                if check_event_condition(
                                    &event.condition,
                                    action_progress,
                                    player_entity,
                                    player_transform,
                                    action_transform,
                                    &state_query,
                                    &modes_query,
                                    &stats_query,
                                ) {
                                    fire_action_event(
                                        event, 
                                        action_entity, 
                                        player_entity, 
                                        &mut event_queue, 
                                        &mut remote_queue, 
                                        &mut camera_queue, 
                                        &mut physics_queue, 
                                        &mut state_change_queue,
                                        &mut weapon_queue,
                                        &mut power_queue,
                                        &mut parenting_queue,
                                        power_multiplier,
                                    );
                                    event.event_triggered = true;
                                } else if !event.check_condition_continuously {
                                    // Mark as triggered even if condition failed (don't retry)
                                    event.event_triggered = true;
                                }
                            } else if !event.use_animation_timing {
                                break; // Stop checking further events (only for time-based)
                            }
                        } else if !event.use_animation_timing {
                            accumulated_time += event.delay_to_activate;
                        }
                    }
                } else {
                    // Parallel mode: All events check independently
                    let action_progress = player_action.action_timer / action.duration;
                    for event in action.event_list.iter_mut() {
                        if !event.event_triggered {
                            // Determine trigger time based on timing mode
                            let should_fire = if event.use_animation_timing {
                                // Animation-based: Check normalized time
                                action_progress >= event.animation_normalized_time
                            } else {
                                // Time-based: Check delay
                                elapsed >= event.delay_to_activate
                            };
                            
                            if should_fire {
                                // Check condition before firing
                                if check_event_condition(
                                    &event.condition,
                                    action_progress,
                                    player_entity,
                                    player_transform,
                                    action_transform,
                                    &state_query,
                                    &modes_query,
                                    &stats_query,
                                ) {
                                    fire_action_event(
                                        event, 
                                        action_entity, 
                                        player_entity, 
                                        &mut event_queue, 
                                        &mut remote_queue, 
                                        &mut camera_queue, 
                                        &mut physics_queue, 
                                        &mut state_change_queue,
                                        &mut weapon_queue,
                                        &mut power_queue,
                                        &mut parenting_queue,
                                        power_multiplier,
                                    );
                                    event.event_triggered = true;
                                } else if !event.check_condition_continuously {
                                    // Mark as triggered even if condition failed (don't retry)
                                    event.event_triggered = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn fire_action_event(
    event: &ActionEvent,
    action_entity: Entity,
    player_entity: Entity,
    event_queue: &mut ActionEventTriggeredQueue,
    remote_queue: &mut RemoteActionEventQueue,
    camera_queue: &mut CameraEventQueue,
    physics_queue: &mut PhysicsEventQueue,
    state_change_queue: &mut StateChangeEventQueue,
    weapon_queue: &mut WeaponEventQueue,
    power_queue: &mut PowerEventQueue,
    parenting_queue: &mut ResMut<ParentingEventQueue>,
    power_multiplier: f32,
) {
    if event.use_bevy_event {
        event_queue.0.push(ActionEventTriggered {
            action_entity,
            player_entity,
            event_name: event.bevy_event_name.clone(),
        });
        info!("Action Event: {} triggered", event.bevy_event_name);
    }
    
    if event.use_remote_event {
        remote_queue.0.push(RemoteActionEvent {
            event_name: event.remote_event_name.clone(),
            player_entity: if event.send_player_entity { Some(player_entity) } else { None },
        });
        info!("Remote Event: {} triggered", event.remote_event_name);
    }
    
    if event.use_camera_event {
        camera_queue.0.push(CameraEventTriggered {
            event_type: event.camera_event_type.clone(),
            player_entity,
            action_entity,
        });
        info!("Camera Event: {:?} triggered", event.camera_event_type);
    }
    
    if event.use_physics_event {
        let target_entity = if event.physics_target_self {
            player_entity
        } else {
            action_entity  // Target is the action entity
        };
        
        physics_queue.0.push(PhysicsEventTriggered {
            event_type: event.physics_event_type.clone(),
            target_entity,
            source_entity: player_entity,
        });
        info!("Physics Event: {:?} triggered on entity {:?}", event.physics_event_type, target_entity);
    }
    
    if event.use_state_change_event {
        state_change_queue.0.push(StateChangeEventTriggered {
            event_type: event.state_change_event_type.clone(),
            player_entity,
        });
        info!("State Change Event: {:?} triggered", event.state_change_event_type);
    }

    if event.use_weapon_event {
        weapon_queue.0.push(WeaponEventTriggered {
            event_type: event.weapon_event_type.clone(),
            player_entity,
            target_entity: event.weapon_target_entity,
        });
        info!("Weapon Event: {:?} triggered", event.weapon_event_type);
    }

    if event.use_power_event {
        let mut amount = match &event.power_event_type {
            PowerEventType::ConsumePower { amount } => *amount,
            PowerEventType::RestorePower { amount } => *amount,
            PowerEventType::DrainOverTime { amount_per_second, duration: _ } => *amount_per_second,
            PowerEventType::RequirePower { minimum_amount } => *minimum_amount,
            _ => 0.0,
        };
        
        // Apply multiplier for consumption and drain
        match &event.power_event_type {
            PowerEventType::ConsumePower { .. } | PowerEventType::DrainOverTime { .. } => {
                amount *= power_multiplier;
            }
            _ => {}
        }
        
        power_queue.0.push(PowerEventTriggered {
            event_type: event.power_event_type.clone(),
            player_entity,
            amount,
        });
        info!("Power Event: {:?} triggered with amount {} (mult: {})", event.power_event_type, amount, power_multiplier);
    }

    if event.use_parenting_event {
        parenting_queue.0.push(ParentingEventTriggered {
            event_type: event.parenting_event_type.clone(),
            player_entity,
        });
        info!("Parenting Event: {:?} triggered", event.parenting_event_type);
    }
}

/// Check if event condition is met
fn check_event_condition(
    condition: &EventCondition,
    action_progress: f32,
    player_entity: Entity,
    player_transform: &Transform,
    action_transform: Option<&Transform>,
    state_query: &Query<&crate::player::player_state::PlayerStateSystem>,
    modes_query: &Query<&crate::player::player_modes::PlayerModesSystem>,
    stats_query: &Query<&crate::stats::stats_system::StatsSystem>,
) -> bool {
    match condition {
        EventCondition::None => true,
        
        // Action progress conditions (fully implemented)
        EventCondition::ActionProgressGreaterThan(threshold) => action_progress > *threshold,
        EventCondition::ActionProgressLessThan(threshold) => action_progress < *threshold,
        EventCondition::ActionProgressBetween(min, max) => {
            action_progress >= *min && action_progress <= *max
        }
        
        // Player state conditions
        EventCondition::PlayerOnGround => {
            if let Ok(state_system) = state_query.get(player_entity) {
                // Check if "On Ground" state is active
                state_system.player_state_list.iter().any(|state| {
                    state.name.to_lowercase().contains("ground") && state.state_active
                })
            } else {
                warn!("PlayerOnGround condition: PlayerStateSystem not found");
                true // Default to true if component not found
            }
        }
        
        EventCondition::PlayerInAir => {
            if let Ok(state_system) = state_query.get(player_entity) {
                // Check if "In Air" or "Jumping" state is active
                state_system.player_state_list.iter().any(|state| {
                    (state.name.to_lowercase().contains("air") || 
                     state.name.to_lowercase().contains("jump")) && 
                    state.state_active
                })
            } else {
                warn!("PlayerInAir condition: PlayerStateSystem not found");
                true
            }
        }
        
        EventCondition::PlayerCrouching => {
            if let Ok(state_system) = state_query.get(player_entity) {
                // Check if "Crouching" state is active
                state_system.player_state_list.iter().any(|state| {
                    state.name.to_lowercase().contains("crouch") && state.state_active
                })
            } else {
                warn!("PlayerCrouching condition: PlayerStateSystem not found");
                true
            }
        }
        
        EventCondition::PlayerSprinting => {
            if let Ok(state_system) = state_query.get(player_entity) {
                // Check if "Sprinting" or "Running" state is active
                state_system.player_state_list.iter().any(|state| {
                    (state.name.to_lowercase().contains("sprint") || 
                     state.name.to_lowercase().contains("run")) && 
                    state.state_active
                })
            } else {
                warn!("PlayerSprinting condition: PlayerStateSystem not found");
                true
            }
        }
        
        // Health conditions
        EventCondition::HealthGreaterThan(threshold) => {
            if let Ok(stats) = stats_query.get(player_entity) {
                if let Some(health) = stats.get_derived_stat(crate::stats::types::DerivedStat::CurrentHealth) {
                    *health > *threshold
                } else {
                    warn!("HealthGreaterThan condition: CurrentHealth stat not found");
                    true
                }
            } else {
                warn!("HealthGreaterThan condition: StatsSystem not found");
                true
            }
        }
        
        EventCondition::HealthLessThan(threshold) => {
            if let Ok(stats) = stats_query.get(player_entity) {
                if let Some(health) = stats.get_derived_stat(crate::stats::types::DerivedStat::CurrentHealth) {
                    *health < *threshold
                } else {
                    warn!("HealthLessThan condition: CurrentHealth stat not found");
                    true
                }
            } else {
                warn!("HealthLessThan condition: StatsSystem not found");
                true
            }
        }
        
        // Distance conditions
        EventCondition::DistanceToTargetLessThan(threshold) => {
            if let Some(action_transform) = action_transform {
                let distance = player_transform.translation.distance(action_transform.translation);
                distance < *threshold
            } else {
                warn!("DistanceToTargetLessThan condition: Action transform not found");
                true
            }
        }
        
        EventCondition::DistanceToTargetGreaterThan(threshold) => {
            if let Some(action_transform) = action_transform {
                let distance = player_transform.translation.distance(action_transform.translation);
                distance > *threshold
            } else {
                warn!("DistanceToTargetGreaterThan condition: Action transform not found");
                true
            }
        }
        
        // Custom condition (placeholder for future extension)
        EventCondition::CustomCondition(name) => {
            // TODO: Implement custom condition registry
            // For now, check if it matches a player state name
            if let Ok(state_system) = state_query.get(player_entity) {
                state_system.player_state_list.iter().any(|state| {
                    state.name.to_lowercase() == name.to_lowercase() && state.state_active
                })
            } else {
                warn!("CustomCondition '{}': PlayerStateSystem not found", name);
                true
            }
        }
    }
}

/// System to block specific inputs during actions
pub fn block_action_inputs_system(
    mut player_query: Query<(&PlayerActionSystem, &mut InputState)>,
    action_query: Query<&ActionSystem>,
) {
    for (player_action, mut input) in player_query.iter_mut() {
        if !player_action.is_action_active { continue; }
        let Some(action_ent) = player_action.current_action else { continue };
        let Ok(action) = action_query.get(action_ent) else { continue };
        
        if action.player_state_control.disable_camera_zoom {
            input.zoom_in_pressed = false;
            input.zoom_out_pressed = false;
        }
    }
}
