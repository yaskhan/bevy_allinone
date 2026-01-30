use bevy::prelude::*;
use bevy::ui::{Node, Val, UiRect, Display, FlexDirection, AlignItems, JustifyContent, PositionType};
use crate::map::types::*;

// ============================================================================
// UI COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct MapRoot;

#[derive(Component)]
pub struct MinimapContainer;

#[derive(Component)]
pub struct FullMapContainer;

// ============================================================================
// UI SYSTEMS
// ============================================================================

pub fn setup_map_ui(mut commands: Commands) {
    // Canvas
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        // PickingBehavior::IGNORE, // Removed as it causes errors
        MapRoot,
    ))
    .with_children(|parent| {
        // Minimap (Top Right)
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(200.0),
                height: Val::Px(200.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            MinimapContainer,
        ));

        // Full Map (Centered, Hidden by default)
        parent.spawn((
            Node {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                display: Display::None,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            FullMapContainer,
        ));
    });
}

/// System to update marker icons in the UI (Minimap)
pub fn update_minimap_positions(
    player_query: Query<&Transform, With<crate::character::Player>>,
    markers: Query<(Entity, &GlobalTransform, &MapMarker)>,
    mut icons: Query<(&mut Node, &MapMarkerIcon)>,
    settings: Res<MapSettings>,
    // Need a way to spawn icons if they don't exist -> usually handled by a separate "sync" system or OnAdd
    // For now, assuming icons are manually added or we add auto-spawn logic here if needed
) {
    let Some(player_transform) = player_query.iter().next() else { return };
    let player_pos = player_transform.translation;

    for (mut node, icon) in icons.iter_mut() {
        if let Ok((_entity, marker_transform, marker)) = markers.get(icon.marker_entity) {
            if !marker.visible_in_minimap || unsafe { !settings.minimap_enabled } { // unsafe just to access field? No, normal field.
                 // Wait, accessing field is fine. Check settings.
                 if !settings.minimap_enabled {
                     node.display = Display::None;
                     continue;
                 }
            }

            if !marker.visible_in_minimap {
                node.display = Display::None;
                continue;
            }
            
            node.display = Display::Flex;
            
            let marker_pos = marker_transform.translation();
            let delta = marker_pos - player_pos;
            
            // Project 3D world delta to 2D UI space (XZ plane to XY plane)
            // Apply zoom and settings here
            let ui_x = delta.x * settings.minimap_zoom * 10.0;
            let ui_y = -delta.z * settings.minimap_zoom * 10.0;
            
            // Clamping to minimap bounds would go here
            
            node.left = Val::Px(ui_x);
            node.top = Val::Px(ui_y);
        } else {
             // Marker destroyed?
             node.display = Display::None; 
        }
    }
}

/// Toggles the Full Map visibility
pub fn update_map_visibility(
    settings: Res<MapSettings>,
    mut full_map_query: Query<&mut Node, With<FullMapContainer>>,
) {
    for mut node in full_map_query.iter_mut() {
        if settings.full_map_enabled {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }
}
