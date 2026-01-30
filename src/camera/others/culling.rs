use bevy::prelude::*;
use crate::character::Player;
use crate::camera::types::{CameraController, PlayerCullingSettings};

pub struct PlayerCullingPlugin;

impl Plugin for PlayerCullingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerCullingSettings>()
           .add_systems(Update, update_player_culling);
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
