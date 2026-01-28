//! Pressure Plate Device
//!
//! A pressure plate that detects objects on it and triggers events.
//! Supports multiple objects, tag filtering, and delayed state changes.

use bevy::prelude::*;
// use bevy::ecs::event::Events;
use avian3d::prelude::*;
use std::collections::HashSet;
use std::time::Duration;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Pressure plate component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PressurePlate {
    /// Minimum distance to trigger (for position-based detection)
    pub min_distance: f32,
    
    /// Tags to ignore (e.g., "Player")
    pub tags_to_ignore: HashSet<String>,
    
    /// Is the plate currently being used?
    pub using_plate: bool,
    
    /// Has the unlock function been called?
    pub active_function_called: bool,
    
    /// Has the lock function been called?
    pub disable_function_called: bool,
    
    /// Objects currently on the plate
    pub objects: HashSet<Entity>,
    
    /// The plate entity (visual)
    pub plate: Option<Entity>,
    
    /// Final position to reach (for animation)
    pub final_position: Option<Vec3>,
    
    /// Current state timer (for delayed activation/deactivation)
    pub state_timer: f32,
    
    /// Delay before deactivating (in seconds)
    pub deactivation_delay: f32,
}

impl Default for PressurePlate {
    fn default() -> Self {
        let mut tags_to_ignore = HashSet::new();
        tags_to_ignore.insert("Player".to_string());
        
        Self {
            min_distance: 0.1,
            tags_to_ignore,
            using_plate: false,
            active_function_called: false,
            disable_function_called: false,
            objects: HashSet::new(),
            plate: None,
            final_position: None,
            state_timer: 0.0,
            deactivation_delay: 1.0,
        }
    }
}

// ============================================================================
// EVENTS
// ============================================================================

/// Event triggered when pressure plate is activated
#[derive(Debug, Clone, Event)]
pub struct PressurePlateActivated {
    pub plate_entity: Entity,
    pub objects_on_plate: HashSet<Entity>,
}

/// Event triggered when pressure plate is deactivated
#[derive(Debug, Clone, Event)]
pub struct PressurePlateDeactivated {
    pub plate_entity: Entity,
}

#[derive(Resource, Default)]
pub struct PressurePlateActivatedQueue(pub Vec<PressurePlateActivated>);

#[derive(Resource, Default)]
pub struct PressurePlateDeactivatedQueue(pub Vec<PressurePlateDeactivated>);

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to handle collision detection for pressure plates
pub fn handle_pressure_plate_collisions(
    mut plate_query: Query<(Entity, &mut PressurePlate, &Transform, Option<&Collider>)>,
    // collision_events: Res<Events<Collision>>,
    transform_query: Query<&Transform>,
    mut activated_queue: ResMut<PressurePlateActivatedQueue>,
    mut deactivated_queue: ResMut<PressurePlateDeactivatedQueue>,
) {
    // Note: In Bevy 0.18 with avian3d, collision detection works differently
    // This is a simplified version
    
    for (entity, mut plate, plate_transform, maybe_collider) in plate_query.iter_mut() {
        // Collect collisions for this plate
        let mut objects_on_plate = HashSet::new();
        
        // for event in collision_events.iter_current_update_events() {
             // Logic to check if event involves this plate (entity and collider)
             // Simplified for now as we don't have full interaction logic
        // }
        
        // Check for objects in proximity (simplified collision detection)
        // In a real implementation, you'd use avian3d's collision detection system
        
        if objects_on_plate.is_empty() {
            // No objects on plate
            if !plate.disable_function_called && plate.objects.is_empty() {
                plate.state_timer += 0.016; // Approximate delta time
                
                if plate.state_timer >= plate.deactivation_delay {
                    plate.using_plate = false;
                    plate.disable_function_called = true;
                    plate.active_function_called = false;
                    
                    deactivated_queue.0.push(PressurePlateDeactivated {
                        plate_entity: entity,
                    });
                    
                    plate.state_timer = 0.0;
                }
            }
        } else {
            // Objects on plate
            plate.state_timer = 0.0;
            
            if !plate.active_function_called {
                plate.active_function_called = true;
                plate.disable_function_called = false;
                plate.using_plate = true;
                
                activated_queue.0.push(PressurePlateActivated {
                    plate_entity: entity,
                    objects_on_plate: objects_on_plate.clone(),
                });
            }
            
            plate.objects = objects_on_plate;
        }
    }
}

