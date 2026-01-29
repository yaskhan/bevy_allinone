use bevy::prelude::*;
use avian3d::prelude::*;
use super::types::*;
use std::collections::HashSet;

pub fn update_event_triggers(
    mut triggers: Query<(
        Entity, 
        &mut EventTrigger, 
        &CollidingEntities, 
        Option<&mut PreviousCollisions>
    )>,
    mut commands: Commands,
    mut remote_event_queue: ResMut<RemoteEventQueue>,
) {
    for (entity, mut trigger, colliding_entities, prev_collisions_opt) in triggers.iter_mut() {
        // Ensure PreviousCollisions component exists
        let mut previous_collisions = if let Some(pc) = prev_collisions_opt {
             pc
        } else {
             commands.entity(entity).insert(PreviousCollisions::default());
             continue; // Wait for next frame to have the component
        };

        // Current set of colliding entities
        let current_set: HashSet<Entity> = colliding_entities.0.iter().cloned().collect();
        
        // ENTER EVENTS
        if trigger.on_enter && trigger.is_active {
             for &other_entity in current_set.difference(&previous_collisions.0) {
                 // Check limit
                 if let Some(limit) = trigger.trigger_limit {
                    if trigger.times_triggered >= limit { continue; }
                 }
                 
                 trigger.times_triggered += 1;
                 
                 for info in &trigger.enter_events {
                      fire_trigger_event(info, other_entity, &mut remote_event_queue);
                 }
             }
        }
        
        // EXIT EVENTS
        if trigger.on_exit && trigger.is_active {
             for &other_entity in previous_collisions.0.difference(&current_set) {
                 // Check limit
                 if let Some(limit) = trigger.trigger_limit {
                    if trigger.times_triggered >= limit { continue; }
                 }
                 
                 trigger.times_triggered += 1;
                 
                  for info in &trigger.exit_events {
                      fire_trigger_event(info, other_entity, &mut remote_event_queue);
                 }
             }
        }
        
        // Update history
        previous_collisions.0 = current_set;
    }
}

fn fire_trigger_event(
    info: &TriggerEventInfo,
    target: Entity,
    queue: &mut RemoteEventQueue
) {
    if info.use_remote_event {
        queue.0.push(RemoteEvent {
            name: info.event_name.clone(),
            target: Some(target), // We send it TO the entity that entered
            source: None,
            parameter: info.parameter.clone(),
        });
        
        info!("Event Triggered: {} -> {:?}", info.event_name, target);
    }
}


pub fn handle_remote_events(
    mut event_queue: ResMut<RemoteEventQueue>,
    receivers: Query<(Entity, &RemoteEventReceiver)>,
    mut commands: Commands,
) {
     for event in event_queue.0.drain(..) {
         // 1. If target is specified, check against it
         if let Some(target) = event.target {
             if let Ok((_, receiver)) = receivers.get(target) {
                 if receiver.events.contains(&event.name) {
                     // In a real ECS usage, we might trigger a specific component method or observer
                     // For now, we just log "Received" or trigger a Bevy Observer
                     info!("Remote Event '{}' received by {:?}", event.name, target);
                 }
             }
         } else {
             // Broadcast to all receivers matching name
             for (entity, receiver) in receivers.iter() {
                  if receiver.events.contains(&event.name) {
                       info!("Remote Event '{}' received by {:?} (Broadcast)", event.name, entity);
                  }
             }
         }
         
         // In Bevy 0.15+, we can also forward this to Observers
         // commands.trigger_targets(CustomObserverEvent { name: ... }, target);
     }
}
