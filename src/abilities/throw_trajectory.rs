use bevy::prelude::*;

/// Trajectory preview system for thrown objects.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ThrowObjectTrajectory {
    pub initial_speed: f32,
    pub gravity: Vec3,
    pub time_step: f32,
    pub sample_count: usize,
    pub points: Vec<Vec3>,
}

impl Default for ThrowObjectTrajectory {
    fn default() -> Self {
        Self {
            initial_speed: 10.0,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            time_step: 0.05,
            sample_count: 20,
            points: Vec::new(),
        }
    }
}

/// Update trajectory points based on current transform and settings.
pub fn update_throw_trajectory(
    mut query: Query<(&GlobalTransform, &mut ThrowObjectTrajectory)>,
) {
    for (transform, mut trajectory) in query.iter_mut() {
        trajectory.points.clear();
        let start = transform.translation();
        let forward = transform.forward().as_vec3();
        let velocity = forward * trajectory.initial_speed;

        for i in 0..trajectory.sample_count {
            let t = trajectory.time_step * i as f32;
            let position = start + velocity * t + 0.5 * trajectory.gravity * t * t;
            trajectory.points.push(position);
        }
    }
}
