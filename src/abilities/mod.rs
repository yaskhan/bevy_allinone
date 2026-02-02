pub mod types;
pub mod systems;
pub mod ability_info;
pub mod player_abilities;
pub mod ui;

use bevy::prelude::*;
use types::*;
use systems::*;
use ability_info::*;
use player_abilities::*;
use ui::*;

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
            .add_event::<AbilityCooldownEvent>()
            .add_event::<AbilityTimeLimitEvent>()
            // Add systems
            .add_systems(Update, (
                update_player_abilities_context,
                update_abilities,
                update_ability_wheel_ui,
                update_ability_slot_elements,
                handle_ability_input,
                handle_ability_activation,
                handle_ability_deactivation,
                handle_ability_enabled_events,
            ));
    }
}
