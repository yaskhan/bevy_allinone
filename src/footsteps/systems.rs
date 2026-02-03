use bevy::prelude::*;
use avian3d::prelude::*;
use crate::physics::GroundDetection;
use crate::character::CharacterMovementState;
use super::types::*;
use rand::Rng;

pub fn update_footsteps(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<(
        Entity,
        &mut FootstepController,
        &GlobalTransform,
        &LinearVelocity,
        &GroundDetection,
        &CharacterMovementState,
    )>,
    surface_query: Query<&FootstepSurface>,
    mut event_queue: ResMut<FootstepEventQueue>,
) {
    let dt = time.delta_secs();

    for (entity, mut footstep, transform, velocity, ground, movement) in query.iter_mut() {
        if !footstep.is_enabled || !ground.is_grounded {
            footstep.accumulated_distance = 0.0;
            continue;
        }

        // Calculate horizontal velocity magnitude
        let horizontal_vel = Vec3::new(velocity.x, 0.0, velocity.z).length();
        
        // Only move if we are actually moving on the ground
        if horizontal_vel < 0.1 {
            footstep.accumulated_distance = 0.0;
            continue;
        }

        // Adjust step distance based on movement state (sprinting makes steps faster/more frequent)
        let effective_step_dist = if movement.is_sprinting {
            footstep.step_distance * footstep.run_step_multiplier
        } else {
            footstep.step_distance
        };

        footstep.accumulated_distance += horizontal_vel * dt;

        if footstep.accumulated_distance >= effective_step_dist {
            footstep.accumulated_distance -= effective_step_dist;
            
            // Perform raycast to find surface
            let ray_pos = transform.translation() + Vec3::Y * 0.1;
            let ray_dir = Dir3::NEG_Y;
            let filter = SpatialQueryFilter::from_excluded_entities([entity]);

            let mut surface_id = "Default".to_string(); // Fallback
            let mut hit_pos = transform.translation();
            let mut hit_normal = Vec3::Y;

            if let Some(hit) = spatial_query.cast_ray(ray_pos, ray_dir, 1.0, true, &filter) {
                hit_pos = ray_pos + ray_dir.as_vec3() * hit.distance;
                hit_normal = hit.normal;
                if let Ok(surface) = surface_query.get(hit.entity) {
                    surface_id = surface.surface_id.clone();
                }
            }

            // Determine volume
            let mut rng = rand::rng();
            let volume = rng.random_range(footstep.volume_range.0..=footstep.volume_range.1);

            // Toggle foot
            footstep.last_foot_left = !footstep.last_foot_left;

            event_queue.0.push(FootstepEvent {
                entity,
                surface_id,
                position: hit_pos,
                normal: hit_normal,
                volume,
                noise_radius: footstep.noise_radius,
                is_left: footstep.last_foot_left,
            });
        }
    }
}

pub fn handle_footstep_audio(
    mut event_queue: ResMut<FootstepEventQueue>,
    assets: Res<FootstepAssets>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<FootstepDecalSettings>,
) {
    for event in event_queue.0.drain(..) {
        let sound_pool = assets.surface_sounds.get(&event.surface_id)
            .or_else(|| assets.surface_sounds.get(&assets.default_surface_id));

        if let Some(pool) = sound_pool {
            if !pool.is_empty() {
                let mut rng = rand::rng();
                let sound_idx = rng.random_range(0..pool.len());
                let sound_handle = pool[sound_idx].clone();

                // Play spatial audio at foot position
                commands.spawn((
                    AudioPlayer::<AudioSource>(sound_handle),
                    PlaybackSettings::ONCE.with_spatial(true).with_volume(bevy::audio::Volume::Linear(event.volume)),
                    Transform::from_translation(event.position),
                    GlobalTransform::default(),
                ));
            }
        }
        
        if settings.enabled {
            let rotation = Quat::from_rotation_arc(Vec3::Z, event.normal.normalize_or_zero());
            let position = event.position + event.normal.normalize_or_zero() * settings.offset;
            let size = if event.is_left {
                settings.size
            } else {
                Vec2::new(settings.size.x, settings.size.y)
            };

            let mesh = meshes.add(Mesh::from(Rectangle::from_size(size)));
            let material = materials.add(StandardMaterial {
                base_color: settings.color,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });

            commands.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_translation(position).with_rotation(rotation),
                    ..default()
                },
                FootstepDecal {
                    lifetime: settings.lifetime,
                },
                Name::new("FootstepDecal"),
            ));
        }

        // Note: Noise signal for AI would be sent here as well
        // apply_damage::send_noise_signal(event.noise_radius, event.position, ...)
    }
}

pub fn update_footstep_decals(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FootstepDecal)>,
) {
    let dt = time.delta_secs();
    for (entity, mut decal) in query.iter_mut() {
        decal.lifetime -= dt;
        if decal.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
