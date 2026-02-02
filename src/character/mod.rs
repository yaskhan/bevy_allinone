use bevy::prelude::*;

pub mod types;
pub mod systems;
pub mod spawn;

pub use types::*;
pub use spawn::*;

use systems::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .register_type::<CharacterController>()
            .register_type::<CharacterMovementState>()
            .register_type::<CharacterAnimationState>()
            .register_type::<FootIk>()
            .register_type::<HandIk>()
            .add_systems(Update, (
                movement::update_character_movement,
                rotation::update_character_rotation,
                animation::update_character_animation,
            ).chain())
            .add_systems(FixedUpdate, (
                movement::apply_character_physics,
                detection::check_ground_state,
                movement::update_friction_material,
                damage::handle_falling_damage,
                movement::handle_crouch_sliding,
                detection::handle_obstacle_detection,
                detection::handle_wall_running_detection,
            ));
    }
}
