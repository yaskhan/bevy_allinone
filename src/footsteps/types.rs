use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct FootstepController {
    pub is_enabled: bool,
    /// Minimum distance moved before playing next footstep sound
    pub step_distance: f32,
    /// Multiplier for run speed frequency
    pub run_step_multiplier: f32,
    /// Distance moved since last footstep
    pub accumulated_distance: f32,
    /// Audio volume range (min, max)
    pub volume_range: (f32, f32),
    /// Radius of the noise signal for AI detection
    pub noise_radius: f32,
    /// Layer mask for ground detection raycast
    pub floor_mask: u32,
    /// Last played foot side (left/right)
    pub last_foot_left: bool,
}

impl Default for FootstepController {
    fn default() -> Self {
        Self {
            is_enabled: true,
            step_distance: 2.0,
            run_step_multiplier: 0.8,
            accumulated_distance: 0.0,
            volume_range: (0.8, 1.0),
            noise_radius: 5.0,
            floor_mask: 0xFFFF,
            last_foot_left: false,
        }
    }
}

/// Attach this to floor/surface entities
#[derive(Component, Debug, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct FootstepSurface {
    /// Identifier for the surface (e.g., "Concrete", "Wood", "Water")
    pub surface_id: String,
}

#[derive(Resource, Debug, Reflect, Clone, Default)]
#[reflect(Resource)]
pub struct FootstepAssets {
    /// Map surface_id to a list of audio handles
    pub surface_sounds: HashMap<String, Vec<Handle<AudioSource>>>,
    pub default_surface_id: String,
}

#[derive(Event, Debug, Clone)]
pub struct FootstepEvent {
    pub entity: Entity,
    pub surface_id: String,
    pub position: Vec3,
    pub normal: Vec3,
    pub volume: f32,
    pub noise_radius: f32,
    pub is_left: bool,
}

#[derive(Resource, Default)]
pub struct FootstepEventQueue(pub Vec<FootstepEvent>);

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct FootstepDecal {
    pub lifetime: f32,
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct FootstepDecalSettings {
    pub enabled: bool,
    pub size: Vec2,
    pub lifetime: f32,
    pub offset: f32,
    pub color: Color,
}

impl Default for FootstepDecalSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            size: Vec2::new(0.2, 0.35),
            lifetime: 8.0,
            offset: 0.01,
            color: Color::srgb(0.12, 0.12, 0.12),
        }
    }
}
