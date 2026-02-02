use bevy::prelude::*;

/// Manages optional features.
///
/// GKC reference: `featuresManager.cs`
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct FeaturesManager {
    pub features: Vec<String>,
}

impl Default for FeaturesManager {
    fn default() -> Self {
        Self {
            features: Vec::new(),
        }
    }
}
