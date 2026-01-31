use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use avian3d::prelude::*;
use super::types::*;

/// System to handle mouse clicks for navigation
pub fn handle_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    spatial_query: SpatialQuery,
    mut controller_query: Query<&mut PointAndClickController>,
    elements_query: Query<(&GlobalTransform, &PointAndClickElement)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window_query.iter().next().ok_or(()) else { return };
    let Some(cursor_position) = window.cursor_position() else { return };

    let Ok((camera, camera_transform)) = camera_query.iter().next().ok_or(()) else { return };

    // Create a ray from the camera into the world
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return };

    // Cast ray using Avian3d
    let max_distance = 1000.0;
    let filter = SpatialQueryFilter::default();

    if let Some(hit) = spatial_query.cast_ray(
        ray.origin,
        ray.direction,
        max_distance,
        true,
        &filter
    ) {
        let hit_point = ray.origin + ray.direction * hit.distance;
        
        let mut target_pos = hit_point;
        
        if let Ok((element_xf, element)) = elements_query.get(hit.entity) {
            if element.enabled {
                let offset = element.interaction_offset;
                let world_offset = element_xf.rotation() * offset;
                target_pos = element_xf.translation() + world_offset;
                info!("Clicked Element: {:?}, moving to {:?}", element.element_type, target_pos);
            }
        } else {
             info!("Clicked Ground at: {:?}", target_pos);
        }

        for mut controller in controller_query.iter_mut() {
            if controller.enabled {
                controller.current_target = Some(target_pos);
            }
        }
    }
}

/// System to move the agent towards the target
pub fn move_agent(
    time: Res<Time>,
    mut agent_query: Query<(&mut Transform, &mut PointAndClickController)>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut controller) in agent_query.iter_mut() {
        let Some(target) = controller.current_target else { continue };

        let current_pos = transform.translation;
        // Ignore Y for distance check (2D plane movement logic usually)
        
        let delta = target - current_pos;
        let distance = delta.length();
        
        if distance < controller.stopping_distance {
            // Reached target
            controller.current_target = None;
            continue;
        }

        let direction = delta.normalize_or_zero();
        
        // Move
        transform.translation += direction * controller.move_speed * dt;
        
        // Rotate to face target
        if direction.length_squared() > 0.001 {
             let target_rot = Quat::from_rotation_arc(Vec3::Z, direction); // Assuming Z-forward model?
             // Actually, usually models are -Z or +Z forward. 
             // Let's assume standard Bevy forward (-Z).
             // Quat::from_rotation_arc(Vec3::NEG_Z, direction)
             
             // Or simpler: look_at. But look_at modifies translation too if not careful with Up.
             // Let's use rotate_towards logic if available, or just look_at for now.
             
             let flat_target = Vec3::new(target.x, current_pos.y, target.z);
             transform.look_at(flat_target, Vec3::Y);
        }
    }
}
