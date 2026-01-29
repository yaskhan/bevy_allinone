use bevy::prelude::*;
use crate::input::InputState;
use super::types::*;

pub fn update_action_system(
    mut start_action_queue: ResMut<StartActionEventQueue>,
    mut end_action_queue: ResMut<EndActionEventQueue>,
    mut player_query: Query<(Entity, &mut PlayerActionSystem, &mut Transform)>,
    mut action_query: Query<(Entity, &mut ActionSystem, &GlobalTransform)>,
    mut commands: Commands,
    time: Res<Time>,
    input: Res<InputState>,
) {
    // 1. Handle Start Action
    for event in start_action_queue.0.drain(..) {
        if let Ok((player_entity, mut player_action, _)) = player_query.get_mut(event.player_entity) {
            if let Ok((action_entity, action, _action_transform)) = action_query.get_mut(event.action_entity) {
                // Set Player State
                player_action.current_action = Some(action_entity);
                player_action.is_action_active = true;
                player_action.action_timer = 0.0;
                
                // TODO: Backup physics/gravity state here
                
                info!("Started action: {}", action.action_name);
                
                // Optional: Snap to start position if defined
                // if action.use_position_to_adjust_player { ... }
            }
        }
    }
    
    // 2. Update Active Actions
    for (player_entity, mut player_action, mut _player_transform) in player_query.iter_mut() {
        if !player_action.is_action_active { continue; }
        
        if let Some(action_entity) = player_action.current_action {
             if let Ok((_, action, _action_glob_transform)) = action_query.get(action_entity) {
                 player_action.action_timer += time.delta_secs();
                 
                 // Logic for playing the action (e.g. interpolating position)
                 // For now, we just wait for duration
                 
                 if player_action.action_timer >= action.duration {
                     // Finish Action
                     end_action_queue.0.push(EndActionEvent {
                         player_entity,
                         action_entity,
                     });
                 }
             } else {
                 // Action entity missing?
                 player_action.is_action_active = false;
                 player_action.current_action = None;
             }
        }
    }
    
    // 3. Handle End Action
    for event in end_action_queue.0.drain(..) {
        if let Ok((_, mut player_action, _)) = player_query.get_mut(event.player_entity) {
            if player_action.current_action == Some(event.action_entity) {
                player_action.is_action_active = false;
                player_action.current_action = None;
                
                // TODO: Restore physics/gravity state
                info!("Ended action");
            }
        }
    }
    
    // 4. Input Detection (Simple Proximity Check for now)
    // In a real implementation this would be more optimized or use the existing spatial query system
    if input.interact_pressed {
         for (player_entity, player_action, player_transform) in player_query.iter() {
             if player_action.is_action_active { continue; }
             
             for (action_entity, action, action_transform) in action_query.iter() {
                 if !action.is_active { continue; }
                 
                 let dist = player_transform.translation.distance(action_transform.translation());
                 if dist <= action.min_distance {
                     // Check Angle if needed
                     let to_action = (action_transform.translation() - player_transform.translation).normalize();
                     let _dot = player_transform.forward().dot(to_action);
                     
                     // Convert min_angle (degrees) to dot product threshold roughly
                     // This is a simple check
                     
                     start_action_queue.0.push(StartActionEvent {
                         player_entity,
                         action_entity,
                     });
                     break; // Only start one action
                 }
             }
         }
    }
}
