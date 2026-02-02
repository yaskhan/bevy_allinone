use bevy::prelude::*;
use super::stats_system::StatsSystem;
use super::types::{AddModifierEventQueue, RemoveModifierEventQueue};

/// System to update stats (modifiers, regeneration, etc.)
pub fn update_stats(
    time: Res<Time>,
    mut stats_query: Query<&mut StatsSystem>,
) {
    let delta_time = time.delta_secs();

    for mut stats in stats_query.iter_mut() {
        // Update modifiers
        stats.update_modifiers(delta_time);
        
        // Apply modifiers
        stats.apply_modifiers();
    }
}

/// System to handle stat change events
pub fn handle_stat_changes() {
    // Event handling would be added here if needed
}

/// System to handle modifier events
pub fn handle_modifier_events(
    mut stats_query: Query<&mut StatsSystem>,
    mut add_queue: ResMut<AddModifierEventQueue>,
    mut remove_queue: ResMut<RemoveModifierEventQueue>,
) {
    // Note: This logic assumes modifiers are applied to ALL StatsSystems.
    // In a multi-character game, we'd need entity targeting in the events.
    // For now, let's assume it's for the player or all entities.
    
    // Process add events
    for event in add_queue.0.drain(..) {
        for mut stats in stats_query.iter_mut() {
            stats.add_modifier(event.modifier.clone());
        }
    }

    // Process remove events
    for event in remove_queue.0.drain(..) {
        for mut stats in stats_query.iter_mut() {
            stats.remove_modifier(&event.modifier_name);
        }
    }
}
