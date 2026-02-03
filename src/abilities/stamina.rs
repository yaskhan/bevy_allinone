use bevy::prelude::*;
use crate::stats::{StatsSystem};
use crate::stats::types::DerivedStat;

/// Stamina management system.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct StaminaSystem {
    pub max_stamina: f32,
    pub current_stamina: f32,
    pub regen_rate: f32,
    pub drain_rate: f32,
    pub is_exerting: bool,
    pub use_stats: bool,
}

impl Default for StaminaSystem {
    fn default() -> Self {
        Self {
            max_stamina: 100.0,
            current_stamina: 100.0,
            regen_rate: 15.0,
            drain_rate: 20.0,
            is_exerting: false,
            use_stats: true,
        }
    }
}

/// Update stamina values and optionally sync to the stats system.
pub fn update_stamina_system(
    time: Res<Time>,
    mut query: Query<(&mut StaminaSystem, Option<&mut StatsSystem>)>,
) {
    let dt = time.delta_secs();
    for (mut stamina, mut stats) in query.iter_mut() {
        if stamina.use_stats {
            if let Some(stats_system) = stats.as_deref_mut() {
                if let Some(current) = stats_system.get_derived_stat(DerivedStat::CurrentStamina) {
                    stamina.current_stamina = *current;
                }
                if let Some(max) = stats_system.get_derived_stat(DerivedStat::MaxStamina) {
                    stamina.max_stamina = *max;
                }
            }
        }

        if stamina.is_exerting {
            stamina.current_stamina = (stamina.current_stamina - stamina.drain_rate * dt).max(0.0);
        } else {
            stamina.current_stamina = (stamina.current_stamina + stamina.regen_rate * dt)
                .min(stamina.max_stamina);
        }

        if stamina.use_stats {
            if let Some(stats_system) = stats.as_deref_mut() {
                stats_system.set_derived_stat(DerivedStat::CurrentStamina, stamina.current_stamina);
                stats_system.set_derived_stat(DerivedStat::MaxStamina, stamina.max_stamina);
            }
        }
    }
}
