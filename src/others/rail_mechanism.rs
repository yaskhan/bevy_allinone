use bevy::prelude::*;

/// Moves along a rail path.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct RailMechanism {
    pub points: Vec<Vec3>,
    pub speed: f32,
    pub progress: f32,
}

impl Default for RailMechanism {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            speed: 1.0,
            progress: 0.0,
        }
    }
}

pub fn update_rail_mechanism(
    time: Res<Time>,
    mut query: Query<(&mut RailMechanism, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (mut rail, mut transform) in query.iter_mut() {
        if rail.points.len() < 2 {
            continue;
        }
        rail.progress = (rail.progress + rail.speed * delta).clamp(0.0, 1.0);
        let start = rail.points[0];
        let end = rail.points[1];
        transform.translation = start.lerp(end, rail.progress);
    }
}
