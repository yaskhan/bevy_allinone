use bevy::prelude::*;
use avian3d::prelude::*;
use crate::ai::types::*;

pub fn update_ai_perception(
    mut ai_query: Query<(Entity, &GlobalTransform, &mut AiController, &AiPerception, Option<&CharacterFaction>, &AIPerceptionSettings)>,
    target_query: Query<(Entity, &GlobalTransform, Option<&CharacterFaction>)>,
    faction_system: Res<FactionSystem>,
    spatial_query: SpatialQuery,
) {
    for (entity, transform, mut ai, _perception, ai_faction, settings) in ai_query.iter_mut() {
        if ai.is_paused { continue; }
        if ai.state == AiBehaviorState::Flee || ai.state == AiBehaviorState::Dead { continue; }

        let mut closest_target = None;
        let mut min_dist = settings.range;
        let current_pos = transform.translation();
        let forward = transform.forward();
        let ai_faction_name = ai_faction.map(|f| f.name.as_str()).unwrap_or("Default");

        for (target_entity, target_transform, target_faction) in target_query.iter() {
            if target_entity == entity { continue; }

            let target_faction_name = target_faction.map(|f| f.name.as_str()).unwrap_or("Default");
            if faction_system.get_relation(ai_faction_name, target_faction_name) != FactionRelation::Enemy {
                continue;
            }

            let to_target = target_transform.translation() - current_pos;
            let dist = to_target.length();
            if dist > settings.range { continue; }

            let dir_to_target = to_target.normalize();
            if forward.angle_between(dir_to_target).to_degrees() > settings.fov / 2.0 {
                continue;
            }

            let origin = current_pos + Vec3::Y * 1.5;
            let target_eye = target_transform.translation() + Vec3::Y * 1.5;
            let direction_vec = (target_eye - origin).normalize();
            if let Ok(direction) = Dir3::new(direction_vec) {
                let filter = SpatialQueryFilter::from_excluded_entities([entity]);
                if let Some(hit) = spatial_query.cast_ray(origin, direction, (target_eye - origin).length(), true, &filter) {
                    if hit.entity != target_entity { continue; }
                }
            }

            if dist < min_dist {
                min_dist = dist;
                closest_target = Some(target_entity);
            }
        }

        if let Some(target) = closest_target {
            ai.target = Some(target);
            ai.suspicion_timer = ai.max_suspicion_time;
            if min_dist <= ai.attack_range {
                ai.state = AiBehaviorState::Attack;
            } else {
                ai.state = AiBehaviorState::Chase;
            }
        } else if ai.state == AiBehaviorState::Chase || ai.state == AiBehaviorState::Attack {
            // Target lost, transition to Suspect if we have a target previously
            if ai.target.is_some() {
                ai.state = AiBehaviorState::Suspect;
            } else {
                ai.state = AiBehaviorState::Idle;
            }
        }
    }
}

pub fn update_ai_hearing(
    mut queue: ResMut<NoiseEventQueue>,
    mut ai_query: Query<(
        &GlobalTransform,
        &mut AiController,
        &AIPerceptionSettings,
        Option<&AiHearingSettings>,
    )>,
) {
    for event in queue.0.iter() {
        for (transform, mut ai, settings, hearing) in ai_query.iter_mut() {
            if ai.is_paused { continue; }
            if let Some(hearing) = hearing {
                if !hearing.enabled {
                    continue;
                }
                if event.volume < hearing.min_decibels {
                    continue;
                }
                if hearing.investigate_only_if_idle
                    && ai.state != AiBehaviorState::Idle
                    && ai.state != AiBehaviorState::Suspect
                {
                    continue;
                }
            }
            let dist = transform.translation().distance(event.position);
            if dist <= settings.hearing_range * event.volume {
                // Investigate noise
                if ai.target.is_none() {
                    ai.state = AiBehaviorState::Suspect;
                    ai.suspicion_timer = ai.max_suspicion_time;
                    ai.target_last_position = Some(event.position);
                }
            }
        }
    }
    queue.0.clear();
}

pub fn draw_ai_vision_cones(
    mut gizmos: Gizmos,
    query: Query<(&GlobalTransform, &AiController, &AiPerception, &AiVisionVisualizer, &AIPerceptionSettings)>,
) {
    for (transform, ai, _perception, visualizer, settings) in query.iter() {
        if !visualizer.active { continue; }
        let color = if ai.state == AiBehaviorState::Attack || ai.state == AiBehaviorState::Chase {
            visualizer.alert_color
        } else {
            visualizer.normal_color
        };
        let pos = transform.translation() + Vec3::Y * 1.5;
        let forward = transform.forward();
        let half_fov = settings.fov.to_radians() / 2.0;
        let range = settings.range;

        let left_dir = Quat::from_rotation_y(half_fov) * forward;
        let right_dir = Quat::from_rotation_y(-half_fov) * forward;
        gizmos.ray(pos, left_dir * range, color);
        gizmos.ray(pos, right_dir * range, color);
    }
}
