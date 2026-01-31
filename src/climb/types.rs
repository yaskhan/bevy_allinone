use bevy::prelude::*;

/// Force mode for applying forces to rigidbodies
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub enum ForceMode {
    #[default]
    Impulse,
    VelocityChange,
    Force,
}

/// Ledge zone component for configuring climbable surfaces
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LedgeZone {
    pub tag_to_check: String,
    pub ledge_zone_active: bool,
    pub check_down_raycast_offset: Vec3,
    pub climb_ledge_forward_ray_distance: f32,
    pub climb_ledge_down_ray_distance: f32,
    pub ledge_zone_can_be_climbed: bool,
    pub avoid_player_grab_ledge: bool,
    pub can_check_for_hang_from_ledge_on_ground: bool,
    pub only_hang_from_ledge_if_player_is_not_moving: bool,
    pub can_grab_any_surface_on_air_active: bool,
}

impl Default for LedgeZone {
    fn default() -> Self {
        Self {
            tag_to_check: "Player".to_string(),
            ledge_zone_active: true,
            check_down_raycast_offset: Vec3::ZERO,
            climb_ledge_forward_ray_distance: 1.0,
            climb_ledge_down_ray_distance: 1.0,
            ledge_zone_can_be_climbed: true,
            avoid_player_grab_ledge: false,
            can_check_for_hang_from_ledge_on_ground: true,
            only_hang_from_ledge_if_player_is_not_moving: true,
            can_grab_any_surface_on_air_active: true,
        }
    }
}

/// Climb state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum ClimbState {
    None,
    Approaching,
    Hanging,
    ClimbingUp,
    ClimbingDown,
    ClimbingLeft,
    ClimbingRight,
    Vaulting,
    Falling,
}

impl Default for ClimbState {
    fn default() -> Self {
        Self::None
    }
}

/// Component to track current climb state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ClimbStateTracker {
    pub current_state: ClimbState,
    pub previous_state: ClimbState,
    pub state_timer: f32,
    pub climb_speed: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub stamina_drain_rate: f32,
    pub stamina_regen_rate: f32,
    pub is_stamina_depleted: bool,
}

impl Default for ClimbStateTracker {
    fn default() -> Self {
        Self {
            current_state: ClimbState::None,
            previous_state: ClimbState::None,
            state_timer: 0.0,
            climb_speed: 3.0,
            stamina: 100.0,
            max_stamina: 100.0,
            stamina_drain_rate: 10.0,
            stamina_regen_rate: 5.0,
            is_stamina_depleted: false,
        }
    }
}

/// Component for ledge detection results
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LedgeDetection {
    pub ledge_found: bool,
    pub ledge_position: Vec3,
    pub ledge_normal: Vec3,
    pub ledge_distance: f32,
    pub ledge_height: f32,
    pub is_hangable: bool,
    pub is_climbable: bool,
    pub surface_type: SurfaceType,
    pub raycast_hit_point: Vec3,
    pub raycast_hit_normal: Vec3,
}

impl Default for LedgeDetection {
    fn default() -> Self {
        Self {
            ledge_found: false,
            ledge_position: Vec3::ZERO,
            ledge_normal: Vec3::ZERO,
            ledge_distance: 0.0,
            ledge_height: 0.0,
            is_hangable: false,
            is_climbable: false,
            surface_type: SurfaceType::Default,
            raycast_hit_point: Vec3::ZERO,
            raycast_hit_normal: Vec3::ZERO,
        }
    }
}

/// Surface type for different climb speeds
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub enum SurfaceType {
    #[default]
    Default,
    Stone,
    Wood,
    Metal,
    Ice,
    Rope,
    Custom(f32), // Custom climb speed multiplier
}

impl SurfaceType {
    pub fn climb_speed_multiplier(&self) -> f32 {
        match self {
            SurfaceType::Default => 1.0,
            SurfaceType::Stone => 1.0,
            SurfaceType::Wood => 0.9,
            SurfaceType::Metal => 0.8,
            SurfaceType::Ice => 1.2,
            SurfaceType::Rope => 0.7,
            SurfaceType::Custom(multiplier) => *multiplier,
        }
    }
}

/// Component for auto-hang functionality
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AutoHang {
    pub active: bool,
    pub moving_toward_ledge: bool,
    pub target_ledge_position: Vec3,
    pub target_ledge_normal: Vec3,
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub timeout: f32,
    pub timer: f32,
    pub only_when_not_moving: bool,
    pub look_at_ledge_on_first_person: bool,
    pub max_distance_to_camera_center: f32,
}

