//! Sprite Animator System
//!
//! Manages sprite sheet animation states and logic for 2.5D/2D characters.

use bevy::prelude::*;
use crate::input::InputState; // Assuming InputState is available

pub struct SpriteAnimatorPlugin;

impl Plugin for SpriteAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SpriteAnimator>()
            .register_type::<SpriteAnimationState>()
            .add_systems(Update, (
                update_sprite_animation,
                handle_sprite_direction,
            ).chain());
    }
}

/// Enum representing the possible states for sprite animation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum SpriteAnimationState {
    #[default]
    Idle,
    Walk,
    Run,
    Jump,
    Fall,
    Land,
}

/// Component to manage sprite animation logic
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SpriteAnimator {
    pub active: bool,
    pub current_state: SpriteAnimationState,
    pub flip_x: bool,
    pub is_grounded: bool,
    pub velocity: Vec3,
}

impl Default for SpriteAnimator {
    fn default() -> Self {
        Self {
            active: true,
            current_state: SpriteAnimationState::Idle,
            flip_x: false,
            is_grounded: true,
            velocity: Vec3::ZERO,
        }
    }
}

/// System to update animation state based on movement and status
pub fn update_sprite_animation(
    mut query: Query<&mut SpriteAnimator>,
    input_state: Res<InputState>,
    // In a real scenario, we might query a CharacterController component for velocity/grounded state
    // For now, we simulate or rely on manual updates to SpriteAnimator fields from other systems
) {
    for mut animator in query.iter_mut() {
        if !animator.active {
            continue;
        }

        // Determine state based on priority
        let new_state = if !animator.is_grounded {
            if animator.velocity.y > 0.0 {
                SpriteAnimationState::Jump
            } else {
                SpriteAnimationState::Fall
            }
        } else {
            let speed = animator.velocity.xz().length(); // Assuming Y is up
            if speed > 5.0 { // threshold for run
                SpriteAnimationState::Run
            } else if speed > 0.1 {
                SpriteAnimationState::Walk
            } else {
                SpriteAnimationState::Idle
            }
        };

        if animator.current_state != new_state {
            animator.current_state = new_state;
            info!("Sprite Animator: Switched to state {:?}", new_state);
        }
        
        // Update data simulation (in real app, this comes from physics/controller)
        // Simulate velocity based on input for demonstration if needed, 
        // but ideally this is driven by the CharacterController
        if input_state.movement.length_squared() > 0.0 {
             // Just a simulation hack for state transition verification if no physics is hooked up yet
             if animator.is_grounded {
                 animator.velocity = (input_state.movement.normalize_or_zero() * 3.0).extend(0.0).xzy();
             }
        } else {
             animator.velocity = Vec3::ZERO;
        }
    }
}

/// System to handle sprite direction (flipping)
pub fn handle_sprite_direction(
    mut query: Query<(&mut SpriteAnimator, Option<&mut Sprite>)>,
    input_state: Res<InputState>,
) {
    for (mut animator, sprite_opt) in query.iter_mut() {
        if !animator.active {
            continue;
        }

        // Determine direction from input or velocity
        let move_x = input_state.movement.x;

        if move_x.abs() > 0.01 {
            let should_flip = move_x < 0.0;
            if animator.flip_x != should_flip {
                animator.flip_x = should_flip;
                
                // If there's an actual Sprite component, update it
                if let Some(mut sprite) = sprite_opt {
                    sprite.flip_x = should_flip;
                }
            }
        }
    }
}
