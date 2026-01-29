use bevy::prelude::*;
use crate::character::CharacterMovementState;
use super::types::*;

/// Configuration for a specific bobbing state
#[derive(Debug, Clone, Reflect)]
pub struct BobPreset {
    pub pos_amount: Vec3,
    pub rot_amount: Vec3,
    pub pos_speed: Vec3,
    pub rot_speed: Vec3,
    pub transition_speed: f32,
    pub smooth: f32,
}

impl Default for BobPreset {
    fn default() -> Self {
        Self {
            pos_amount: Vec3::ZERO,
            rot_amount: Vec3::ZERO,
            pos_speed: Vec3::ZERO,
            rot_speed: Vec3::ZERO,
            transition_speed: 5.0,
            smooth: 8.0,
        }
    }
}

/// Active bobbing state for a camera
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraBobState {
    pub phase: f32,
    pub current_pos_offset: Vec3,
    pub current_rot_offset: Vec3,
    
    // Presets
    pub idle: BobPreset,
    pub walk: BobPreset,
    pub sprint: BobPreset,
    pub aim: BobPreset,
}

impl Default for CameraBobState {
    fn default() -> Self {
        Self {
            phase: 0.0,
            current_pos_offset: Vec3::ZERO,
            current_rot_offset: Vec3::ZERO,
            
            idle: BobPreset {
                pos_amount: Vec3::new(0.01, 0.015, 0.0),
                rot_amount: Vec3::new(0.2, 0.1, 0.1),
                pos_speed: Vec3::new(1.0, 2.0, 0.0),
                rot_speed: Vec3::new(1.0, 0.5, 0.5),
                ..default()
            },
            walk: BobPreset {
                pos_amount: Vec3::new(0.04, 0.06, 0.02),
                rot_amount: Vec3::new(0.8, 0.5, 0.4),
                pos_speed: Vec3::new(6.0, 12.0, 6.0),
                rot_speed: Vec3::new(6.0, 6.0, 6.0),
                ..default()
            },
            sprint: BobPreset {
                pos_amount: Vec3::new(0.08, 0.12, 0.04),
                rot_amount: Vec3::new(1.5, 1.0, 0.8),
                pos_speed: Vec3::new(8.0, 16.0, 8.0),
                rot_speed: Vec3::new(8.0, 8.0, 8.0),
                ..default()
            },
            aim: BobPreset {
                pos_amount: Vec3::new(0.005, 0.01, 0.0),
                rot_amount: Vec3::new(0.1, 0.05, 0.05),
                pos_speed: Vec3::new(1.0, 2.0, 0.0),
                rot_speed: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
        }
    }
}

pub fn update_camera_bob(
    time: Res<Time>,
    mut query: Query<(&CameraController, &mut CameraState, &mut CameraBobState)>,
    target_query: Query<&CharacterMovementState>,
) {
    let dt = time.delta_secs();
    
    for (controller, mut state, mut bob) in query.iter_mut() {
        let Some(target_ent) = controller.follow_target else { continue };
        let Ok(movement) = target_query.get(target_ent) else { continue };

        // 1. Select active preset (copy values to avoid borrow issues)
        let preset = if state.is_aiming {
            bob.aim.clone()
        } else if movement.is_sprinting {
            bob.sprint.clone()
        } else if movement.current_speed > 0.1 {
            bob.walk.clone()
        } else {
            bob.idle.clone()
        };

        // 2. Advance phase
        bob.phase += dt;
        let t = bob.phase;

        // 3. Calculate target sinusoidal offsets
        let target_pos = Vec3::new(
            (t * preset.pos_speed.x).sin() * preset.pos_amount.x,
            (t * preset.pos_speed.y).sin() * preset.pos_amount.y,
            (t * preset.pos_speed.z).cos() * preset.pos_amount.z,
        );

        let target_rot = Vec3::new(
            (t * preset.rot_speed.x).sin() * preset.rot_amount.x,
            (t * preset.rot_speed.y).sin() * preset.rot_amount.y,
            (t * preset.rot_speed.z).cos() * preset.rot_amount.z,
        );

        // 4. Smoothly interpolate current offsets
        bob.current_pos_offset = bob.current_pos_offset.lerp(target_pos, preset.smooth * dt);
        bob.current_rot_offset = bob.current_rot_offset.lerp(target_rot, preset.smooth * dt);

        // 5. Apply to CameraState for follow.rs to consume
        state.bob_offset = bob.current_pos_offset;
        
        state.noise_offset.x += bob.current_rot_offset.y;
        state.noise_offset.y += bob.current_rot_offset.x;
    }
}
