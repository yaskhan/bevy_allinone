//! Roll On Landing System
//!
//! Manages mechanics for rolling upon landing to mitigate fall impact.

use bevy::prelude::*;
use crate::input::InputState;

pub struct RollOnLandingPlugin;

impl Plugin for RollOnLandingPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<RollOnLanding>()
            .init_resource::<PrepareRollOnLandingQueue>()
            .init_resource::<RollOnLandingExecuteQueue>()
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
    pub detection_distance: f32,
    pub active_window: f32,
    pub last_input_time: f32,
    pub prepared: bool,
    pub executing: bool,
    pub execution_duration: f32,
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

/// Event data when player presses input to prepare
#[derive(Debug, Clone, Copy)]
pub struct PrepareRollOnLandingEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct PrepareRollOnLandingQueue(pub Vec<PrepareRollOnLandingEvent>);

/// Event data when the roll actually executes
#[derive(Debug, Clone, Copy)]
pub struct RollOnLandingExecuteEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct RollOnLandingExecuteQueue(pub Vec<RollOnLandingExecuteEvent>);

/// System to handle input and arm the system
pub fn handle_roll_input(
    mut query: Query<&mut RollOnLanding>,
    input_state: Res<InputState>,
    time: Res<Time>,
    mut prepare_queue: ResMut<PrepareRollOnLandingQueue>,
) {
    for mut roll in query.iter_mut() {
        if !roll.enabled {
            continue;
        }

        if input_state.crouch_pressed { 
             if !roll.prepared {
                 roll.prepared = true;
                 roll.last_input_time = time.elapsed_secs();
                 prepare_queue.0.push(PrepareRollOnLandingEvent { entity: Entity::PLACEHOLDER }); // In real use, need entity ID
                 info!("Roll On Landing: Prepared");
             }
        }
    }
}

/// System to check for ground and execute roll
pub fn update_roll_landing_check(
    mut query: Query<(Entity, &mut RollOnLanding, &GlobalTransform)>,
    time: Res<Time>,
    mut execute_queue: ResMut<RollOnLandingExecuteQueue>,
) {
    for (entity, mut roll, global_tf) in query.iter_mut() {
        if !roll.prepared {
            continue;
        }

        if time.elapsed_secs() > roll.last_input_time + roll.active_window {
            roll.prepared = false;
            continue;
        }

        let origin = global_tf.translation();
        // let direction = Vec3::NEG_Y;
        
        let ground_detected = origin.y < roll.detection_distance && origin.y > 0.0;

        if ground_detected {
            roll.prepared = false;
            roll.executing = true;
            roll.execution_start_time = time.elapsed_secs();
            execute_queue.0.push(RollOnLandingExecuteEvent { entity });
            info!("Roll On Landing: Executing roll!");
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
