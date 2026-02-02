use bevy::prelude::*;
use avian3d::prelude::{LayerMask, SpatialQuery, SpatialQueryFilter};
use super::ability_info::AbilityInfo;
use crate::stats::StatsSystem;

#[derive(Clone, Debug, Reflect)]
#[reflect(Clone, Default)]
pub struct DrainStatInfo {
    pub stat_to_increase: String,
    pub increase_amount: f32,
    pub drain_amount: f32,
    pub stat_to_drain_list: Vec<String>,
}

impl Default for DrainStatInfo {
    fn default() -> Self {
        Self {
            stat_to_increase: "current_stamina".to_string(),
            increase_amount: 5.0,
            drain_amount: 5.0,
            stat_to_drain_list: Vec::new(),
        }
    }
}

/// Drain stats from targets and add to player.
///
/// GKC reference: `drainStatSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DrainStatSystem {
    pub enabled: bool,
    pub layer_to_check: LayerMask,
    pub drain_stat_rate: f32,
    pub max_drain_distance: f32,
    pub start_drain_delay: f32,
    pub use_sphere_cast_all: bool,
    pub sphere_cast_radius: f32,
    pub drain_active: bool,
    pub drain_action_paused: bool,
    pub current_targets: Vec<Entity>,
    pub stats_info_list: Vec<DrainStatInfo>,
    pub timer: f32,
    pub delay_timer: f32,
}

impl Default for DrainStatSystem {
    fn default() -> Self {
        Self {
            enabled: true,
            layer_to_check: LayerMask::ALL,
            drain_stat_rate: 0.3,
            max_drain_distance: 20.0,
            start_drain_delay: 0.0,
            use_sphere_cast_all: false,
            sphere_cast_radius: 1.0,
            drain_active: false,
            drain_action_paused: false,
            current_targets: Vec::new(),
            stats_info_list: Vec::new(),
            timer: 0.0,
            delay_timer: 0.0,
        }
    }
}

pub fn update_drain_stat_system(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
    mut query: Query<(Entity, &mut DrainStatSystem, &AbilityInfo, &mut StatsSystem)>,
    mut target_stats_query: Query<&mut StatsSystem>,
) {
    let Some(camera) = camera_query.iter().next() else { return };
    let cam_pos = camera.translation();
    let cam_forward = camera.forward().as_vec3();

    for (entity, mut system, ability, mut player_stats) in query.iter_mut() {
        if ability.name != "DrainStat" {
            continue;
        }

        if !system.enabled || system.drain_action_paused {
            continue;
        }

        if ability.active && !system.drain_active {
            let filter = SpatialQueryFilter::default().with_mask(system.layer_to_check);
            system.current_targets.clear();

            if system.use_sphere_cast_all {
                let hits = spatial_query.cast_ray(
                    cam_pos,
                    cam_forward,
                    system.max_drain_distance,
                    true,
                    &filter,
                );
                if let Some(hit) = hits {
                    system.current_targets.push(hit.entity);
                }
            } else if let Some(hit) = spatial_query.cast_ray(
                cam_pos,
                cam_forward,
                system.max_drain_distance,
                true,
                &filter,
            ) {
                system.current_targets.push(hit.entity);
            }

            if !system.current_targets.is_empty() {
                system.drain_active = true;
                system.delay_timer = system.start_drain_delay;
                system.timer = 0.0;
            }
        }

        if system.drain_active {
            if system.delay_timer > 0.0 {
                system.delay_timer -= time.delta_secs();
                continue;
            }

            system.timer += time.delta_secs();
            if system.timer < system.drain_stat_rate {
                continue;
            }
            system.timer = 0.0;

            for info in &system.stats_info_list {
                if !info.stat_to_increase.is_empty() {
                    player_stats.increase_custom_stat(&info.stat_to_increase, info.increase_amount);
                }

                for stat_name in &info.stat_to_drain_list {
                    for target_entity in &system.current_targets {
                        if let Ok(mut target_stats) = target_stats_query.get_mut(*target_entity) {
                            target_stats.use_custom_stat(stat_name, info.drain_amount);
                        }
                    }
                }
            }
        }
    }
}
