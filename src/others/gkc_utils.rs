use bevy::prelude::*;

/// Utility helpers.
///
/// GKC reference: `GKC_Utils.cs`
pub struct GkcUtils;

impl GkcUtils {
    pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
        a.lerp(b, t.clamp(0.0, 1.0))
    }

    pub fn distance(a: Vec3, b: Vec3) -> f32 {
        a.distance(b)
    }
}
