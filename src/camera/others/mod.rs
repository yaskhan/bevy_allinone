use bevy::prelude::*;
use avian3d::prelude::*;
use crate::character::Player;
use crate::camera::types::*;

pub struct CameraOthersPlugin;

impl Plugin for CameraOthersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TransparencySettings>()
           .init_resource::<PlayerCullingSettings>()
           .register_type::<TransparentSurface>()
           .add_systems(Update, (
               update_transparent_surfaces,
               apply_surface_transparency,
               update_player_culling,
           ).chain());
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct TransparencySettings {
    pub enabled: bool,
    pub alpha_target: f32,
    pub fade_speed: f32,
    pub ray_radius: f32,
}

impl Default for TransparencySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            alpha_target: 0.2,
            fade_speed: 10.0,
            ray_radius: 0.2,
        }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerCullingSettings {
    pub enabled: bool,
    pub min_dist: f32,
    pub fade_speed: f32,
    pub min_alpha: f32,
}

impl Default for PlayerCullingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            min_dist: 1.0,
            fade_speed: 8.0,
            min_alpha: 0.0,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TransparentSurface {
    pub target_alpha: f32,
    pub current_alpha: f32,
    pub active_this_frame: bool,
}

impl Default for TransparentSurface {
    fn default() -> Self {
        Self {
            target_alpha: 1.0,
            current_alpha: 1.0,
            active_this_frame: false,
        }
    }
}

pub fn update_transparent_surfaces(
    settings: Res<TransparencySettings>,
    spatial_query: SpatialQuery,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    camera_query: Query<(Entity, &GlobalTransform), With<CameraController>>,
    mut surface_query: Query<&mut TransparentSurface>,
    mut commands: Commands,
) {
    if !settings.enabled { return; }

    let (player_ent, player_xf) = match player_query.iter().next() {
        Some(p) => p,
        None => return,
    };
    let (camera_ent, camera_xf) = match camera_query.iter().next() {
        Some(c) => c,
        None => return,
    };

    let camera_pos = camera_xf.translation();
    let player_pos = player_xf.translation() + Vec3::Y * 1.2;
    let dir_vec = player_pos - camera_pos;
    let dist = dir_vec.length();
    
    if dist < 0.1 { return; }
    
    let dir_norm = Dir3::new(dir_vec).unwrap_or(Dir3::NEG_Z);

    for mut surface in surface_query.iter_mut() {
        surface.active_this_frame = false;
        surface.target_alpha = 1.0;
    }

    let filter = SpatialQueryFilter::from_excluded_entities([camera_ent, player_ent]);
    
    let hits = spatial_query.ray_hits(
        camera_pos,
        dir_norm,
        dist,
        20,
        true,
        &filter
    );

    for hit in hits {
        let entity = hit.entity;
        if let Ok(mut surface) = surface_query.get_mut(entity) {
            surface.active_this_frame = true;
            surface.target_alpha = settings.alpha_target;
        } else {
            commands.entity(entity).insert(TransparentSurface {
                target_alpha: settings.alpha_target,
                current_alpha: 1.0,
                active_this_frame: true,
            });
        }
    }
}

pub fn apply_surface_transparency(
    time: Res<Time>,
    settings: Res<TransparencySettings>,
    mut surface_query: Query<(&mut TransparentSurface, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_secs();
    let alpha_decay = 1.0 - (-settings.fade_speed * dt).exp();

    for (mut surface, mat_handle) in surface_query.iter_mut() {
        surface.current_alpha = surface.current_alpha + (surface.target_alpha - surface.current_alpha) * alpha_decay;

        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.base_color.set_alpha(surface.current_alpha);
            if surface.current_alpha < 0.99 {
                mat.alpha_mode = AlphaMode::Blend;
            } else {
                mat.alpha_mode = AlphaMode::Opaque;
            }
        }
    }
}

pub fn update_player_culling(
    time: Res<Time>,
    settings: Res<PlayerCullingSettings>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    camera_query: Query<&GlobalTransform, With<CameraController>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_queries: Query<&MeshMaterial3d<StandardMaterial>>,
    children: Query<&Children>,
) {
    if !settings.enabled { return; }

    let (_, player_xf) = match player_query.iter().next() {
        Some(p) => p,
        None => return,
    };
    let camera_xf = match camera_query.iter().next() {
        Some(c) => c,
        None => return,
    };

    let dist = player_xf.translation().distance(camera_xf.translation());
    let target_alpha = if dist < settings.min_dist {
        (dist / settings.min_dist).max(settings.min_alpha)
    } else {
        1.0
    };

    let dt = time.delta_secs();
    let alpha_decay = 1.0 - (-settings.fade_speed * dt).exp();

    // Recursively apply to player model materials
    for (player_ent, _) in player_query.iter() {
        apply_culling_recursive(player_ent, target_alpha, alpha_decay, &mut materials, &mesh_queries, &children);
    }
}

fn apply_culling_recursive(
    entity: Entity,
    target_alpha: f32,
    alpha_decay: f32,
    materials: &mut Assets<StandardMaterial>,
    mesh_queries: &Query<&MeshMaterial3d<StandardMaterial>>,
    children_query: &Query<&Children>,
) {
    if let Ok(mat_handle) = mesh_queries.get(entity) {
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            let current = mat.base_color.alpha();
            let new_alpha = current + (target_alpha - current) * alpha_decay;
            mat.base_color.set_alpha(new_alpha);
            if new_alpha < 0.99 {
                mat.alpha_mode = AlphaMode::Blend;
            } else {
                mat.alpha_mode = AlphaMode::Opaque;
            }
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for i in 0..children.len() {
            let child = children[i];
            apply_culling_recursive(child, target_alpha, alpha_decay, materials, mesh_queries, children_query);
        }
    }
}
