use bevy::prelude::*;
use avian3d::prelude::*;

use crate::abilities::LaserVisionSliceEventQueue;
use crate::combat::result_queue::DamageResultQueue;
use crate::combat::types::DamageType;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct SliceFxSettings {
    pub spawn_debug_marker: bool,
    pub marker_lifetime: f32,
    pub marker_radius: f32,
    pub marker_color: Color,
}

impl Default for SliceFxSettings {
    fn default() -> Self {
        Self {
            spawn_debug_marker: true,
            marker_lifetime: 1.5,
            marker_radius: 0.12,
            marker_color: Color::srgb(1.0, 0.2, 0.2),
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SliceFxMarker {
    pub lifetime: f32,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SliceChunk {
    pub lifetime: f32,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Sliceable {
    pub enabled: bool,
    pub slice_radius: f32,
    pub min_delay_between_slices: f32,
    pub last_slice_time: f32,
    pub spawn_simple_chunks: bool,
    pub chunk_size: Vec3,
    pub chunk_lifetime: f32,
    pub chunk_impulse: f32,
    pub despawn_original: bool,
}

impl Default for Sliceable {
    fn default() -> Self {
        Self {
            enabled: true,
            slice_radius: 1.0,
            min_delay_between_slices: 0.5,
            last_slice_time: -999.0,
            spawn_simple_chunks: true,
            chunk_size: Vec3::splat(0.4),
            chunk_lifetime: 4.0,
            chunk_impulse: 3.0,
            despawn_original: true,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SliceOnDamage {
    pub enabled: bool,
    pub min_damage: f32,
    pub allow_blocked: bool,
    pub radius_override: Option<f32>,
    pub require_damage_type: Option<DamageType>,
}

impl Default for SliceOnDamage {
    fn default() -> Self {
        Self {
            enabled: true,
            min_damage: 5.0,
            allow_blocked: false,
            radius_override: None,
            require_damage_type: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SliceEvent {
    pub source: Option<Entity>,
    pub position: Vec3,
    pub normal: Vec3,
    pub direction: Vec3,
    pub radius: f32,
}

#[derive(Resource, Default)]
pub struct SliceEventQueue(pub Vec<SliceEvent>);

#[derive(Debug, Clone)]
pub struct SliceResultEvent {
    pub target: Entity,
    pub position: Vec3,
    pub normal: Vec3,
    pub source: Option<Entity>,
    pub sliced: bool,
}

#[derive(Resource, Default)]
pub struct SliceResultQueue(pub Vec<SliceResultEvent>);

pub fn queue_slice_events_from_laser(
    mut slice_queue: ResMut<SliceEventQueue>,
    mut laser_queue: Option<ResMut<LaserVisionSliceEventQueue>>,
) {
    let Some(mut laser_queue) = laser_queue else { return };

    for event in laser_queue.0.drain(..) {
        slice_queue.0.push(SliceEvent {
            source: Some(event.entity),
            position: event.position,
            normal: event.direction,
            direction: event.direction,
            radius: 0.8,
        });
    }
}

pub fn queue_slice_events_from_damage_results(
    damage_queue: Res<DamageResultQueue>,
    mut slice_queue: ResMut<SliceEventQueue>,
    query: Query<(&GlobalTransform, &SliceOnDamage, &Sliceable)>,
) {
    for event in damage_queue.0.iter() {
        let Ok((transform, settings, sliceable)) = query.get(event.target) else {
            continue;
        };

        if !settings.enabled || !sliceable.enabled {
            continue;
        }

        if event.final_amount < settings.min_damage {
            continue;
        }

        if event.is_block && !settings.allow_blocked {
            continue;
        }

        if let Some(required_type) = settings.require_damage_type {
            if event.damage_type != required_type {
                continue;
            }
        }

        let radius = settings.radius_override.unwrap_or(sliceable.slice_radius);

        slice_queue.0.push(SliceEvent {
            source: event.source,
            position: transform.translation(),
            normal: Vec3::Y,
            direction: Vec3::Y,
            radius,
        });
    }
}

pub fn apply_slice_events(
    time: Res<Time>,
    mut slice_queue: ResMut<SliceEventQueue>,
    mut result_queue: ResMut<SliceResultQueue>,
    mut sliceables: Query<(Entity, &GlobalTransform, &mut Sliceable)>,
) {
    let now = time.elapsed_secs();

    for event in slice_queue.0.drain(..) {
        for (entity, transform, mut sliceable) in sliceables.iter_mut() {
            if !sliceable.enabled {
                continue;
            }

            if now - sliceable.last_slice_time < sliceable.min_delay_between_slices {
                continue;
            }

            let distance = transform.translation().distance(event.position);
            if distance > sliceable.slice_radius.max(event.radius) {
                continue;
            }

            sliceable.last_slice_time = now;

            result_queue.0.push(SliceResultEvent {
                target: entity,
                position: event.position,
                normal: event.normal,
                source: event.source,
                sliced: true,
            });

            // TODO: Integrate actual mesh slicing once a Bevy-compatible slicer is chosen.
        }
    }
}

pub fn handle_slice_results(
    mut commands: Commands,
    time: Res<Time>,
    mut results: ResMut<SliceResultQueue>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<SliceFxSettings>,
    target_query: Query<(&GlobalTransform, &Sliceable, Option<&Handle<StandardMaterial>>)>,
) {
    if !settings.spawn_debug_marker {
        results.0.clear();
        return;
    }

    for result in results.0.drain(..) {
        if !result.sliced {
            continue;
        }

        let mesh = meshes.add(Mesh::from(Sphere::new(settings.marker_radius)));
        let material = materials.add(StandardMaterial {
            base_color: settings.marker_color,
            unlit: true,
            ..default()
        });

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(result.position),
            GlobalTransform::default(),
            Visibility::default(),
            SliceFxMarker {
                lifetime: settings.marker_lifetime,
            },
            Name::new("SliceFxMarker"),
        ));

        let Ok((target_transform, sliceable, target_material)) = target_query.get(result.target) else {
            continue;
        };

        if sliceable.spawn_simple_chunks {
            let chunk_mesh = meshes.add(Mesh::from(Cuboid::new(
                sliceable.chunk_size.x,
                sliceable.chunk_size.y,
                sliceable.chunk_size.z,
            )));

            let material_handle = target_material
                .cloned()
                .unwrap_or_else(|| materials.add(StandardMaterial {
                    base_color: Color::srgb(0.6, 0.6, 0.6),
                    ..default()
                }));

            let normal = result.normal.normalize_or_zero();
            let offset = normal * (sliceable.chunk_size.length() * 0.25);

            for dir in [-1.0f32, 1.0f32] {
                let position = target_transform.translation() + offset * dir;
                let impulse = normal * sliceable.chunk_impulse * dir;

                commands.spawn((
                    Mesh3d(chunk_mesh.clone()),
                    MeshMaterial3d(material_handle.clone()),
                    Transform::from_translation(position),
                    GlobalTransform::default(),
                    Visibility::default(),
                    RigidBody::Dynamic,
                    Collider::cuboid(
                        sliceable.chunk_size.x * 0.5,
                        sliceable.chunk_size.y * 0.5,
                        sliceable.chunk_size.z * 0.5,
                    ),
                    LinearVelocity(impulse),
                    SliceChunk {
                        lifetime: sliceable.chunk_lifetime,
                    },
                    Name::new("SliceChunk"),
                ));
            }

            if sliceable.despawn_original {
                commands.entity(result.target).despawn_recursive();
            }
        }
    }
}

pub fn update_slice_fx_markers(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SliceFxMarker)>,
) {
    let dt = time.delta_secs();
    for (entity, mut marker) in query.iter_mut() {
        marker.lifetime -= dt;
        if marker.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_slice_chunks(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SliceChunk)>,
) {
    let dt = time.delta_secs();
    for (entity, mut chunk) in query.iter_mut() {
        chunk.lifetime -= dt;
        if chunk.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
