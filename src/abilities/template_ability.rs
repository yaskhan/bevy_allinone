use bevy::prelude::*;
use super::ability_info::AbilityInfo;

/// Template ability behavior for custom abilities.
///
/// GKC reference: `Custom Abilities/templateAbilitySystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TemplateAbilitySystem {
    pub ability_name: String,
    pub active: bool,
    pub secondary_action_triggered: bool,
}

impl Default for TemplateAbilitySystem {
    fn default() -> Self {
        Self {
            ability_name: "TemplateAbility".to_string(),
            active: false,
            secondary_action_triggered: false,
        }
    }
}

/// Sync template ability state from AbilityInfo.
pub fn update_template_ability(
    mut query: Query<(&AbilityInfo, &mut TemplateAbilitySystem)>,
) {
    for (ability, mut template) in query.iter_mut() {
        if ability.name != template.ability_name {
            continue;
        }
        template.active = ability.active;
    }
}
