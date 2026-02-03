use bevy::prelude::*;

/// Launch trajectory helper for vehicle projectiles.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LaunchTrajectory {
    pub initial_speed: f32,
    pub gravity: Vec3,
    pub time_step: f32,
    pub sample_count: usize,
    pub points: Vec<Vec3>,
}

impl Default for LaunchTrajectory {
    fn default() -> Self {
        Self {
            initial_speed: 20.0,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            time_step: 0.05,
            sample_count: 24,
            points: Vec::new(),
        }
    }
}

pub fn update_launch_trajectory(
    mut query: Query<(&GlobalTransform, &mut LaunchTrajectory)>,
) {
    for (transform, mut traj) in query.iter_mut() {
        traj.points.clear();
        let start = transform.translation();
        let forward = transform.forward().as_vec3();
        let velocity = forward * traj.initial_speed;
        for i in 0..traj.sample_count {
            let t = traj.time_step * i as f32;
            let pos = start + velocity * t + 0.5 * traj.gravity * t * t;
            traj.points.push(pos);
        }
    }
}
