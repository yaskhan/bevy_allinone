use bevy::prelude::*;
pub mod photo_mode;

pub struct CameraEffectPlugin;

impl Plugin for CameraEffectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraEffectManager>()
           .register_type::<PixelEffectSettings>()
           .register_type::<SolidEffectSettings>()
           .add_plugins(photo_mode::PhotoModePlugin)
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
    Overlay, // New: Screen overlay
    Noise,   // New: Camera grain/noise
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

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct OverlayEffectSettings {
    pub texture_path: String,
    pub opacity: f32,
    pub color: Color,
}

impl Default for OverlayEffectSettings {
    fn default() -> Self {
        Self {
            texture_path: "".to_string(),
            opacity: 0.5,
            color: Color::WHITE,
        }
    }
}

pub fn update_camera_effects(
    manager: Res<CameraEffectManager>,
    // System to drive shader parameters
) {
    if !manager.enabled { return; }
    
    match manager.active_effect {
        ActiveEffect::None => {},
        ActiveEffect::Pixel => {
            // Update pixelation shader
        },
        ActiveEffect::Solid => {
            // Update screen fade shader
        },
        ActiveEffect::Overlay => {
            // Update overlay image/texture
        },
        ActiveEffect::Noise => {
            // Update noise intensity
        },
    }
}
