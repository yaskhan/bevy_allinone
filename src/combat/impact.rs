use bevy::prelude::*;
use std::collections::HashMap;

use crate::combat::result_queue::DamageResultQueue;
use crate::combat::types::DamageType;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SurfaceType {
    pub name: String,
}

impl Default for SurfaceType {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct SurfaceFxDefinition {
    pub particles: String,
    pub sound: String,
    pub decal: String,
    pub color: Color,
}

impl Default for SurfaceFxDefinition {
    fn default() -> Self {
        Self {
            particles: String::new(),
            sound: String::new(),
            decal: String::new(),
            color: Color::srgb(0.9, 0.9, 0.9),
        }
    }
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct SurfaceFxDatabase {
    pub map: HashMap<String, SurfaceFxDefinition>,
}

impl Default for SurfaceFxDatabase {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("Default".to_string(), SurfaceFxDefinition::default());
        map.insert(
            "Metal".to_string(),
            SurfaceFxDefinition {
                color: Color::srgb(0.7, 0.7, 0.8),
                ..default()
            },
        );
        map.insert(
            "Stone".to_string(),
            SurfaceFxDefinition {
                color: Color::srgb(0.6, 0.6, 0.6),
                ..default()
            },
        );
        map.insert(
            "Wood".to_string(),
            SurfaceFxDefinition {
                color: Color::srgb(0.6, 0.45, 0.3),
                ..default()
            },
        );
        Self { map }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SurfaceFxMarker {
    pub lifetime: f32,
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct SurfaceFxSettings {
    pub enable_melee_fx: bool,
    pub marker_lifetime: f32,
    pub marker_radius: f32,
}

impl Default for SurfaceFxSettings {
    fn default() -> Self {
        Self {
            enable_melee_fx: true,
            marker_lifetime: 1.2,
            marker_radius: 0.08,
        }
    }
}

pub fn spawn_surface_fx_from_damage(
    mut commands: Commands,
    damage_queue: Res<DamageResultQueue>,
    fx_db: Res<SurfaceFxDatabase>,
    settings: Res<SurfaceFxSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    surface_query: Query<(&GlobalTransform, Option<&SurfaceType>)>,
) {
    if !settings.enable_melee_fx {
        return;
    }

    for event in damage_queue.0.iter() {
        if event.damage_type != DamageType::Melee || event.final_amount <= 0.0 {
            continue;
        }

        let Ok((transform, surface_opt)) = surface_query.get(event.target) else {
            continue;
        };

        let surface_name = surface_opt
            .map(|surface| surface.name.as_str())
            .unwrap_or("Default");

        let fx = fx_db
            .map
            .get(surface_name)
            .cloned()
            .unwrap_or_default();

        let mesh = meshes.add(Mesh::from(shape::Icosphere {
            radius: settings.marker_radius,
            subdivisions: 2,
        }));
        let material = materials.add(StandardMaterial {
            base_color: fx.color,
            unlit: true,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(transform.translation()),
                ..default()
            },
            SurfaceFxMarker {
                lifetime: settings.marker_lifetime,
            },
            Name::new("SurfaceFxMarker"),
        ));
    }
}

pub fn update_surface_fx_markers(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SurfaceFxMarker)>,
) {
    let dt = time.delta_secs();
    for (entity, mut marker) in query.iter_mut() {
        marker.lifetime -= dt;
        if marker.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
