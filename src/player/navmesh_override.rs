//! NavMesh Override System
//!
//! Allows overriding player control with external NavMesh targets (e.g. for cutscenes or AI).

use bevy::prelude::*;

pub struct NavMeshOverridePlugin;

impl Plugin for NavMeshOverridePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<NavMeshOverride>()
            .init_resource::<EnableNavMeshOverrideQueue>()
            .init_resource::<DisableNavMeshOverrideQueue>()
            .init_resource::<SetNavMeshTargetQueue>()
            .add_systems(Update, (
                handle_navmesh_override_events,
                update_navmesh_override,
            ).chain());
    }
}

/// Component to manage NavMesh override state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct NavMeshOverride {
    pub active: bool,
    pub target_entity: Option<Entity>,
    pub target_position: Option<Vec3>,
    pub path_status: String, // Placeholder for path status (e.g., "Moving", "Reached")
    
    // Walk settings
    pub walk_speed: f32,
    pub min_distance: f32,
    pub timeout: f32,
    pub elapsed_time: f32,
}

impl Default for NavMeshOverride {
    fn default() -> Self {
        Self {
            active: false,
            target_entity: None,
            target_position: None,
            path_status: "Idle".to_string(),
            walk_speed: 2.0,
            min_distance: 0.5,
            timeout: 10.0,
            elapsed_time: 0.0,
        }
    }
}

/// Event to enable NavMesh override
#[derive(Debug, Clone, Copy)]
pub struct EnableNavMeshOverrideEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct EnableNavMeshOverrideQueue(pub Vec<EnableNavMeshOverrideEvent>);

/// Event to disable NavMesh override
#[derive(Debug, Clone, Copy)]
pub struct DisableNavMeshOverrideEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct DisableNavMeshOverrideQueue(pub Vec<DisableNavMeshOverrideEvent>);

/// Event to set a new NavMesh target
#[derive(Debug, Clone, Copy)]
pub struct SetNavMeshTargetEvent {
    pub entity: Entity,
    pub target_position: Option<Vec3>,
    pub target_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct SetNavMeshTargetQueue(pub Vec<SetNavMeshTargetEvent>);

/// System to handle override events
pub fn handle_navmesh_override_events(
    mut enable_queue: ResMut<EnableNavMeshOverrideQueue>,
    mut disable_queue: ResMut<DisableNavMeshOverrideQueue>,
    mut target_queue: ResMut<SetNavMeshTargetQueue>,
    mut query: Query<&mut NavMeshOverride>,
) {
    for event in enable_queue.0.drain(..) {
        if let Ok(mut nav_override) = query.get_mut(event.entity) {
            nav_override.active = true;
            info!("NavMesh Override: Enabled for {:?}", event.entity);
        }
    }

    for event in disable_queue.0.drain(..) {
        if let Ok(mut nav_override) = query.get_mut(event.entity) {
            nav_override.active = false;
            nav_override.target_entity = None;
            nav_override.target_position = None;
            nav_override.path_status = "Idle".to_string();
            info!("NavMesh Override: Disabled for {:?}", event.entity);
        }
    }

    for event in target_queue.0.drain(..) {
        if let Ok(mut nav_override) = query.get_mut(event.entity) {
            if !nav_override.active {
                warn!("NavMesh Override: Received target for {:?} but override is inactive", event.entity);
                continue;
            }
            
            nav_override.target_position = event.target_position;
            nav_override.target_entity = event.target_entity;
            nav_override.path_status = "Moving".to_string();
            
            info!("NavMesh Override: Set target for {:?} to Pos: {:?}, Entity: {:?}", 
                event.entity, event.target_position, event.target_entity);
        }
    }
}

/// System to update override logic (Placeholder for actual NavMesh integration)
pub fn update_navmesh_override(
    mut query: Query<(&mut NavMeshOverride, &GlobalTransform, Option<&mut Transform>)>,
    global_transforms: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (mut nav_override, char_global, mut char_transform) in query.iter_mut() {
        if !nav_override.active {
            continue;
        }

        let mut dest = None;
        if let Some(pos) = nav_override.target_position {
            dest = Some(pos);
        } else if let Some(target_ent) = nav_override.target_entity {
            if let Ok(target_tf) = global_transforms.get(target_ent) {
                dest = Some(target_tf.translation());
            }
        }

        if let Some(destination) = dest {
            // Update elapsed time
            nav_override.elapsed_time += time.delta_secs();
            
            // Check timeout
            if nav_override.elapsed_time >= nav_override.timeout {
                if nav_override.path_status != "TimedOut" {
                    nav_override.path_status = "TimedOut".to_string();
                    warn!("NavMesh Override: Timed out after {} seconds", nav_override.timeout);
                }
                return;
            }
            
            // Check distance to target
            let dist = (destination - char_global.translation()).length();
            if dist < nav_override.min_distance {
                if nav_override.path_status != "Reached" {
                    nav_override.path_status = "Reached".to_string();
                    info!("NavMesh Override: Reached target (distance: {:.2})", dist);
                }
            } else {
                // Move towards target (Simple simulation)
                if let Some(transform) = char_transform.as_mut() {
                    // Placeholder for actual movement logic
                    // In a real implementation, this would use pathfinding
                    let direction = (destination - char_global.translation()).normalize_or_zero();
                    let movement = direction * nav_override.walk_speed * time.delta_secs();
                    transform.translation += movement;
                }
            }
        }
    }
}
