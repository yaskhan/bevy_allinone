use bevy::prelude::*;

/// Oxygen management system.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct OxygenSystem {
    pub max_oxygen: f32,
    pub current_oxygen: f32,
    pub drain_rate: f32,
    pub recover_rate: f32,
    pub is_underwater: bool,
    pub depleted: bool,
}

impl Default for OxygenSystem {
    fn default() -> Self {
        Self {
            max_oxygen: 100.0,
            current_oxygen: 100.0,
            drain_rate: 10.0,
            recover_rate: 15.0,
            is_underwater: false,
            depleted: false,
        }
    }
}

/// Update oxygen values based on environment state.
pub fn update_oxygen_system(
    time: Res<Time>,
    mut query: Query<&mut OxygenSystem>,
) {
    let dt = time.delta_secs();
    for mut oxygen in query.iter_mut() {
        if oxygen.is_underwater {
            oxygen.current_oxygen = (oxygen.current_oxygen - oxygen.drain_rate * dt).max(0.0);
        } else {
            oxygen.current_oxygen = (oxygen.current_oxygen + oxygen.recover_rate * dt)
                .min(oxygen.max_oxygen);
        }

        oxygen.depleted = oxygen.current_oxygen <= 0.0;
    }
}