/// System to handle position-based pressure plate detection
pub fn update_pressure_plate_position(
    mut plate_query: Query<(Entity, &mut PressurePlate, &Transform)>,
    object_query: Query<(Entity, &Transform, &Collider, Option<&Name>)>,
    mut activated_queue: ResMut<PressurePlateActivatedQueue>,
    mut deactivated_queue: ResMut<PressurePlateDeactivatedQueue>,
) {
    for (entity, mut plate, plate_transform) in plate_query.iter_mut() {
        let mut objects_on_plate = HashSet::new();
        
        // Check distance to objects
        for (obj_entity, object_transform, collider, maybe_name) in object_query.iter() {
            // Skip ignored tags
            if let Some(name) = maybe_name {
                if plate.tags_to_ignore.contains(&name.to_string()) {
                    continue;
                }
            }
            
            let distance = plate_transform
                .translation
                .distance(object_transform.translation);
            
            if distance <= plate.min_distance {
                objects_on_plate.insert(obj_entity);
            }
        }
        
        // Update state
        if objects_on_plate.is_empty() {
            if !plate.disable_function_called && !plate.objects.is_empty() {
                plate.state_timer += 0.016;
                
                if plate.state_timer >= plate.deactivation_delay {
                    plate.using_plate = false;
                    plate.disable_function_called = true;
                    plate.active_function_called = false;
                    
                    deactivated_queue.0.push(PressurePlateDeactivated {
                        plate_entity: entity,
                    });
                    
                    plate.state_timer = 0.0;
                    plate.objects.clear();
                }
            }
        } else {
            plate.state_timer = 0.0;
            
            if !plate.active_function_called {
                plate.active_function_called = true;
                plate.disable_function_called = false;
                plate.using_plate = true;
                
                activated_queue.0.push(PressurePlateActivated {
                    plate_entity: entity,
                    objects_on_plate: objects_on_plate.clone(),
                });
            }
            
            plate.objects = objects_on_plate;
        }
    }
}

/// System to animate pressure plate movement
pub fn animate_pressure_plate(
    mut plate_query: Query<(&PressurePlate, &mut Transform)>,
    time: Res<Time>,
) {
    for (plate, mut transform) in plate_query.iter_mut() {
        if let Some(final_pos) = plate.final_position {
            let target_y = if plate.using_plate {
                final_pos.y
            } else {
                0.0 // Original position
            };
            
            let current_y = transform.translation.y;
            let new_y = current_y.lerp(target_y, time.delta_secs() * 5.0);
            
            transform.translation.y = new_y;
        }
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl PressurePlate {
    /// Add an object to the plate
    pub fn add_object(&mut self, entity: Entity) {
        self.objects.insert(entity);
        self.using_plate = true;
        self.active_function_called = true;
        self.disable_function_called = false;
        self.state_timer = 0.0;
    }
    
    /// Remove an object from the plate
    pub fn remove_object(&mut self, entity: Entity) {
        self.objects.remove(&entity);
        
        if self.objects.is_empty() {
            self.state_timer = 0.0;
        }
    }
    
    /// Check if plate is active
    pub fn is_active(&self) -> bool {
        self.using_plate && !self.objects.is_empty()
    }
    
    /// Clear all objects from plate
    pub fn clear_objects(&mut self) {
        self.objects.clear();
        self.using_plate = false;
        self.active_function_called = false;
        self.disable_function_called = true;
        self.state_timer = 0.0;
    }
    
    /// Set tags to ignore
    pub fn set_tags_to_ignore(&mut self, tags: Vec<String>) {
        self.tags_to_ignore = tags.into_iter().collect();
    }
    
    /// Add tag to ignore
    pub fn add_tag_to_ignore(&mut self, tag: String) {
        self.tags_to_ignore.insert(tag);
    }
    
    /// Remove tag from ignore list
    pub fn remove_tag_to_ignore(&mut self, tag: &str) {
        self.tags_to_ignore.remove(tag);
    }
}

// ============================================================================
// EVENTS HANDLER
// ============================================================================

/// System to handle pressure plate events
pub fn handle_pressure_plate_events(
    mut activated_queue: ResMut<PressurePlateActivatedQueue>,
    mut deactivated_queue: ResMut<PressurePlateDeactivatedQueue>,
) {
    for event in activated_queue.0.drain(..) {
        info!(
            "Pressure plate {:?} activated with {} objects",
            event.plate_entity,
            event.objects_on_plate.len()
        );
    }
    
    for event in deactivated_queue.0.drain(..) {
        info!("Pressure plate {:?} deactivated", event.plate_entity);
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin for pressure plate system
pub struct PressurePlatePlugin;

impl Plugin for PressurePlatePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PressurePlate>()
            .init_resource::<PressurePlateActivatedQueue>()
            .init_resource::<PressurePlateDeactivatedQueue>()
            .add_systems(Update, (
                handle_pressure_plate_collisions,
                update_pressure_plate_position,
                animate_pressure_plate,
                handle_pressure_plate_events,
            ));
    }
}
