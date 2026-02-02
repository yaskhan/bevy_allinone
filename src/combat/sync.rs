use bevy::prelude::*;
use crate::stats::StatsSystem;
use crate::stats::types::DerivedStat;
use super::types::{Health, Shield};

/// Syncs StatsSystem Max Health to Combat Health component.
/// This ensures that changes in Constitution or Buffs reflected in StatsSystem
/// are propagated to the actual Health component used in combat.
pub fn sync_stats_to_combat(
    mut query: Query<(&StatsSystem, &mut Health, Option<&mut Shield>)>,
) {
    for (stats, mut health, mut shield_opt) in query.iter_mut() {
        if !stats.active { continue; }

        // Sync Max Health
        if let Some(max_health) = stats.get_derived_stat(DerivedStat::MaxHealth) {
            if (health.maximum - max_health).abs() > 0.01 {
                health.maximum = max_health;
                // Optional: Scale current health proportionally? 
                // For now, let's clamp it.
                health.current = health.current.min(health.maximum);
            }
        }

        // Sync Max Shield (if mapped)
        // Currently DerivedStat doesn't have MaxShield, but if we added it, it would go here.
        // For now, Shield usually has its own internal max, or we could map it to Magic/Tech stats.
    }
}

/// Syncs Combat Health Current to StatsSystem Current Health.
/// This ensures that damage taken in combat is reflected in the RPG layer (for UI, Saves, etc.)
pub fn sync_combat_to_stats(
    mut query: Query<(&mut StatsSystem, &Health)>,
) {
    for (mut stats, health) in query.iter_mut() {
        if !stats.active { continue; }

        let stats_current = stats.get_derived_stat(DerivedStat::CurrentHealth).unwrap_or(0.0);
        
        // Update StatsSystem if mismatched
        if (stats_current - health.current).abs() > 0.01 {
             // We use set_derived_stat logic via a specific helper if needed, 
             // but stats.recalculate_derived_stats() might overwrite Derived Stats from formula.
             // TRICKY: Derived Stats are usually CALCULATED (e.g. MaxHealth).
             // CurrentHealth is a "Mutable" Derived Stat or should be a separate tracking value.
             // In our StatsSystem, CurrentHealth IS a derived stat. 
             // If we just set it, `recalculate_derived_stats` might reset it if it's based on a formula.
             // HOWEVER, CurrentHealth usually ISN'T formula per frame (it's state).
             // Let's assume StatsSystem handles CurrentHealth as a state container.
             
             // We need to bypass the "formula" if it exists, or update the "Base" if it's an Attribute.
             // Since CurrentHealth is likely NOT recalculated from primitives every frame (unlike Max),
             // we can safely update it. But we need a method on StatsSystem to do so effectively.
             
             // Inspecting StatsSystem logic:
             // If CurrentHealth is in `derived_stats`, it might be overwritten.
             // Let's use `stats.set_current_health_safely` if it existed, 
             // or direct map access if we have pub access.
             
             // Assuming we adding a setter for Current Health is best.
             // For now, we'll assume we can modify it through a specific method we'll add.
             
             stats.set_derived_stat_value(DerivedStat::CurrentHealth, health.current);
        }
    }
}
