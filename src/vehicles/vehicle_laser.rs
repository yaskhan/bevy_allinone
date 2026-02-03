use bevy::prelude::*;
use avian3d::prelude::{SpatialQuery, SpatialQueryFilter, LayerMask};

/// Vehicle laser weapon helper.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VehicleLaser {
    pub enabled: bool,
    pub active: bool,
    pub range: f32,
    pub layer_mask: LayerMask,
    pub origin: Entity,
    pub direction: Entity,
    pub last_hit: Option<Vec3>,
}

impl Default for VehicleLaser {
    fn default() -> Self {
        Self {
            enabled: true,
            active: false,
            range: 100.0,
            layer_mask: LayerMask::ALL,
            origin: Entity::PLACEHOLDER,
            direction: Entity::PLACEHOLDER,
            last_hit: None,
        }
    }
}

pub fn update_vehicle_laser(
    spatial_query: SpatialQuery,
    transform_query: Query<&GlobalTransform>,
    mut query: Query<&mut VehicleLaser>,
) {
    for mut laser in query.iter_mut() {
        if !laser.enabled || !laser.active {
            continue;
        }
        let Ok(origin) = transform_query.get(laser.origin) else { continue };
        let Ok(direction) = transform_query.get(laser.direction) else { continue };
        let dir = direction.forward().as_vec3();
        let filter = SpatialQueryFilter::default().with_mask(laser.layer_mask);
        if let Some(hit) = spatial_query.cast_ray(origin.translation(), dir, laser.range, true, &filter) {
            laser.last_hit = Some(hit.point);
        } else {
            laser.last_hit = None;
        }
    }
}
