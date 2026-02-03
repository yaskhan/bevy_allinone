use bevy::prelude::*;

/// Weapon attachment pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WeaponAttachmentPickup {
    pub attachment_id: String,
    pub attachment_name: String,
}

impl Default for WeaponAttachmentPickup {
    fn default() -> Self {
        Self {
            attachment_id: String::new(),
            attachment_name: String::new(),
        }
    }
}
