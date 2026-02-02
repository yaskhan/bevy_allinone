use bevy::prelude::*;

use super::bezier_spline::BezierSpline;

/// Moves along a Bezier spline.
///
/// GKC reference: `SplineWalker.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SplineWalker {
    pub spline: Entity,
    pub speed: f32,
    pub t: f32,
}

impl Default for SplineWalker {
    fn default() -> Self {
        Self {
            spline: Entity::PLACEHOLDER,
            speed: 0.5,
            t: 0.0,
        }
    }
}

pub fn update_spline_walker(
    time: Res<Time>,
    splines: Query<&BezierSpline>,
    mut query: Query<(&mut SplineWalker, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (mut walker, mut transform) in query.iter_mut() {
        let Ok(spline) = splines.get(walker.spline) else { continue };
        walker.t = (walker.t + walker.speed * delta).clamp(0.0, 1.0);
        transform.translation = spline.evaluate(walker.t);
    }
}