impl Default for AutoHang {
    fn default() -> Self {
        Self {
            active: false,
            moving_toward_ledge: false,
            target_ledge_position: Vec3::ZERO,
            target_ledge_normal: Vec3::ZERO,
            move_speed: 3.0,
            rotation_speed: 10.0,
            timeout: 3.0,
            timer: 0.0,
            only_when_not_moving: true,
            look_at_ledge_on_first_person: false,
            max_distance_to_camera_center: 100.0,
        }
    }
}

/// Component for climb animation control
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ClimbAnimation {
    pub is_active: bool,
    pub action_id: i32,
    pub action_name: String,
    pub match_start_value: f32,
    pub match_end_value: f32,
    pub match_mask_value: Vec3,
    pub match_mask_rotation_value: f32,
    pub base_layer_index: i32,
    pub is_first_person: bool,
    pub time_to_climb_third_person: f32,
    pub time_to_climb_first_person: f32,
}

impl Default for ClimbAnimation {
    fn default() -> Self {
        Self {
            is_active: false,
            action_id: 1,
            action_name: "Hold On Ledge".to_string(),
            match_start_value: 0.0,
            match_end_value: 1.0,
            match_mask_value: Vec3::ONE,
            match_mask_rotation_value: 1.0,
            base_layer_index: 0,
            is_first_person: false,
            time_to_climb_third_person: 2.0,
            time_to_climb_first_person: 1.0,
        }
    }
}

/// Component for climb movement control
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ClimbMovement {
    pub is_active: bool,
    pub target_position: Vec3,
    pub target_rotation: Quat,
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub hand_offset: f32,
    pub is_first_person: bool,
    pub climb_speed_first_person: f32,
    pub adjust_position_speed: f32,
    pub adjust_rotation_speed: f32,
}

impl Default for ClimbMovement {
    fn default() -> Self {
        Self {
            is_active: false,
            target_position: Vec3::ZERO,
            target_rotation: Quat::IDENTITY,
            move_speed: 3.0,
            rotation_speed: 10.0,
            hand_offset: 0.2,
            is_first_person: false,
            climb_speed_first_person: 1.0,
            adjust_position_speed: 3.0,
            adjust_rotation_speed: 10.0,
        }
    }
}

/// Component for jump from ledge
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LedgeJump {
    pub can_jump: bool,
    pub jump_force: f32,
    pub jump_force_mode: ForceMode,
    pub is_jumping: bool,
    pub jump_timer: f32,
}

impl Default for LedgeJump {
    fn default() -> Self {
        Self {
            can_jump: false,
            jump_force: 10.0,
            jump_force_mode: ForceMode::Impulse,
            is_jumping: false,
            jump_timer: 0.0,
        }
    }
}

/// Component for grab surface on air
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GrabSurfaceOnAir {
    pub can_grab: bool,
    pub use_amount_limit: bool,
    pub amount_limit: i32,
    pub current_amount: i32,
    pub is_grabbing: bool,
    pub grab_timer: f32,
}

impl Default for GrabSurfaceOnAir {
    fn default() -> Self {
        Self {
            can_grab: false,
            use_amount_limit: false,
            amount_limit: 3,
            current_amount: 0,
            is_grabbing: false,
            grab_timer: 0.0,
        }
    }
}

/// Event for when a ledge is grabbed
#[derive(Event, Debug, Reflect)]
pub struct LedgeGrabbedEvent {
    pub entity: Entity,
    pub ledge_position: Vec3,
    pub ledge_normal: Vec3,
    pub surface_type: SurfaceType,
}

/// Event for when a ledge is climbed
#[derive(Event, Debug, Reflect)]
pub struct LedgeClimbedEvent {
    pub entity: Entity,
    pub climb_time: f32,
    pub surface_type: SurfaceType,
}

/// Event for when a ledge is lost
#[derive(Event, Debug, Reflect)]
pub struct LedgeLostEvent {
    pub entity: Entity,
    pub reason: LedgeLostReason,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum LedgeLostReason {
    PlayerMoved,
    StaminaDepleted,
    SurfaceBecameUnclimbable,
    ManualRelease,
    JumpedOff,
}

/// Event for when a ledge jump occurs
#[derive(Event, Debug, Reflect)]
pub struct LedgeJumpEvent {
    pub entity: Entity,
    pub jump_force: f32,
    pub jump_direction: Vec3,
}
