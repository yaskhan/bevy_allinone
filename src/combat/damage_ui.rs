use bevy::prelude::*;
use super::types::*;
use super::result_queue::*;
use crate::character::Player;

/// Component for the full-screen damage tint effect.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DamageScreenEffect {
    /// Current intensity of the redness (0.0 to 1.0).
    pub intensity: f32,
    /// How fast the tint fades out per second.
    pub fade_speed: f32,
    /// Color of the tint (default red).
    pub color: Color,
}

impl Default for DamageScreenEffect {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            fade_speed: 2.0,
            color: Color::srgba(1.0, 0.0, 0.0, 0.3),
        }
    }
}

/// Component for a directional damage indicator UI element.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DamageIndicator {
    /// How long this indicator lasts.
    pub lifetime: f32,
    /// World position of the damage source.
    pub source_position: Vec3,
}

/// Resource to hold settings for damage feedback.
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct DamageFeedbackSettings {
    pub flash_enabled: bool,
    pub indicators_enabled: bool,
    pub indicator_lifetime: f32,
}

impl Default for DamageFeedbackSettings {
    fn default() -> Self {
        Self {
            flash_enabled: true,
            indicators_enabled: true,
            indicator_lifetime: 2.0,
        }
    }
}

/// System to spawn Damage UI for the player.
/// Should be called once during setup or player spawn.
pub fn setup_damage_ui(mut commands: Commands) {
    // Spawn screen tint overlay
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        GlobalZIndex(100), // Ensure it's on top
        DamageScreenEffect::default(),
        // PickingBehavior::IGNORE removed due to potential import issue
    ));
}

/// System to update damage indicators and screen flash.
pub fn update_damage_ui(
    mut commands: Commands,
    time: Res<Time>,
    // Query for the effect overlay
    mut effect_query: Query<(&mut DamageScreenEffect, &mut BackgroundColor)>,
    // Query for indicators
    mut indicator_query: Query<(Entity, &mut DamageIndicator, &mut Transform, &mut Node)>,
    // Query for player camera to calculate directions
    camera_query: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    // Query for player entity check (to ensure we only show for player)
    player_query: Query<Entity, With<Player>>,
) {
    let dt = time.delta_secs();

    // 1. Update Screen Flash
    for (mut effect, mut bg_color) in effect_query.iter_mut() {
        if effect.intensity > 0.0 {
            effect.intensity = (effect.intensity - effect.fade_speed * dt).max(0.0);
            let mut color = effect.color;
            color.set_alpha(effect.intensity); // Fading alpha
            bg_color.0 = color;
        }
    }

    // 2. Update Indicators
    let (cam_xf, _cam) = match camera_query.iter().next() {
        Some(c) => c,
        None => return, // No camera, can't update visual directions
    };
    
    // We need the player's transform as the center, or just use camera position?
    // Using camera position usually feels better for FPS/TPS UI.
    let cam_pos = cam_xf.translation();
    let cam_forward = cam_xf.forward();
    let cam_right = cam_xf.right();

    // flattened to XZ plane for 2D UI comparison
    let flat_forward = Vec3::new(cam_forward.x, 0.0, cam_forward.z).normalize_or_zero();
    let flat_right = Vec3::new(cam_right.x, 0.0, cam_right.z).normalize_or_zero();

    for (entity, mut indicator, mut transform, _) in indicator_query.iter_mut() {
        indicator.lifetime -= dt;
        if indicator.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // Calculate Angle
        let direction_to_source = (indicator.source_position - cam_pos).normalize_or_zero();
        let flat_dir = Vec3::new(direction_to_source.x, 0.0, direction_to_source.z).normalize_or_zero();

        // Screen space angle logic
        // Angle between camera forward and source direction
        let angle = flat_forward.angle_between(flat_dir);
        // Determine sign (left or right)
        let is_right = flat_forward.cross(flat_dir).y < 0.0;
        let final_angle = if is_right { -angle } else { angle };

        // Rotate the indicator UI element
        // Assuming the indicator sprite points UP by default. 
        // We rotate it around Z axis.
        transform.rotation = Quat::from_rotation_z(final_angle);
    }
}

/// System to listen for damage events and trigger UI effects.
pub fn trigger_damage_ui(
    mut commands: Commands,
    damage_queue: Res<DamageResultQueue>,
    mut effect_query: Query<&mut DamageScreenEffect>,
    player_query: Query<Entity, With<Player>>,
    settings: Res<DamageFeedbackSettings>,
    asset_server: Res<AssetServer>,
    transform_query: Query<&GlobalTransform>,
) {
    // Only process if player exists
    let player_entity = match player_query.iter().next() {
        Some(p) => p,
        None => return,
    };

    for event in damage_queue.0.iter() {
        if event.target == player_entity && (event.final_amount > 0.0 || event.shielded_amount > 0.0) {
            // Trigger Flash
            if settings.flash_enabled {
                for mut effect in effect_query.iter_mut() {
                    effect.intensity = 0.5; // Set flash intensity
                }
            }

            // Trigger Indicator
            if settings.indicators_enabled {
                // Determine source position for indicator
                let source_pos = if let Some(source) = event.source {
                    if let Ok(transform) = transform_query.get(source) {
                        Some(transform.translation())
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(pos) = source_pos {
                    // Spawn Indicator
                    commands.spawn((
                        ImageNode::new(asset_server.load("ui/damage_indicator.png")),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(50.0),
                            left: Val::Percent(50.0),
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            margin: UiRect::all(Val::Auto), 
                             ..default()
                        },
                        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                        GlobalZIndex(101),
                        DamageIndicator {
                            lifetime: settings.indicator_lifetime,
                            source_position: pos,
                        },
                    ));
                }
            }
        }
    }
}
