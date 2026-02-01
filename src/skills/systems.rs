use bevy::prelude::*;
use super::skills_system::SkillsSystem;

/// Skills system update
pub fn skills_system_update(
    mut query: Query<&mut SkillsSystem>,
) {
    for mut skills_system in query.iter_mut() {
        if !skills_system.active {
            continue;
        }

        // Add skill update logic here
        // For example, event processing or automatic value updates
        // This was largely empty in the original file, so keeping it minimal but modular
    }
}
