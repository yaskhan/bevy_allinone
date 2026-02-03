use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct InventoryAutoEquipSettings {
    pub equip_weapons_when_picked: bool,
}

impl Default for InventoryAutoEquipSettings {
    fn default() -> Self {
        Self {
            equip_weapons_when_picked: false,
        }
    }
}
