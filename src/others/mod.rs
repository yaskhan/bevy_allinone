use bevy::prelude::*;

pub mod add_force_to_object_system;
pub mod animator_trigger_enter_exit_event;
pub mod animator_trigger_event;
pub mod audio_source_info;
pub mod bezier_spline;
pub mod change_object_colors;
pub mod check_collision_type;
pub mod console_log_on_screen_system;
pub mod destroy_game_object;
pub mod dissolve_object;
pub mod event_object_found_on_raycast_system;
pub mod fade_object;
pub mod features_manager;
pub mod flying_turret_system;
pub mod follow_object_position_system;
pub mod follow_object_position_update_system;
pub mod gkc_utils;
pub mod head_explode_example;
pub mod hide_body_part_on_character_system;
pub mod ignore_collision_helper;
pub mod ignore_collision_system;

pub use add_force_to_object_system::AddForceToObjectSystem;
pub use animator_trigger_enter_exit_event::{
    AnimatorTriggerEnterExitEvent,
    AnimatorTriggerEnterEvent,
    AnimatorTriggerExitEvent,
};
pub use animator_trigger_event::{AnimatorTriggerEvent, AnimatorTriggerEventRequest};
pub use audio_source_info::AudioSourceInfo;
pub use bezier_spline::BezierSpline;
pub use change_object_colors::ChangeObjectColors;
pub use check_collision_type::{CheckCollisionType, CollisionType};
pub use console_log_on_screen_system::{ConsoleLogEvent, ConsoleLogOnScreenSystem};
pub use destroy_game_object::DestroyGameObject;
pub use dissolve_object::DissolveObject;
pub use event_object_found_on_raycast_system::{EventObjectFoundOnRaycastSystem, RaycastObjectFoundEvent};
pub use fade_object::FadeObject;
pub use features_manager::FeaturesManager;
pub use flying_turret_system::FlyingTurretSystem;
pub use follow_object_position_system::FollowObjectPositionSystem;
pub use follow_object_position_update_system::FollowObjectPositionUpdateSystem;
pub use gkc_utils::GkcUtils;
pub use head_explode_example::HeadExplodeExample;
pub use hide_body_part_on_character_system::HideBodyPartOnCharacterSystem;
pub use ignore_collision_helper::IgnoreCollisionHelper;
pub use ignore_collision_system::IgnoreCollisionSystem;

pub struct OthersPlugin;

impl Plugin for OthersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FeaturesManager>()
            .add_event::<AnimatorTriggerEnterEvent>()
            .add_event::<AnimatorTriggerExitEvent>()
            .add_event::<AnimatorTriggerEventRequest>()
            .add_event::<ConsoleLogEvent>()
            .add_event::<RaycastObjectFoundEvent>()
            .add_systems(Update, (
                add_force_to_object_system::update_add_force_to_object_system,
                animator_trigger_enter_exit_event::update_animator_trigger_enter_exit_event,
                animator_trigger_event::update_animator_trigger_event,
                change_object_colors::update_change_object_colors,
                console_log_on_screen_system::update_console_log_on_screen_system,
                destroy_game_object::update_destroy_game_object,
                dissolve_object::update_dissolve_object,
                event_object_found_on_raycast_system::update_event_object_found_on_raycast_system,
                fade_object::update_fade_object,
                flying_turret_system::update_flying_turret_system,
                follow_object_position_system::update_follow_object_position_system,
                follow_object_position_update_system::update_follow_object_position_update_system,
                head_explode_example::update_head_explode_example,
                hide_body_part_on_character_system::update_hide_body_part_on_character_system,
            ));
    }
}
