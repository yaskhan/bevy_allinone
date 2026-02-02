use bevy::prelude::*;

/// Hides a specific body part entity.
///
/// GKC reference: `hideBodyPartOnCharacterSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HideBodyPartOnCharacterSystem {
    pub target: Entity,
    pub hidden: bool,
}

impl Default for HideBodyPartOnCharacterSystem {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            hidden: false,
        }
    }
}

pub fn update_hide_body_part_on_character_system(
    mut query: Query<&HideBodyPartOnCharacterSystem>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for system in query.iter_mut() {
        let Ok(mut visibility) = visibility_query.get_mut(system.target) else { continue };
        *visibility = if system.hidden { Visibility::Hidden } else { Visibility::Visible };
    }
}
