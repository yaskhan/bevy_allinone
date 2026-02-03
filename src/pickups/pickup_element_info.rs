use bevy::prelude::*;

/// Pickup element metadata.
///
/// GKC reference: `pickUpElementInfo.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpElementInfo {
    pub pick_up_type: String,
    pub pick_up_type_list: Vec<PickUpTypeElementInfo>,
    pub use_general_icon: bool,
    pub general_icon_path: String,
    pub use_custom_icon_prefab: bool,
    pub custom_icon_prefab: String,
}

impl Default for PickUpElementInfo {
    fn default() -> Self {
        Self {
            pick_up_type: String::new(),
            pick_up_type_list: Vec::new(),
            use_general_icon: false,
            general_icon_path: String::new(),
            use_custom_icon_prefab: false,
            custom_icon_prefab: String::new(),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct PickUpTypeElementInfo {
    pub name: String,
    pub pick_up_object: String,
    pub pickup_icon_path: String,
    pub use_custom_pickup_icon: bool,
    pub use_custom_icon_prefab: bool,
    pub custom_icon_prefab: String,
}

impl Default for PickUpTypeElementInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            pick_up_object: String::new(),
            pickup_icon_path: String::new(),
            use_custom_pickup_icon: false,
            use_custom_icon_prefab: false,
            custom_icon_prefab: String::new(),
        }
    }
}
