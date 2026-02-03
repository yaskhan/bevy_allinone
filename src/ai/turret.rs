use bevy::prelude::*;
use crate::ai::types::*;
use crate::combat::{Health, DamageEventQueue, DamageEvent, DamageType};
use avian3d::prelude::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Turret {
    pub rotation_speed: f32,
    pub max_range: f32,
    pub target: Option<Entity>,
    pub base_entity: Option<Entity>,
    pub cannon_entity: Option<Entity>,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TurretCombat {
    pub fire_rate: f32,
    pub last_fire_time: f32,
    pub damage: f32,
    pub damage_type: DamageType,
    pub require_line_of_sight: bool,
    pub use_hitscan: bool,
}

impl Default for TurretCombat {
    fn default() -> Self {
        Self {
            fire_rate: 0.4,
            last_fire_time: 0.0,
            damage: 10.0,
            damage_type: DamageType::Ranged,
            require_line_of_sight: true,
            use_hitscan: true,
        }
    }
}

impl Default for Turret {
    fn default() -> Self {
        Self {
            rotation_speed: 5.0,
            max_range: 30.0,
            target: None,
            base_entity: None,
            cannon_entity: None,
        }
    }
}

pub fn update_turrets(
    time: Res<Time>,
    mut query: Query<(&mut Turret, &GlobalTransform, Option<&AiPerception>)>,
    target_query: Query<(Entity, &GlobalTransform, &Health)>,
    mut transform_query: Query<&mut Transform>,
) {
    let delta = time.delta_secs();

    for (mut turret, turret_transform, perception) in query.iter_mut() {
        let my_pos = turret_transform.translation();

        if let Some(target_entity) = turret.target {
            if let Ok((_entity, target_transform, health)) = target_query.get(target_entity) {
                if health.current <= 0.0 {
                    turret.target = None;
                } else if my_pos.distance(target_transform.translation()) > turret.max_range {
                    turret.target = None;
                }
            } else {
                turret.target = None;
            }
        } else if let Some(perception) = perception {
            let mut best_target = None;
            let mut best_dist = turret.max_range;

            for &candidate in &perception.visible_targets {
                if let Ok((entity, target_gt, health)) = target_query.get(candidate) {
                    if health.current <= 0.0 {
                        continue;
                    }
                    let dist = my_pos.distance(target_gt.translation());
                    if dist <= best_dist {
                        best_dist = dist;
                        best_target = Some(entity);
                    }
                }
            }

            turret.target = best_target;
        }

        if let Some(target_entity) = turret.target {
            if let Ok((_entity, target_transform, _health)) = target_query.get(target_entity) {
                let target_pos = target_transform.translation();

                // Base rotates only on Y
                if let Some(base_entity) = turret.base_entity {
                    if let Ok(mut base_xf) = transform_query.get_mut(base_entity) {
                        let target_dir = (target_pos - my_pos).normalize();
                        let target_dir_flat = Vec3::new(target_dir.x, 0.0, target_dir.z).normalize();
                        let target_rotation = Quat::from_rotation_arc(Vec3::Z, target_dir_flat);
                        base_xf.rotation = base_xf.rotation.slerp(target_rotation, turret.rotation_speed * delta);
                    }
                }

                // Cannon/Head rotates on X and Y
                if let Some(cannon_entity) = turret.cannon_entity {
                    if let Ok(mut cannon_xf) = transform_query.get_mut(cannon_entity) {
                        let target_dir = (target_pos - turret_transform.translation()).normalize();
                        let target_rotation = Quat::from_rotation_arc(Vec3::Z, target_dir);
                        cannon_xf.rotation = cannon_xf.rotation.slerp(target_rotation, turret.rotation_speed * delta);
                    }
                }
            }
        } else {
            // Idle rotation (patrol)
            if let Some(base_entity) = turret.base_entity {
                if let Ok(mut base_xf) = transform_query.get_mut(base_entity) {
                    base_xf.rotate_y(turret.rotation_speed * 0.5 * delta);
                }
            }
        }
    }
}

pub fn update_turret_firing(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut damage_queue: ResMut<DamageEventQueue>,
    turret_query: Query<(Entity, &Turret, &GlobalTransform, &mut TurretCombat)>,
    target_query: Query<&GlobalTransform>,
) {
    let now = time.elapsed_secs();

    for (entity, turret, turret_transform, mut combat) in turret_query.iter_mut() {
        let Some(target_entity) = turret.target else { continue };
        let Ok(target_gt) = target_query.get(target_entity) else { continue };

        if now - combat.last_fire_time < combat.fire_rate {
            continue;
        }

        let origin = turret_transform.translation();
        let target_pos = target_gt.translation();
        let to_target = target_pos - origin;
        let dist = to_target.length();
        if dist > turret.max_range {
            continue;
        }

        let mut can_fire = true;
        if combat.require_line_of_sight && combat.use_hitscan {
            let dir = Dir3::new(to_target).unwrap_or(Dir3::Z);
            let filter = SpatialQueryFilter::from_excluded_entities([entity]);
            if let Some(hit) = spatial_query.cast_ray(origin, dir, dist, true, &filter) {
                if hit.entity != target_entity {
                    can_fire = false;
                }
            }
        }

        if can_fire {
            damage_queue.0.push(DamageEvent {
                amount: combat.damage,
                damage_type: combat.damage_type,
                source: Some(entity),
                target: target_entity,
                position: Some(target_pos),
                direction: Some(to_target.normalize_or_zero()),
                ignore_shield: false,
            });
            combat.last_fire_time = now;
        }
    }
}
