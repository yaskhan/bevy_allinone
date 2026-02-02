pub mod types;
pub mod stats_system;
pub mod systems;

use bevy::prelude::*;
use types::*;
use stats_system::*;
use systems::*;

// Re-export specific types for cleaner imports
pub use types::{
    CoreAttribute, DerivedStat, ModifierType, StatModifier, StatEntry, StatValue,
    StatTemplate, StatTemplateEntry, StatChangedEvent, CoreAttributeChangedEvent,
    AddModifierEvent, RemoveModifierEvent,
};
pub use stats_system::StatsSystem;
pub use systems::*;

/// Plugin for the stats system
pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<StatsSystem>()
            .init_resource::<AddModifierEventQueue>()
            // Add systems
            .add_systems(Update, (
                update_stats,
                handle_stat_changes,
                handle_modifier_events,
            ));
    }
}
