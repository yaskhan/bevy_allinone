use bevy::prelude::*;

pub mod types;
pub mod systems;
pub mod damage_over_time;
pub mod destroyable;

pub use types::*;
pub use systems::*;
pub use damage_over_time::*;
pub use destroyable::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DamageEventQueue>()
            .init_resource::<DeathEventQueue>()
            .register_type::<Health>()
            .register_type::<Shield>()
            .register_type::<DamageReceiver>()
            .register_type::<MeleeCombat>()
            .register_type::<Blocking>()
            .register_type::<DamageOverTime>()
            .register_type::<DestroyableObject>()
            .add_systems(Update, (
                systems::update_timers,
                systems::regenerate_health,
                systems::regenerate_shields,
                systems::perform_melee_attacks,
                systems::perform_blocking,
                systems::process_damage_events,
                systems::update_damage_numbers,
                damage_over_time::update_damage_over_time,
                destroyable::handle_destroyable_death,
            ).chain());
    }
}
