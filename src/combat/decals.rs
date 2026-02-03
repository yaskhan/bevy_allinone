use bevy::prelude::*;
use std::collections::HashMap;

use crate::combat::types::DamageEventQueue;
use crate::combat::types::DamageType;
use crate::combat::impact::SurfaceType;

#[derive(Debug, Clone, Reflect)]
pub struct DecalInfo {
    pub color: Color,
    pub size: Vec2,
    pub lifetime: f32,
    pub offset: f32,
    pub texture_path: String,
}

impl Default for DecalInfo {
    fn default() -> Self {
        Self {
            color: Color::srgb(0.15, 0.15, 0.15),
            size: Vec2::splat(0.2),
            lifetime: 12.0,
            offset: 0.01,
            texture_path: String::new(),
        }
    }
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct DecalRegistry {
    pub map: HashMap<String, DecalInfo>,
    pub fallback: DecalInfo,
}

impl Default for DecalRegistry {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            "Metal".to_string(),
            DecalInfo {
                color: Color::srgb(0.1, 0.1, 0.1),
                ..default()
            },
        );
        map.insert(
            "Stone".to_string(),
            DecalInfo {
                color: Color::srgb(0.2, 0.2, 0.2),
                ..default()
            },
        );
        map.insert(
            "Wood".to_string(),
            DecalInfo {
                color: Color::srgb(0.25, 0.18, 0.12),
                ..default()
            },
        );
        map.insert(
            "Flesh".to_string(),
            DecalInfo {
                color: Color::srgb(0.6, 0.1, 0.1),
                ..default()
            },
        );
        Self {
            map,
            fallback: DecalInfo::default(),
        }
    }
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct DecalSettings {
    pub enabled: bool,
    pub attach_to_target: bool,
    pub max_per_frame: usize,
    pub include_damage_types: Vec<DamageType>,
}

impl Default for DecalSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            attach_to_target: true,
            max_per_frame: 16,
            include_damage_types: vec![DamageType::Ranged, DamageType::Explosion, DamageType::Melee],
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Decal {
    pub lifetime: f32,
}

pub fn spawn_decals_from_damage(
    mut commands: Commands,
    damage_queue: Res<DamageEventQueue>,
    registry: Res<DecalRegistry>,
    settings: Res<DecalSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    target_query: Query<(&GlobalTransform, Option<&SurfaceType>)>,
) {
    if !settings.enabled {
        return;
    }

    let mut spawned = 0;
    for event in damage_queue.0.iter() {
        if spawned >= settings.max_per_frame {
            break;
        }

        if !settings.include_damage_types.contains(&event.damage_type) {
            continue;
        }

        let Some(hit_pos) = event.position else {
            continue;
        };

        let Ok((target_transform, surface_opt)) = target_query.get(event.target) else {
            continue;
        };

        let surface_name = surface_opt
            .map(|surface| surface.name.as_str())
            .unwrap_or("Default");
        let info = registry
            .map
            .get(surface_name)
            .cloned()
            .unwrap_or_else(|| registry.fallback.clone());

        let normal = event.direction.unwrap_or(Vec3::Y).normalize_or_zero();
        let rotation = Quat::from_rotation_arc(Vec3::Z, normal);
        let world_pos = hit_pos + normal * info.offset;

        let mesh = meshes.add(Mesh::from(Rectangle::from_size(info.size)));
        let mut material = StandardMaterial {
            base_color: info.color,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        };
        if !info.texture_path.is_empty() {
            material.base_color_texture = Some(asset_server.load(info.texture_path.as_str()));
        }
        let material = materials.add(material);

        let decal_entity = commands
            .spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_translation(world_pos).with_rotation(rotation),
                    ..default()
                },
                Decal {
                    lifetime: info.lifetime,
                },
                Name::new("Decal"),
            ))
            .id();

        if settings.attach_to_target {
            let parent_rot = target_transform.rotation();
            let local_pos = target_transform
                .compute_matrix()
                .inverse()
                .transform_point3(world_pos);
            let local_rot = parent_rot.inverse() * rotation;

            commands.entity(decal_entity).set_parent(event.target);
            commands.entity(decal_entity).insert(Transform {
                translation: local_pos,
                rotation: local_rot,
                scale: Vec3::ONE,
            });
        }

        spawned += 1;
    }
}

pub fn update_decals(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Decal)>,
) {
    let dt = time.delta_secs();
    for (entity, mut decal) in query.iter_mut() {
        decal.lifetime -= dt;
        if decal.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
