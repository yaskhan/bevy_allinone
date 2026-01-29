//! Free Fall System
//!
//! Monitors time in air and triggers a free fall state if the player falls for too long.

use bevy::prelude::*;

pub struct FreeFallPlugin;

impl Plugin for FreeFallPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<FreeFall>()
            .init_resource::<FreeFallEnterQueue>()
            .init_resource::<FreeFallExitQueue>()
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

/// Event data triggered when free fall begins
#[derive(Debug, Clone, Copy)]
pub struct FreeFallEnterEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct FreeFallEnterQueue(pub Vec<FreeFallEnterEvent>);

/// Event data triggered when free fall ends
#[derive(Debug, Clone, Copy)]
pub struct FreeFallExitEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct FreeFallExitQueue(pub Vec<FreeFallExitEvent>);

/// System to monitor air time and trigger free fall
pub fn update_free_fall_logic(
    mut query: Query<(Entity, &mut FreeFall)>,
    time: Res<Time>,
    mut enter_queue: ResMut<FreeFallEnterQueue>,
    mut exit_queue: ResMut<FreeFallExitQueue>,
) {
    for (entity, mut free_fall) in query.iter_mut() {
        if !free_fall.enabled {
            continue;
        }

        let is_grounded = false; // Placeholder

        if is_grounded {
            if free_fall.active {
                free_fall.active = false;
                free_fall.is_falling = false;
                exit_queue.0.push(FreeFallExitEvent { entity });
                info!("Free Fall: Ended (Grounded)");
            }
            free_fall.last_grounded_time = time.elapsed_secs();
        } else {
            let time_in_air = time.elapsed_secs() - free_fall.last_grounded_time;

            if !free_fall.active && time_in_air > free_fall.min_time_to_activate {
                free_fall.active = true;
                enter_queue.0.push(FreeFallEnterEvent { entity });
                info!("Free Fall: Activated!");
            }
        }
    }
}
