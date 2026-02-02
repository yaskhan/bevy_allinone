use bevy::prelude::*;
use bevy::app::App;

pub mod types;
pub mod systems;
pub mod weapon_integration;
pub mod camera_integration;
pub mod parenting_integration;

pub struct ActionSystemPlugin;

impl Plugin for ActionSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components
            .register_type::<types::ActionSystem>()
            .register_type::<types::PlayerActionSystem>()
            .register_type::<types::AnimatorParameters>()
            .register_type::<types::MatchTargetConfig>()
            .register_type::<types::BoneParentingConfig>()
            .register_type::<types::ParentedObject>()
            .register_type::<types::CustomActionInfo>()
            
            // Register resources
            .init_resource::<types::StartActionEventQueue>()
            .init_resource::<types::EndActionEventQueue>()
            .init_resource::<types::ActivateCustomActionEventQueue>()
            .init_resource::<types::StopCustomActionEventQueue>()
            .init_resource::<types::ActionInterruptedEventQueue>()
            .init_resource::<types::ActionEventTriggeredQueue>()
            .init_resource::<types::RemoteActionEventQueue>()
            .init_resource::<types::CameraEventQueue>()
            .init_resource::<types::PhysicsEventQueue>()
            .init_resource::<types::StateChangeEventQueue>()
            .init_resource::<types::WeaponEventQueue>()
            .init_resource::<types::PowerEventQueue>()
            .init_resource::<types::ParentingEventQueue>()
            .init_resource::<types::CustomActionManager>()
            
            // Register systems
            .add_systems(Update, (
                systems::update_action_system,
                systems::process_action_events_system,
                systems::update_animator_parameters_system,
                systems::apply_match_target_system,
                systems::update_walk_to_target_system,
                systems::handle_custom_action_activation_system,
                systems::update_custom_action_manager_system,
                systems::block_action_inputs_system,
                weapon_integration::process_weapon_events,
                weapon_integration::process_power_events,
                camera_integration::process_camera_events,
                parenting_integration::process_parenting_events,
                parenting_integration::update_bone_parenting_system,
            ).chain());
    }
}
