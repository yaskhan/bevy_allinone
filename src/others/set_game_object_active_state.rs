use bevy::prelude::*;

/// Sets active state on a target entity.
///
/// GKC reference: `setGameObjectActiveState.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SetGameObjectActiveState {
    pub target: Entity,
    pub active: bool,
}

impl Default for SetGameObjectActiveState {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            active: true,
        }
    }
}

pub fn update_set_game_object_active_state(
    query: Query<&SetGameObjectActiveState>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for state in query.iter() {
        let Ok(mut visibility) = visibility_query.get_mut(state.target) else { continue };
        *visibility = if state.active { Visibility::Visible } else { Visibility::Hidden };
    }
}
