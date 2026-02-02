pub mod types;
pub mod systems;
pub mod ability_info;
pub mod player_abilities;

use bevy::prelude::*;
use types::*;
use systems::*;
use ability_info::*;
use player_abilities::*;

// Re-export specific types for cleaner imports
pub use types::AbilityStatus;
pub use types::AbilityInputType;
pub use types::EnergyConsumptionType;
pub use ability_info::AbilityInfo;
pub use types::ActivateAbilityEvent;
pub use types::DeactivateAbilityEvent;
pub use types::SetAbilityEnabledEvent;

pub use player_abilities::PlayerAbilitiesSystem;

/// Plugin for the abilities system
pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<AbilityInfo>()
            .register_type::<PlayerAbilitiesSystem>()
            // Events
            .add_event::<ActivateAbilityEvent>()
            .add_event::<DeactivateAbilityEvent>()
            .add_event::<SetAbilityEnabledEvent>()
            // Add systems
            .add_systems(Update, (
                update_abilities,
                handle_ability_activation,
                handle_ability_deactivation,
                handle_ability_enabled_events,
            ));
    }
}
