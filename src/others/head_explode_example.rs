use bevy::prelude::*;

/// Example component for head explode effect.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HeadExplodeExample {
    pub triggered: bool,
}

impl Default for HeadExplodeExample {
    fn default() -> Self {
        Self { triggered: false }
    }
}

pub fn update_head_explode_example(
    mut query: Query<&mut HeadExplodeExample>,
) {
    for mut example in query.iter_mut() {
        if example.triggered {
            example.triggered = false;
        }
    }
}
