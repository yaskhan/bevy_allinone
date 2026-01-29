//! Upper Body Rotation System
//!
//! Handles procedural rotation of the character's upper body (spine, chest) to face a target.

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

/// Component to configure upper body rotation
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UpperBodyRotation {
    pub enabled: bool,
    pub rotation_speed: f32,
    pub max_bending_angle: f32,
    pub horizontal_enabled: bool,
    pub vertical_enabled: bool,
    pub spine_bone: Option<Entity>,
    pub chest_bone: Option<Entity>,
}

impl Default for UpperBodyRotation {
    fn default() -> Self {
        Self {
            enabled: true,
            rotation_speed: 5.0,
            max_bending_angle: 90.0,
            horizontal_enabled: true,
            vertical_enabled: true,
            spine_bone: None,
            chest_bone: None,
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

/// System to update upper body rotation based on target
pub fn update_upper_body_rotation(
    mut query: Query<(&UpperBodyRotation, &UpperBodyTarget, &GlobalTransform)>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (rotation_config, target, char_global_transform) in query.iter_mut() {
        if !rotation_config.enabled {
            continue;
        }

        // Determine target position
        let mut target_pos = None;
        if let Some(entity) = target.target_entity {
            if let Ok(transform) = global_transforms.get(entity) {
                target_pos = Some(transform.translation());
            }
        } else if let Some(pos) = target.target_position {
            target_pos = Some(pos);
        }

        // If no target, simplify return (could add reset logic here)
        let target_pos = match target_pos {
            Some(pos) => pos,
            None => continue,
        };

        // Calculate Look Direction
        let char_pos = char_global_transform.translation();
        let look_dir_world = (target_pos - char_pos).normalize();

        // Apply rotation to bones
        // Note: Real implementation would need more complex IK math to handle hierarchy and constraints properly
        // This is a simplified version applying rotation towards target
        
        // Helper to rotate a bone
        let mut rotate_bone = |bone_entity: Entity| {
            if let Ok(mut transform) = transforms.get_mut(bone_entity) {
                // Get bone global rotation (simplified assumption: bone is child of character)
                // In a real scenario we'd need to calculate local rotation required to achieve global look dir
                
                // Simplified: Rotate towards target in local space or just look at
                // For now, let's assume we want to construct a rotation that faces the target
                // constrained by max limits.
                
                // TODO: Implement full IK math similar to original C# reference
                // (AngleAroundAxis, projection on planes, clamping)
                
                 let target_rot = transform.looking_at(target_pos, Vec3::Y).rotation;
                 transform.rotation = transform.rotation.slerp(target_rot, time.delta_secs() * rotation_config.rotation_speed);
            }
        };

        if rotation_config.horizontal_enabled || rotation_config.vertical_enabled {
             if let Some(spine) = rotation_config.spine_bone {
                rotate_bone(spine);
            }
            if let Some(chest) = rotation_config.chest_bone {
                 rotate_bone(chest);
            }
        }
    }
}
