use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::InputState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_camera_rotation,
                update_camera_follow,
                update_camera_zoom,
                handle_camera_collision,
                update_camera_fov,
            ).chain());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum CameraMode {
    #[default]
    ThirdPerson,
    FirstPerson,
    Locked,
    SideScroller,
}

/// Camera controller component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GameCamera {
    pub follow_target: Option<Entity>,
    pub mode: CameraMode,
    
    // Rotation
    pub rotation_speed: f32,
    pub min_vertical_angle: f32,
    pub max_vertical_angle: f32,
    
    // Zoom/Distance
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub zoom_speed: f32,
    
    // Smoothing
    pub smooth_follow_speed: f32,
    pub smooth_rotation_speed: f32,
    
    // Offsets
    pub pivot_offset: Vec3,
    pub camera_offset: Vec3,
    
    // FOV
    pub default_fov: f32,
    pub target_fov: f32,
    pub fov_speed: f32,
    
    // Collision
    pub use_collision: bool,
    pub collision_radius: f32,
}

impl Default for GameCamera {
    fn default() -> Self {
        Self {
            follow_target: None,
            mode: CameraMode::ThirdPerson,
            
            rotation_speed: 0.1,
            min_vertical_angle: -80.0,
            max_vertical_angle: 80.0,
            
            distance: 4.0,
            min_distance: 1.0,
            max_distance: 10.0,
            zoom_speed: 1.0,
            
            smooth_follow_speed: 10.0,
            smooth_rotation_speed: 15.0,
            
            pivot_offset: Vec3::new(0.0, 1.5, 0.0),
            camera_offset: Vec3::ZERO,
            
            default_fov: 60.0,
            target_fov: 60.0,
            fov_speed: 5.0,
            
            use_collision: true,
            collision_radius: 0.2,
        }
    }
}

/// Camera state tracking current angles
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraState {
    pub yaw: f32,
    pub pitch: f32,
    pub current_distance: f32,
    pub smoothed_pivot: Vec3,
}

// ============================================================================
// SYSTEMS
// ============================================================================

fn update_camera_rotation(
    input: Res<InputState>,
    mut query: Query<(&GameCamera, &mut CameraState)>,
) {
    for (camera, mut state) in query.iter_mut() {
        if camera.mode == CameraMode::Locked {
            continue;
        }

        state.yaw -= input.look.x * camera.rotation_speed;
        state.pitch -= input.look.y * camera.rotation_speed;
        state.pitch = state.pitch.clamp(camera.min_vertical_angle, camera.max_vertical_angle);
    }
}

fn update_camera_follow(
    time: Res<Time>,
    mut camera_query: Query<(&GameCamera, &mut CameraState, &mut Transform)>,
    target_query: Query<&Transform, Without<GameCamera>>,
) {
    for (camera, mut state, mut transform) in camera_query.iter_mut() {
        let Some(target_entity) = camera.follow_target else { continue };
        let Ok(target_transform) = target_query.get(target_entity) else { continue };

        let pivot_pos = target_transform.translation + camera.pivot_offset;
        
        // Smooth pivot following
        state.smoothed_pivot = state.smoothed_pivot.lerp(pivot_pos, camera.smooth_follow_speed * time.delta_secs());

        // Calculate rotation based on yaw/pitch
        let rotation = Quat::from_rotation_y(state.yaw.to_radians()) * Quat::from_rotation_x(state.pitch.to_radians());
        
        // Smooth rotation update
        transform.rotation = transform.rotation.slerp(rotation, camera.smooth_rotation_speed * time.delta_secs());

        // Initial position calculation (before collision)
        let direction = transform.back();
        transform.translation = state.smoothed_pivot + direction * state.current_distance;
    }
}

fn update_camera_zoom(
    mut query: Query<(&GameCamera, &mut CameraState)>,
) {
    for (camera, mut state) in query.iter_mut() {
        // Zoom logic can be added here if we have scroll input in InputState
        state.current_distance = camera.distance; 
    }
}

fn handle_camera_collision(
    spatial_query: SpatialQuery,
    mut query: Query<(&GameCamera, &CameraState, &mut Transform)>,
) {
    for (camera, state, mut transform) in query.iter_mut() {
        if !camera.use_collision { continue; }

        let start = state.smoothed_pivot;
        let direction = transform.back();
        let max_dist = camera.distance;

        let filter = SpatialQueryFilter::default(); // Exclude player if needed

        if let Some(hit) = spatial_query.cast_ray(
            start,
            direction,
            max_dist,
            true,
            filter,
        ) {
            transform.translation = start + direction * (hit.distance - camera.collision_radius);
        }
    }
}

fn update_camera_fov(
    time: Res<Time>,
    mut query: Query<(&GameCamera, &mut Projection)>,
) {
    for (camera, mut projection) in query.iter_mut() {
        if let Projection::Perspective(ref mut p) = *projection {
            let target_rad = camera.target_fov.to_radians();
            p.fov = p.fov + (target_rad - p.fov) * camera.fov_speed * time.delta_secs();
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn spawn_camera(
    commands: &mut Commands,
    target: Entity,
) -> Entity {
    commands.spawn((
        Camera3d::default(),
        GameCamera {
            follow_target: Some(target),
            ..default()
        },
        CameraState {
            current_distance: 4.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ))
    .id()
}
