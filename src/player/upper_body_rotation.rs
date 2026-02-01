//! Upper Body Rotation System
//!
//! Handles procedural rotation of the player's upper body (spine, chest) to face a target.

use bevy::prelude::*;

pub struct UpperBodyRotationPlugin;

impl Plugin for UpperBodyRotationPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UpperBodyRotation>()
            .register_type::<UpperBodyTarget>()
            .add_systems(Update, update_upper_body_rotation);
    }
}

/// Component to configure upper body rotation with proper IK constraints
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UpperBodyRotation {
    pub enabled: bool,
    pub rotation_speed: f32,
    pub max_horizontal_angle: f32,
    pub max_vertical_angle: f32,
    pub horizontal_enabled: bool,
    pub vertical_enabled: bool,
    pub spine_bone: Option<Entity>,
    pub chest_bone: Option<Entity>,
    /// How much of the rotation each bone handles (spine, chest)
    pub bone_rotation_split: Vec2,
    /// Offset angles for fine-tuning
    pub angle_offset: Vec2,
    /// Interpolation factor for smooth return to neutral
    pub return_to_neutral_speed: f32,
}

impl Default for UpperBodyRotation {
    fn default() -> Self {
        Self {
            enabled: true,
            rotation_speed: 8.0,
            max_horizontal_angle: 70.0,
            max_vertical_angle: 45.0,
            horizontal_enabled: true,
            vertical_enabled: true,
            spine_bone: None,
            chest_bone: None,
            bone_rotation_split: Vec2::new(0.6, 0.4), // Spine takes 60%, chest 40%
            angle_offset: Vec2::ZERO,
            return_to_neutral_speed: 3.0,
        }
    }
}

/// Component to specify the target for upper body rotation
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UpperBodyTarget {
    pub target_entity: Option<Entity>,
    pub target_position: Option<Vec3>,
}

impl Default for UpperBodyTarget {
    fn default() -> Self {
        Self {
            target_entity: None,
            target_position: None,
        }
    }
}

/// System to update upper body rotation based on target using proper IK math
pub fn update_upper_body_rotation(
    mut query: Query<(Entity, &UpperBodyRotation, &UpperBodyTarget, &GlobalTransform)>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (_player_entity, rotation_config, target, player_global) in query.iter() {
        if !rotation_config.enabled {
            continue;
        }

        // Determine target position
        let target_pos = if let Some(entity) = target.target_entity {
            global_transforms.get(entity).ok().map(|t| t.translation())
        } else {
            target.target_position
        };

        let dt = time.delta_secs();

        // Helper to apply rotation to a bone with proper IK constraints
        let mut rotate_bone = |bone_entity: Entity, rotation_weight: f32, has_target: bool| {
            let Ok(mut transform) = transforms.get_mut(bone_entity) else { return };

            if !has_target {
                // Return to neutral rotation smoothly
                let neutral = Quat::IDENTITY;
                transform.rotation = transform.rotation.slerp(
                    neutral,
                    dt * rotation_config.return_to_neutral_speed
                );
                return;
            }

            let target_pos = target_pos.unwrap();
            let player_pos = player_global.translation();

            // Get the bone's global transform for accurate direction calculation
            let Ok(bone_global) = global_transforms.get(bone_entity) else { return };
            let bone_pos = bone_global.translation();

            // Calculate look direction in world space
            let to_target = target_pos - bone_pos;
            let to_target_dir = to_target.normalize_or_zero();

            if to_target_dir.length_squared() < 0.001 {
                return;
            }

            // Get player's forward and up directions
            let player_forward = player_global.forward();
            let player_up = player_global.up();
            let player_right = player_global.right();

            // Calculate horizontal and vertical angles relative to player's facing
            // Project onto player's horizontal plane (forward-right)
            let horizontal_proj = Vec3::new(
                to_target_dir.dot(*player_forward),
                0.0,
                to_target_dir.dot(*player_right),
            );

            let horizontal_angle = if horizontal_proj.length_squared() > 0.001 {
                horizontal_proj.x.atan2(horizontal_proj.z).to_degrees()
            } else {
                0.0
            };

            // Calculate vertical angle (elevation)
            let vertical_angle = if rotation_config.vertical_enabled {
                let vertical_dot = to_target_dir.dot(*player_up);
                vertical_dot.asin().to_degrees()
            } else {
                0.0
            };

            // Apply constraints
            let clamped_horizontal = if rotation_config.horizontal_enabled {
                horizontal_angle.clamp(
                    -rotation_config.max_horizontal_angle,
                    rotation_config.max_horizontal_angle
                ) + rotation_config.angle_offset.x
            } else {
                0.0
            };

            let clamped_vertical = if rotation_config.vertical_enabled {
                vertical_angle.clamp(
                    -rotation_config.max_vertical_angle,
                    rotation_config.max_vertical_angle
                ) + rotation_config.angle_offset.y
            } else {
                0.0
            };

            // Scale by rotation weight (how much this bone contributes)
            let weighted_yaw = clamped_horizontal * rotation_weight;
            let weighted_pitch = clamped_vertical * rotation_weight;

            // Create target rotation (yaw around Y, pitch around X in local space)
            let yaw_rot = Quat::from_rotation_y(weighted_yaw.to_radians());
            let pitch_rot = Quat::from_rotation_x(-weighted_pitch.to_radians());
            let target_rotation = yaw_rot * pitch_rot;

            // Smoothly interpolate to target rotation
            transform.rotation = transform.rotation.slerp(
                target_rotation,
                dt * rotation_config.rotation_speed
            );
        };

        let has_target = target_pos.is_some();

        // Apply rotation to spine and chest with weighted distribution
        if let Some(spine) = rotation_config.spine_bone {
            rotate_bone(spine, rotation_config.bone_rotation_split.x, has_target);
        }
        if let Some(chest) = rotation_config.chest_bone {
            rotate_bone(chest, rotation_config.bone_rotation_split.y, has_target);
        }
    }
}
