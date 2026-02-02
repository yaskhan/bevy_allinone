pub mod types;
pub mod systems;
pub mod ability_info;
pub mod player_abilities;
pub mod ui;
pub mod dash;
pub mod magic_spell;
pub mod oxygen;
pub mod stamina;
pub mod throw_trajectory;
pub mod wall_running_zone;
pub mod particle_detection;
pub mod template_ability;
pub mod weapon_integration;
pub mod ability_pickups;
pub mod player_teleport;

use bevy::prelude::*;
use types::*;
use systems::*;
use ability_info::*;
use player_abilities::*;
use ui::*;
use dash::*;
use magic_spell::*;
use oxygen::*;
use stamina::*;
use throw_trajectory::*;
use wall_running_zone::*;
use particle_detection::*;
use template_ability::*;
use weapon_integration::*;
use ability_pickups::*;
use player_teleport::*;

// Re-export specific types for cleaner imports
pub use types::AbilityStatus;
pub use types::AbilityInputType;
pub use types::EnergyConsumptionType;
pub use ability_info::AbilityInfo;
pub use types::ActivateAbilityEvent;
pub use types::DeactivateAbilityEvent;
pub use types::SetAbilityEnabledEvent;
pub use types::AbilityCooldownEvent;
pub use types::AbilityTimeLimitEvent;

pub use player_abilities::PlayerAbilitiesSystem;
pub use ui::{AbilityWheelUI, AbilitySlotElement};
pub use dash::DashAbility;
pub use magic_spell::MagicSpellAbility;
pub use types::MagicSpellCastEvent;
pub use oxygen::OxygenSystem;
pub use stamina::StaminaSystem;
pub use throw_trajectory::ThrowObjectTrajectory;
pub use wall_running_zone::{WallRunningZone, WallRunningZoneTracker};
pub use particle_detection::{
    ParticleCollisionDetector,
    ParticleTriggerDetector,
    ParticleCollisionEvent,
    ParticleTriggerEvent,
    ParticleCollisionEventQueue,
    ParticleTriggerEventQueue,
};
pub use template_ability::TemplateAbilitySystem;
pub use weapon_integration::AbilityWeaponIntegration;
pub use ability_pickups::AbilityPickup;
pub use player_teleport::PlayerTeleportAbility;
pub use player_teleport::{TeleportStartEvent, TeleportEndEvent, TeleportStartEventQueue, TeleportEndEventQueue};

/// Plugin for the abilities system
pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<AbilityInfo>()
            .register_type::<PlayerAbilitiesSystem>()
            // Events (Resource Queues)
            .init_resource::<ActivateAbilityEventQueue>()
            .init_resource::<DeactivateAbilityEventQueue>()
            .init_resource::<SetAbilityEnabledEventQueue>()
            .init_resource::<AbilityCooldownEventQueue>()
            .init_resource::<AbilityTimeLimitEventQueue>()
            .init_resource::<MagicSpellCastEventQueue>()
            .init_resource::<TeleportStartEventQueue>()
            .init_resource::<TeleportEndEventQueue>()
            .init_resource::<ParticleCollisionEventQueue>()
            .init_resource::<ParticleTriggerEventQueue>()
            // Add systems
            .add_systems(Update, (
                update_player_abilities_context,
                update_abilities,
                update_ability_wheel_ui,
                update_ability_slot_elements,
                handle_ability_input,
                start_dash_from_ability,
                update_dash_ability,
                start_magic_spell_cast,
                update_magic_spell_cast,
                update_oxygen_system,
                update_stamina_system,
                update_throw_trajectory,
                update_wall_running_zones,
                handle_particle_collision_events,
                handle_particle_trigger_events,
                update_template_ability,
                update_weapon_ability_hooks,
                update_teleport_target,
                handle_teleport_input,
                handle_ability_activation,
                handle_ability_deactivation,
                handle_ability_enabled_events,
            ));
    }
}
