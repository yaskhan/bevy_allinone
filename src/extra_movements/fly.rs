//! Fly System
//!
//! Enables 6DOF flight movement for characters.

use bevy::prelude::*;
use crate::input::InputState; // Assuming InputState is available

pub struct FlyPlugin;

impl Plugin for FlyPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Fly>()
            .add_event::<ToggleFlyModeEvent>()
            .add_event::<FlyTurboEvent>()
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
    pub velocity: Vec3, // Simulated velocity for now, ideally coupled with RB
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

/// Event to toggle flight mode
#[derive(Event)]
pub struct ToggleFlyModeEvent {
    pub entity: Entity,
}

/// Event to toggle turbo speed
#[derive(Event)]
pub struct FlyTurboEvent {
    pub entity: Entity,
    pub active: bool,
}

/// System to handle flight mode toggling
pub fn handle_fly_mode_events(
    mut toggle_events: EventReader<ToggleFlyModeEvent>,
    mut turbo_events: EventReader<FlyTurboEvent>,
    mut query: Query<&mut Fly>,
) {
    for event in toggle_events.read() {
        if let Ok(mut fly) = query.get_mut(event.entity) {
            fly.active = !fly.active;
            fly.velocity = Vec3::ZERO; // Reset velocity on toggle
            info!("Fly System: Toggled active state to {} for {:?}", fly.active, event.entity);
        }
    }

    for event in turbo_events.read() {
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

        // Calculate target direction relative to camera/character facing
        // For simplicity, we assume generic movement in XZ plane relative to character forward
        let forward = global_transform.forward();
        let right = global_transform.right();
        let up = Vec3::Y;

        let move_input = input_state.move_direction; // Vec3 (x, y, z) where y might be 0 from input

        let mut target_velocity = Vec3::ZERO;

        // Horizontal movement
        target_velocity += forward * move_input.z * fly.fly_speed;
        target_velocity += right * move_input.x * fly.fly_speed;

        // Vertical movement (Space/Shift usually, or buttons)
        // Simulating vertical input check
        if input_state.jump { // Using jump as Up
            target_velocity += up * fly.vertical_speed;
        }
        // Needs a 'Down' input mapping in InputState ideally, assuming none for now or maybe Crouch?
        
        if fly.turbo_active {
            target_velocity *= fly.turbo_speed_multiplier;
        }

        // Apply to simulated velocity with friction/smoothing
        let friction_factor = 1.0 - (fly.friction * time.delta_secs()).clamp(0.0, 1.0);
        fly.velocity = fly.velocity * friction_factor + target_velocity * time.delta_secs();

        // Clamp max velocity
        if fly.velocity.length() > fly.max_velocity {
            fly.velocity = fly.velocity.normalize() * fly.max_velocity;
        }

        // Apply velocity to transform (Manual integration)
        if let Some(mut transform) = transform_opt {
            transform.translation += fly.velocity * time.delta_secs();
        }
    }
}
