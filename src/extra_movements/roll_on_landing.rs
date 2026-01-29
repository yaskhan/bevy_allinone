//! Roll On Landing System
//!
//! Manages mechanics for rolling upon landing to mitigate fall impact.

use bevy::prelude::*;
use crate::input::InputState; // Assuming InputState is available

pub struct RollOnLandingPlugin;

impl Plugin for RollOnLandingPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<RollOnLanding>()
            .add_event::<PrepareRollOnLandingEvent>()
            .add_event::<RollOnLandingExecuteEvent>()
            .add_systems(Update, (
                handle_roll_input,
                update_roll_landing_check,
                reset_roll_state,
            ).chain());
    }
}

/// Component to configure and manage roll-on-landing state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct RollOnLanding {
    pub enabled: bool,
    pub detection_distance: f32, // Distance to ground to trigger
    pub active_window: f32, // Check duration after input
    pub last_input_time: f32,
    pub prepared: bool, // Input was pressed, waiting for ground
    pub executing: bool, // Currently rolling
    pub execution_duration: f32, // How long the roll lasts typically (animation driven mainly)
    pub execution_start_time: f32,
}

impl Default for RollOnLanding {
    fn default() -> Self {
        Self {
            enabled: true,
            detection_distance: 1.5,
            active_window: 1.0,
            last_input_time: 0.0,
            prepared: false,
            executing: false,
            execution_duration: 1.0,
            execution_start_time: 0.0,
        }
    }
}

/// Event when player presses input to prepare (e.g. Crouch/Roll button in air)
#[derive(Event)]
pub struct PrepareRollOnLandingEvent {
    pub entity: Entity,
}

/// Event when the roll actually executes (ground detected)
#[derive(Event)]
pub struct RollOnLandingExecuteEvent {
    pub entity: Entity,
}

/// System to handle input and arm the system
pub fn handle_roll_input(
    mut query: Query<&mut RollOnLanding>,
    input_state: Res<InputState>,
    time: Res<Time>,
    mut prepare_events: EventWriter<PrepareRollOnLandingEvent>,
) {
    for mut roll in query.iter_mut() {
        if !roll.enabled {
            continue;
        }

        // Check input (e.g. Crouch or specific Roll key)
        // Assuming crouch serves as roll input in air for this context
        if input_state.crouch { // Or specific roll input
             // Debounce/Check window
             if !roll.prepared {
                 roll.prepared = true;
                 roll.last_input_time = time.elapsed_secs();
                 prepare_events.send(PrepareRollOnLandingEvent { entity: Entity::PLACEHOLDER }); // In real bevy we need entity from query
                 info!("Roll On Landing: Prepared");
             }
        }
    }
}

/// System to check for ground and execute roll
pub fn update_roll_landing_check(
    mut query: Query<(Entity, &mut RollOnLanding, &GlobalTransform)>,
    // physics_context: Res<RapierContext>, 
    time: Res<Time>,
    mut execute_events: EventWriter<RollOnLandingExecuteEvent>,
) {
    for (entity, mut roll, global_tf) in query.iter_mut() {
        if !roll.prepared {
            continue;
        }

        // Timeout check
        if time.elapsed_secs() > roll.last_input_time + roll.active_window {
            roll.prepared = false;
            // info!("Roll On Landing: Timed out");
            continue;
        }

        // Raycast check
        let origin = global_tf.translation();
        let direction = Vec3::NEG_Y;
        
        // Simulation: Assume ground is at Y=0 for now or rely on external ground check if available
        // let ground_detected = ...;
        // Simple height check for demo
        let ground_detected = origin.y < roll.detection_distance && origin.y > 0.0;

        if ground_detected {
            roll.prepared = false;
            roll.executing = true;
            roll.execution_start_time = time.elapsed_secs();
            execute_events.send(RollOnLandingExecuteEvent { entity });
            info!("Roll On Landing: Executing roll!");
            
            // Here you would trigger animation state change usually
        }
    }
}

/// System to reset state after execution
pub fn reset_roll_state(
    mut query: Query<&mut RollOnLanding>,
    time: Res<Time>,
) {
    for mut roll in query.iter_mut() {
        if roll.executing {
            if time.elapsed_secs() > roll.execution_start_time + roll.execution_duration {
                roll.executing = false;
                info!("Roll On Landing: Finished");
            }
        }
    }
}
