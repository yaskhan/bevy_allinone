use bevy::prelude::*;
use crate::input::InputState;

/// Grappling hook visual effect controller.
///
/// GKC reference: `grapplingHookEffect.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GrapplingHookEffect {
    pub speed: f32,
    pub spiral_speed: f32,
    pub distance_speed: f32,
    pub gravity: f32,
    pub segments: usize,
    pub magnitude: Vec2,
    pub frequency: f32,
    pub horizontal_offset: f32,
    pub noise_strength: f32,
    pub noise_scale: f32,

    pub active: bool,
    pub fixed_update_active: bool,

    pub origin: Entity,
    pub target: Entity,

    pub scaled_time_offset: f32,
    pub spiral_time_offset: f32,
    pub last_grapple_time: f32,

    pub points: Vec<Vec3>,
}

impl Default for GrapplingHookEffect {
    fn default() -> Self {
        Self {
            speed: 3.0,
            spiral_speed: 4.0,
            distance_speed: 2.0,
            gravity: 0.5,
            segments: 100,
            magnitude: Vec2::ONE,
            frequency: 0.5,
            horizontal_offset: 0.25,
            noise_strength: 0.5,
            noise_scale: 0.25,
            active: false,
            fixed_update_active: false,
            origin: Entity::PLACEHOLDER,
            target: Entity::PLACEHOLDER,
            scaled_time_offset: 0.0,
            spiral_time_offset: 0.0,
            last_grapple_time: 0.0,
            points: Vec::new(),
        }
    }
}

/// Activate grappling hook effect manually.
pub fn activate_grappling_hook_effect(
    mut query: Query<&mut GrapplingHookEffect>,
    time: Res<Time>,
) {
    for mut effect in query.iter_mut() {
        effect.active = true;
        effect.scaled_time_offset = 0.0;
        effect.spiral_time_offset = 0.0;
        effect.last_grapple_time = time.elapsed_secs();
        if effect.points.len() != effect.segments {
            effect.points = vec![Vec3::ZERO; effect.segments];
        }
    }
}

/// Deactivate grappling hook effect.
pub fn deactivate_grappling_hook_effect(
    mut query: Query<&mut GrapplingHookEffect>,
) {
    for mut effect in query.iter_mut() {
        effect.active = false;
        for point in effect.points.iter_mut() {
            *point = Vec3::ZERO;
        }
    }
}

/// Update grappling hook effect points.
pub fn update_grappling_hook_effect(
    time: Res<Time>,
    mut effect_query: Query<&mut GrapplingHookEffect>,
    transform_query: Query<&GlobalTransform>,
) {
    for mut effect in effect_query.iter_mut() {
        if !effect.active {
            continue;
        }

        let Ok(origin_transform) = transform_query.get(effect.origin) else { continue };
        let Ok(target_transform) = transform_query.get(effect.target) else { continue };

        let origin = origin_transform.translation();
        let target = target_transform.translation();
        let difference = target - origin;
        let direction = difference.normalize_or_zero();
        let distance_multiplier = (effect.scaled_time_offset * effect.distance_speed).clamp(0.0, 1.0);
        let distance = difference.length() * distance_multiplier;

        effect.scaled_time_offset += effect.speed * time.delta_secs();
        if distance_multiplier < 1.0 {
            effect.spiral_time_offset += effect.speed * effect.spiral_speed * time.delta_secs();
        }

        if effect.points.len() != effect.segments {
            effect.points = vec![Vec3::ZERO; effect.segments];
        }

        for i in 0..effect.points.len() {
            let t = i as f32 / effect.points.len() as f32;
            let forward_offset = direction * (t * distance);
            let mut current_position = origin + forward_offset;

            let curve_sample_pos = forward_offset.length() * effect.frequency - effect.spiral_time_offset;
            let sin_val = curve_sample_pos.sin();
            let sin_offset_h = (curve_sample_pos + effect.horizontal_offset).sin();

            let mut vertical_offset = origin_transform.up() * sin_val * effect.magnitude.y;
            let mut horizontal_offset = origin_transform.right() * sin_offset_h * effect.magnitude.x;

            let noise_sample_pos = -t * effect.noise_scale + effect.scaled_time_offset + effect.last_grapple_time;
            let noise = (noise_sample_pos * 12.9898).sin() * 43758.5453;
            let noise_val = noise.fract() * 2.0 - 1.0;
            vertical_offset += origin_transform.up() * (noise_val * effect.noise_strength);
            horizontal_offset += origin_transform.right() * (noise_val * effect.noise_strength);

            current_position += vertical_offset + horizontal_offset;
            current_position += Vec3::Y * (effect.gravity * t);
            effect.points[i] = current_position;
        }
    }
}
