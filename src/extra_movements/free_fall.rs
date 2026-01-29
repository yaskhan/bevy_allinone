//! Free Fall System
//!
//! Monitors time in air and triggers a free fall state if the player falls for too long.

use bevy::prelude::*;
// use crate::input::InputState; 

pub struct FreeFallPlugin;

impl Plugin for FreeFallPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<FreeFall>()
            .add_event::<FreeFallEnterEvent>()
            .add_event::<FreeFallExitEvent>()
            .add_systems(Update, (
                update_free_fall_logic,
            ).chain());
    }
}

/// Component to configure and manage free fall state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct FreeFall {
    pub enabled: bool,
    pub min_time_to_activate: f32,
    pub active: bool,
    
    // State
    pub last_grounded_time: f32,
    pub is_falling: bool,
}

impl Default for FreeFall {
    fn default() -> Self {
        Self {
            enabled: true,
            min_time_to_activate: 2.0,
            active: false,
            last_grounded_time: 0.0,
            is_falling: false,
        }
    }
}

/// Event triggered when free fall begins
#[derive(Event)]
pub struct FreeFallEnterEvent {
    pub entity: Entity,
}

/// Event triggered when free fall ends
#[derive(Event)]
pub struct FreeFallExitEvent {
    pub entity: Entity,
}

/// System to monitor air time and trigger free fall
pub fn update_free_fall_logic(
    mut query: Query<(Entity, &mut FreeFall)>, // Add grounded check component
    time: Res<Time>,
    mut enter_events: EventWriter<FreeFallEnterEvent>,
    mut exit_events: EventWriter<FreeFallExitEvent>,
) {
    for (entity, mut free_fall) in query.iter_mut() {
        if !free_fall.enabled {
            continue;
        }

        // Placeholder grounded check
        // let is_grounded = ...;
        let is_grounded = false; // Assume falling for testing

        if is_grounded {
            if free_fall.active {
                free_fall.active = false;
                free_fall.is_falling = false;
                exit_events.send(FreeFallExitEvent { entity });
                info!("Free Fall: Ended (Grounded)");
            }
            free_fall.last_grounded_time = time.elapsed_secs();
        } else {
            // Is in air
            let time_in_air = time.elapsed_secs() - free_fall.last_grounded_time;

            if !free_fall.active && time_in_air > free_fall.min_time_to_activate {
                free_fall.active = true;
                enter_events.send(FreeFallEnterEvent { entity });
                info!("Free Fall: Activated!");
            }
        }
    }
}
