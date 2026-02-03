use bevy::prelude::*;

/// Bezier spline data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct BezierSpline {
    pub points: Vec<Vec3>,
    pub looped: bool,
}

impl Default for BezierSpline {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            looped: false,
        }
    }
}

impl BezierSpline {
    pub fn evaluate(&self, t: f32) -> Vec3 {
        if self.points.len() < 4 {
            return self.points.get(0).copied().unwrap_or(Vec3::ZERO);
        }
        let clamped = t.clamp(0.0, 1.0);
        cubic_bezier(self.points[0], self.points[1], self.points[2], self.points[3], clamped)
    }
}

fn cubic_bezier(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;
    p0 * uuu + p1 * (3.0 * uu * t) + p2 * (3.0 * u * tt) + p3 * ttt
}
