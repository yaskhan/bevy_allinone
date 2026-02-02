use bevy::prelude::*;
use avian3d::prelude::{SpatialQuery, SpatialQueryFilter, LayerMask, LinearVelocity};

/// Rope constraint data for grappling hook.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GrapplingHookRope {
    pub max_length: f32,
    pub current_length: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub collision_mask: LayerMask,
    pub rope_hit_point: Option<Vec3>,
}

impl Default for GrapplingHookRope {
    fn default() -> Self {
        Self {
            max_length: 25.0,
            current_length: 25.0,
            stiffness: 25.0,
            damping: 4.0,
            collision_mask: LayerMask::ALL,
            rope_hit_point: None,
        }
    }
}

/// Apply rope constraint and simple collision for grappling hook.
pub fn update_grappling_hook_rope(
    spatial_query: SpatialQuery,
    time: Res<Time>,
    mut query: Query<(&GlobalTransform, &mut LinearVelocity, &mut GrapplingHookRope, &super::grappling_hook_system::GrapplingHookSystem)>,
) {
    for (player_transform, mut velocity, mut rope, hook) in query.iter_mut() {
        if !hook.active {
            rope.rope_hit_point = None;
            continue;
        }

        let Some(target) = hook.current_target else { continue };
        let origin = player_transform.translation();
        let dir = (target - origin).normalize_or_zero();
        let dist = origin.distance(target);

        // Rope collision: shorten if obstacle between player and target.
        let filter = SpatialQueryFilter::default().with_mask(rope.collision_mask);
        if let Some(hit) = spatial_query.cast_ray(origin, dir, dist, true, &filter) {
            rope.rope_hit_point = Some(hit.point);
            rope.current_length = origin.distance(hit.point);
        } else {
            rope.rope_hit_point = None;
            rope.current_length = rope.max_length;
        }

        // Spring-damper rope pull toward target (or hit point).
        let anchor = rope.rope_hit_point.unwrap_or(target);
        let to_anchor = anchor - origin;
        let length = to_anchor.length();
        if length > rope.current_length {
            let stretch = length - rope.current_length;
            let dir = to_anchor / length;
            let spring = dir * (stretch * rope.stiffness);
            let damping = -velocity.0 * rope.damping;
            velocity.0 += (spring + damping) * time.delta_secs();
        }
    }
}
