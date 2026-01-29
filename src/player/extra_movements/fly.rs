//! Fly System
//!
//! Enables 6DOF flight movement for characters.

use bevy::prelude::*;
use crate::input::InputState;

pub struct FlyPlugin;

impl Plugin for FlyPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Fly>()
            .init_resource::<ToggleFlyModeQueue>()
            .init_resource::<FlyTurboQueue>()
            .add_systems(Update, (
                handle_fly_mode_events,
                update_fly_physics,
            ).chain());
    }
}

/// Component to configure and manage flight state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Fly {
    pub active: bool,
    pub fly_speed: f32,
    pub turbo_speed_multiplier: f32,
    pub friction: f32,
    pub max_velocity: f32,
    pub vertical_speed: f32,
    pub turbo_active: bool,
    pub velocity: Vec3, // Simulated velocity
}

impl Default for Fly {
    fn default() -> Self {
        Self {
            active: false,
            fly_speed: 10.0,
            turbo_speed_multiplier: 2.0,
            friction: 2.0,
            max_velocity: 50.0,
            vertical_speed: 5.0,
            turbo_active: false,
            velocity: Vec3::ZERO,
        }
    }
}

/// Event data to toggle flight mode
#[derive(Debug, Clone, Copy)]
pub struct ToggleFlyModeEvent {
    pub entity: Entity,
}

/// Queue for toggle events
#[derive(Resource, Default)]
pub struct ToggleFlyModeQueue(pub Vec<ToggleFlyModeEvent>);

/// Event data to toggle turbo speed
#[derive(Debug, Clone, Copy)]
pub struct FlyTurboEvent {
    pub entity: Entity,
    pub active: bool,
}

/// Queue for turbo events
#[derive(Resource, Default)]
pub struct FlyTurboQueue(pub Vec<FlyTurboEvent>);

/// System to handle flight mode settings
pub fn handle_fly_mode_events(
    mut toggle_queue: ResMut<ToggleFlyModeQueue>,
    mut turbo_queue: ResMut<FlyTurboQueue>,
    mut query: Query<&mut Fly>,
) {
    for event in toggle_queue.0.drain(..) {
        if let Ok(mut fly) = query.get_mut(event.entity) {
            fly.active = !fly.active;
            fly.velocity = Vec3::ZERO; // Reset velocity on toggle
            info!("Fly System: Toggled active state to {} for {:?}", fly.active, event.entity);
        }
    }

    for event in turbo_queue.0.drain(..) {
        if let Ok(mut fly) = query.get_mut(event.entity) {
            fly.turbo_active = event.active;
            info!("Fly System: Turbo active: {} for {:?}", fly.turbo_active, event.entity);
        }
    }
}

/// System to apply flight physics
pub fn update_fly_physics(
    mut query: Query<(&mut Fly, &GlobalTransform, Option<&mut Transform>)>,
    input_state: Res<InputState>,
    time: Res<Time>,
) {
    for (mut fly, global_transform, transform_opt) in query.iter_mut() {
        if !fly.active {
            continue;
        }

        let forward = global_transform.forward();
        let right = global_transform.right();
        let up = Vec3::Y;

        let move_input = input_state.movement;

        let mut target_velocity = Vec3::ZERO;

        // Horizontal movement
        target_velocity += *forward * move_input.y * fly.fly_speed;
        target_velocity += *right * move_input.x * fly.fly_speed;

        // Vertical movement
        if input_state.jump_pressed {
            target_velocity += up * fly.vertical_speed;
        }
        
        if fly.turbo_active {
            target_velocity *= fly.turbo_speed_multiplier;
        }

        // Apply to simulated velocity with friction/smoothing
        let friction_factor = 1.0 - (fly.friction * time.delta_secs()).clamp(0.0, 1.0);
        fly.velocity = fly.velocity * friction_factor + target_velocity * time.delta_secs();

        if fly.velocity.length() > fly.max_velocity {
            fly.velocity = fly.velocity.normalize() * fly.max_velocity;
        }

        if let Some(mut transform) = transform_opt {
            transform.translation += fly.velocity * time.delta_secs();
        }
    }
}
