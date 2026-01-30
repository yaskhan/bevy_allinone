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
) {
    let Some(player_transform) = player_query.iter().next() else { return };
    let player_pos = player_transform.translation;

    for (mut node, icon) in icons.iter_mut() {
        if let Ok((_entity, marker_transform, marker)) = markers.get(icon.marker_entity) {
            if !settings.show_minimap {
                 node.display = Display::None;
                 continue;
            }

            if !marker.visible_in_minimap {
                node.display = Display::None;
                continue;
            }
            
            // Check if hierarchy system hid it (display none)
            // If we set Flex here, we override the hierarchy system.
            // But this system runs *after*? Queries are random. 
            // We should respect existing Display::None if it was set by hierarchy system?
            // Actually, let's trust the hierarchy system to set it to None. 
            // If it is None, we shouldn't show it unless we are sure it's visible.
            // Problem: If hierarchy hides it, but this system sets it to Flex, it flickers or stays visible.
            
            // BETTER ARCHITECTURE:
            // add `is_visible` bool to `MapMarkerIcon`? Or check `display` state?
            // Checking display state is fragile.
            // Let's rely on `update_visible_map_elements` to set the BASE visibility.
            // If that set it to hidden, we skip projection.
            
            if node.display == Display::None {
                continue; 
            }
            
            // Ensure flex if not hidden
            node.display = Display::Flex;
            
            let marker_pos = marker_transform.translation();
            let delta = marker_pos - player_pos;
            
            // Project 3D world delta to 2D UI space
            let (x, y) = match settings.orientation {
                MapOrientation::XZ => (delta.x, -delta.z), // Top Down (3D)
                MapOrientation::XY => (delta.x, delta.y),  // Side Scroller (2D)
                MapOrientation::YZ => (delta.z, delta.y),  // Side (ZY)
            };
            
            let ui_x = x * settings.minimap_zoom * 10.0;
            let ui_y = y * settings.minimap_zoom * 10.0;
            
            // Clamp functionality could go here
            
            node.left = Val::Px(ui_x);
            node.top = Val::Px(ui_y);
        } else {
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
