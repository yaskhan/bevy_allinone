use bevy::prelude::*;
use super::types::*;

/// Trigger for a camera shake effect
#[derive(Debug, Clone, Reflect)]
pub struct ShakeRequest {
    /// Name of the shake preset
    pub name: String,
    /// Multiplier for intensity
    pub intensity: f32,
    /// Custom duration (optional, overrides preset)
    pub duration: Option<f32>,
}

/// Resource for queuing camera shakes from any system
#[derive(Resource, Default)]
pub struct ShakeQueue(pub Vec<ShakeRequest>);

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraShakeInstance {
    pub camera_entity: Option<Entity>,
    pub name: String,
    pub timer: f32,
    pub duration: f32,
    pub intensity: f32,
    pub pos_amount: Vec3,
    pub rot_amount: Vec3,
    pub pos_speed: Vec3,
    pub rot_speed: Vec3,
    pub pos_smooth: f32,
    pub rot_smooth: f32,
    pub current_pos: Vec3,
    pub current_rot: Vec3,
    pub decrease_in_time: bool,
}

impl Default for CameraShakeInstance {
    fn default() -> Self {
        Self {
            camera_entity: None,
            name: "Default Shake".to_string(),
            timer: 0.0,
            duration: 0.5,
            intensity: 1.0,
            pos_amount: Vec3::new(0.05, 0.05, 0.05),
            rot_amount: Vec3::new(2.0, 2.0, 2.0),
            pos_speed: Vec3::new(15.0, 15.0, 15.0),
            rot_speed: Vec3::new(15.0, 15.0, 15.0),
            pos_smooth: 8.0,
            rot_smooth: 8.0,
            current_pos: Vec3::ZERO,
            current_rot: Vec3::ZERO,
            decrease_in_time: true,
        }
    }
}

/// A world-space shake source that affects cameras within range
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PointShake {
    pub name: String,
    pub radius: f32,
    pub intensity: f32,
    pub active: bool,
    pub shake_using_distance: bool,
}

impl Default for PointShake {
    fn default() -> Self {
        Self {
            name: "Explosion".to_string(),
            radius: 10.0,
            intensity: 1.0,
            active: false,
            shake_using_distance: true,
        }
    }
}

/// Apply noise and active shakes to the camera state
pub fn update_camera_shake(
    time: Res<Time>,
    mut shakes_queue: ResMut<ShakeQueue>,
    mut camera_query: Query<(&CameraController, &mut CameraState, &GlobalTransform, Entity)>,
    mut commands: Commands,
    mut shakes_instances: Query<(Entity, &mut CameraShakeInstance)>,
    point_shakes: Query<(&PointShake, &GlobalTransform)>,
) {
    let dt = time.delta_secs();
    let elapsed = time.elapsed_secs();

    // 1. Trigger new shakes from queue
    for request in shakes_queue.0.drain(..) {
        if let Some((_, _, _, camera_ent)) = camera_query.iter_mut().next() {
            commands.spawn(CameraShakeInstance {
                camera_entity: Some(camera_ent),
                name: request.name,
                intensity: request.intensity,
                duration: request.duration.unwrap_or(0.5),
                ..default()
            });
        }
    }

    // 2. Proximity-based Point Shakes
    for (point, point_gt) in point_shakes.iter() {
        if !point.active { continue; }
        
        for (_, _, camera_gt, cam_ent) in camera_query.iter() {
            let dist = point_gt.translation().distance(camera_gt.translation());
            if dist <= point.radius {
                let mut int_mult = 1.0;
                if point.shake_using_distance {
                    int_mult = 1.0 - (dist / point.radius);
                }
                
                // Trigger an instance if not already shaking from this point?
                // For simplicity, let's just spawn a one-shot instance.
                // In a real system we might want a cooldown.
                commands.spawn(CameraShakeInstance {
                    camera_entity: Some(cam_ent),
                    name: point.name.clone(),
                    intensity: point.intensity * int_mult,
                    duration: 0.3,
                    ..default()
                });
            }
        }
    }

    // 3. Accumulate all active shakes
    let mut camera_offsets: std::collections::HashMap<Entity, (Vec3, Vec3)> = std::collections::HashMap::new();

    for (ent, mut shake) in shakes_instances.iter_mut() {
        let Some(cam_ent) = shake.camera_entity else {
            commands.entity(ent).despawn();
            continue;
        };
        shake.timer += dt;
        
        if shake.timer >= shake.duration {
            commands.entity(ent).despawn();
            continue;
        }

        let phase = elapsed + shake.timer; // Add timer to phase to make multiple instances unique
        
        // Calculate target positions
        let target_pos = Vec3::new(
            (phase * shake.pos_speed.x).sin() * shake.pos_amount.x,
            (phase * shake.pos_speed.y).sin() * shake.pos_amount.y,
            (phase * shake.pos_speed.z).cos() * shake.pos_amount.z,
        ) * shake.intensity;

        let target_rot = Vec3::new(
            (phase * shake.rot_speed.x).sin() * shake.rot_amount.x,
            (phase * shake.rot_speed.y).sin() * shake.rot_amount.y,
            (phase * shake.rot_speed.z).cos() * shake.rot_amount.z,
        ) * shake.intensity;

        let mut multiplier = 1.0;
        if shake.decrease_in_time {
            multiplier = 1.0 - (shake.timer / shake.duration);
        }

        shake.current_pos = shake.current_pos.lerp(target_pos * multiplier, shake.pos_smooth * dt);
        shake.current_rot = shake.current_rot.lerp(target_rot * multiplier, shake.rot_smooth * dt);

        let offsets = camera_offsets.entry(cam_ent).or_insert((Vec3::ZERO, Vec3::ZERO));
        offsets.0 += shake.current_pos;
        offsets.1 += shake.current_rot;
    }

    // 4. Apply to CameraState
    for (_, mut state, _, cam_ent) in camera_query.iter_mut() {
        let t = elapsed * 2.0;
        let noise_x = (t * 0.5).sin() * 0.05;
        let noise_y = (t * 0.8).cos() * 0.05;
        let noise_mult = if state.is_aiming { 0.3 } else { 1.0 };
        state.noise_offset = Vec2::new(noise_x * noise_mult, noise_y * noise_mult);

        if let Some((_total_pos, total_rot)) = camera_offsets.get(&cam_ent) {
            state.noise_offset.x += total_rot.y * 0.1;
            state.noise_offset.y += total_rot.x * 0.1;
        }
    }
}
