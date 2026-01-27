use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::InputState;
use crate::character::CharacterController;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_camera_state,
                update_camera_rotation,
                apply_camera_noise,
                update_camera_follow,
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
    
    // Sensitivity
    pub rot_sensitivity_3p: f32,
    pub rot_sensitivity_1p: f32,
    pub aim_zoom_sensitivity_mult: f32,
    
    // Limits
    pub min_vertical_angle: f32,
    pub max_vertical_angle: f32,
    
    // Zoom/Distance
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    
    // Smoothing
    pub smooth_follow_speed: f32,
    pub smooth_rotation_speed: f32,
    pub pivot_smooth_speed: f32,
    
    // Offsets (Dynamic)
    pub default_pivot_offset: Vec3,
    pub aim_pivot_offset: Vec3,
    pub crouch_pivot_offset: Vec3,
    
    // Leaning
    pub lean_amount: f32,
    pub lean_angle: f32,
    pub lean_speed: f32,
    
    // FOV
    pub default_fov: f32,
    pub aim_fov: f32,
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
            
            rot_sensitivity_3p: 0.15,
            rot_sensitivity_1p: 0.1,
            aim_zoom_sensitivity_mult: 0.5,
            
            min_vertical_angle: -80.0,
            max_vertical_angle: 80.0,
            
            distance: 4.0,
            min_distance: 1.0,
            max_distance: 10.0,
            
            smooth_follow_speed: 15.0,
            smooth_rotation_speed: 20.0,
            pivot_smooth_speed: 10.0,
            
            default_pivot_offset: Vec3::new(0.0, 1.6, 0.0),
            aim_pivot_offset: Vec3::new(0.5, 1.5, 0.0),
            crouch_pivot_offset: Vec3::new(0.0, 1.0, 0.0),
            
            lean_amount: 0.4,
            lean_angle: 15.0,
            lean_speed: 8.0,
            
            default_fov: 60.0,
            aim_fov: 40.0,
            fov_speed: 10.0,
            
            use_collision: true,
            collision_radius: 0.2,
        }
    }
}

/// Camera state tracking
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraState {
    pub yaw: f32,
    pub pitch: f32,
    pub current_distance: f32,
    pub current_pivot: Vec3,
    pub current_lean: f32,
    pub noise_offset: Vec2,
    pub is_aiming: bool,
    pub is_crouching: bool,
    pub lock_on_target: Option<Entity>,
}

// ============================================================================
// SYSTEMS
// ============================================================================

fn update_camera_state(
    input: Res<InputState>,
    mut query: Query<(&GameCamera, &mut CameraState)>,
    target_query: Query<(&CharacterController, &crate::character::CharacterMovementState)>,
) {
    for (camera, mut state) in query.iter_mut() {
        if let Some(target) = camera.follow_target {
            if let Ok((_controller, movement)) = target_query.get(target) {
                state.is_aiming = input.aim_pressed;
                state.is_crouching = movement.is_crouching;
            }
        }
    }
}

fn update_camera_rotation(
    input: Res<InputState>,
    time: Res<Time>,
    mut query: Query<(&GameCamera, &mut CameraState)>,
    target_query: Query<&GlobalTransform>,
) {
    for (camera, mut state) in query.iter_mut() {
        if camera.mode == CameraMode::Locked { continue; }

        // Dynamic sensitivity
        let base_sens = if camera.mode == CameraMode::FirstPerson {
            camera.rot_sensitivity_1p
        } else {
            camera.rot_sensitivity_3p
        };
        
        let sens_mult = if state.is_aiming { camera.aim_zoom_sensitivity_mult } else { 1.0 };
        let sensitivity = base_sens * sens_mult;

        // Lock-on logic
        if let Some(lock_target) = state.lock_on_target {
            if let Ok(target_gt) = target_query.get(lock_target) {
                let dir = (target_gt.translation() - state.current_pivot).normalize();
                let target_yaw = dir.x.atan2(dir.z).to_degrees();
                let target_pitch = (-dir.y).asin().to_degrees();
                
                state.yaw = state.yaw + (target_yaw - state.yaw) * 10.0 * time.delta_secs();
                state.pitch = state.pitch + (target_pitch - state.pitch) * 10.0 * time.delta_secs();
            }
        } else {
            // Manual rotation
            state.yaw -= input.look.x * sensitivity;
            state.pitch -= input.look.y * sensitivity;
        }

        state.pitch = state.pitch.clamp(camera.min_vertical_angle, camera.max_vertical_angle);

        // Leaning logic
        let target_lean = if input.lean_left { -1.0 } else if input.lean_right { 1.0 } else { 0.0 };
        state.current_lean = state.current_lean + (target_lean - state.current_lean) * camera.lean_speed * time.delta_secs();
    }
}

fn apply_camera_noise(
    time: Res<Time>,
    mut query: Query<(&GameCamera, &mut CameraState)>,
) {
    for (_camera, mut state) in query.iter_mut() {
        let t = time.elapsed_secs() * 2.0;
        // Simple procedural noise (breathing effect)
        let noise_x = (t * 0.5).sin() * 0.05;
        let noise_y = (t * 0.8).cos() * 0.05;
        
        if state.is_aiming {
            state.noise_offset = Vec2::new(noise_x * 0.3, noise_y * 0.3);
        } else {
            state.noise_offset = Vec2::new(noise_x, noise_y);
        }
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

        // Dynamic pivot calculation
        let base_pivot = if state.is_aiming {
            camera.aim_pivot_offset
        } else if state.is_crouching {
            camera.crouch_pivot_offset
        } else {
            camera.default_pivot_offset
        };

        // Apply leaning to pivot
        let lean_pivot_offset = transform.right() * state.current_lean * camera.lean_amount;
        let target_pivot_pos = target_transform.translation + base_pivot + lean_pivot_offset;
        
        state.current_pivot = state.current_pivot.lerp(target_pivot_pos, camera.pivot_smooth_speed * time.delta_secs());

        // Rotation
        let rotation = Quat::from_rotation_y((state.yaw + state.noise_offset.x).to_radians()) 
                     * Quat::from_rotation_x((state.pitch + state.noise_offset.y).to_radians());
        
        // Lean rotation
        let lean_rotation = Quat::from_rotation_z(-state.current_lean * camera.lean_angle.to_radians());
        
        transform.rotation = transform.rotation.slerp(rotation * lean_rotation, camera.smooth_rotation_speed * time.delta_secs());

        // Position
        state.current_distance = state.current_distance + (camera.distance - state.current_distance) * 5.0 * time.delta_secs();
        let direction = transform.back();
        transform.translation = state.current_pivot + direction * state.current_distance;
    }
}

fn handle_camera_collision(
    spatial_query: SpatialQuery,
    mut query: Query<(&GameCamera, &CameraState, &mut Transform)>,
) {
    for (camera, state, mut transform) in query.iter_mut() {
        if !camera.use_collision { continue; }

        let start = state.current_pivot;
        let direction = transform.back();
        let max_dist = state.current_distance;
        let filter = SpatialQueryFilter::default();

        if let Some(hit) = spatial_query.cast_ray(start, direction, max_dist, true, filter) {
            transform.translation = start + direction * (hit.distance - camera.collision_radius);
        }
    }
}

fn update_camera_fov(
    time: Res<Time>,
    mut query: Query<(&GameCamera, &CameraState, &mut Projection)>,
) {
    for (camera, state, mut projection) in query.iter_mut() {
        if let Projection::Perspective(ref mut p) = *projection {
            let target_fov = if state.is_aiming { camera.aim_fov } else { camera.default_fov };
            let target_rad = target_fov.to_radians();
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
