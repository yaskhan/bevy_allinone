use bevy::prelude::*;

pub struct CameraEffectPlugin;

impl Plugin for CameraEffectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraEffectManager>()
           .register_type::<PixelEffectSettings>()
           .register_type::<SolidEffectSettings>()
           .add_systems(Update, update_camera_effects);
    }
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct CameraEffectManager {
    pub enabled: bool,
    pub active_effect: ActiveEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum ActiveEffect {
    #[default]
    None,
    Pixel,
    Solid,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PixelEffectSettings {
    pub block_count: f32,
    pub tint_color: Color,
}

impl Default for PixelEffectSettings {
    fn default() -> Self {
        Self {
            block_count: 128.0,
            tint_color: Color::WHITE,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SolidEffectSettings {
    pub color: Color,
}

impl Default for SolidEffectSettings {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
        }
    }
}

pub fn update_camera_effects(
    manager: Res<CameraEffectManager>,
    // This system would typically update post-processing materials
) {
    if !manager.enabled { return; }
    // Logic to toggle post-process nodes would go here
}
