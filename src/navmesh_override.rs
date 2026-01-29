//! NavMesh Override System
//!
//! Allows overriding player control with external NavMesh targets (e.g. for cutscenes or AI).

use bevy::prelude::*;

pub struct NavMeshOverridePlugin;

impl Plugin for NavMeshOverridePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<NavMeshOverride>()
            .add_event::<EnableNavMeshOverrideEvent>()
            .add_event::<DisableNavMeshOverrideEvent>()
            .add_event::<SetNavMeshTargetEvent>()
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
}

impl Default for NavMeshOverride {
    fn default() -> Self {
        Self {
            active: false,
            target_entity: None,
            target_position: None,
            path_status: "Idle".to_string(),
        }
    }
}

/// Event to enable NavMesh override
#[derive(Event)]
pub struct EnableNavMeshOverrideEvent {
    pub entity: Entity,
}

/// Event to disable NavMesh override
#[derive(Event)]
pub struct DisableNavMeshOverrideEvent {
    pub entity: Entity,
}

/// Event to set a new NavMesh target
#[derive(Event)]
pub struct SetNavMeshTargetEvent {
    pub entity: Entity,
    pub target_position: Option<Vec3>,
    pub target_entity: Option<Entity>,
}

/// System to handle override events
pub fn handle_navmesh_override_events(
    mut enable_events: EventReader<EnableNavMeshOverrideEvent>,
    mut disable_events: EventReader<DisableNavMeshOverrideEvent>,
    mut target_events: EventReader<SetNavMeshTargetEvent>,
    mut query: Query<&mut NavMeshOverride>,
) {
    for event in enable_events.read() {
        if let Ok(mut nav_override) = query.get_mut(event.entity) {
            nav_override.active = true;
            info!("NavMesh Override: Enabled for {:?}", event.entity);
        }
    }

    for event in disable_events.read() {
        if let Ok(mut nav_override) = query.get_mut(event.entity) {
            nav_override.active = false;
            nav_override.target_entity = None;
            nav_override.target_position = None;
            nav_override.path_status = "Idle".to_string();
            info!("NavMesh Override: Disabled for {:?}", event.entity);
        }
    }

    for event in target_events.read() {
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
    // time: Res<Time>,
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
             // Placeholder: In a real system, we would pump this destination into the NavMesh agent component
             // For now, we just verify logic matches C# intent (setting target)
             
             // Simple debug check: if close enough, set to Reached
             let dist = (destination - char_global.translation()).length();
             if dist < 0.5 {
                 if nav_override.path_status != "Reached" {
                      nav_override.path_status = "Reached".to_string();
                      info!("NavMesh Override: Reached target");
                 }
             } else {
                 // Move towards target (Simple simulation)
                  if let Some(mut transform) = char_transform.as_mut() {
                      // transform.translation = transform.translation.lerp(destination, time.delta_secs());
                      // Commented out to prevent unintended movement without proper physics/navmesh integration
                  }
             }
        }
    }
}
