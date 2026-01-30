use bevy::prelude::*;
use bevy::ui::{Node, Val, UiRect, Display, FlexDirection, AlignItems, JustifyContent};
// use bevy::hierarchy::DespawnRecursiveExt; // Disabled
use crate::level_manager::types::*;

// ============================================================================
// UI COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct TravelStationMenu;

#[derive(Component)]
pub struct TravelStationButton {
    pub target_scene: i32,
    pub target_id: i32,
}

#[derive(Component)]
pub struct TravelStationButtonContainer;

// ============================================================================
// UI SYSTEMS
// ============================================================================

/// Setup UI (spawn hidden menu)
pub fn setup_travel_ui(mut commands: Commands) {
    // Root node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::None, // Hidden by default
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            GlobalZIndex(100),
            TravelStationMenu,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                Text::new("Travel Station"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            
            // List container (will be populated dynamically)
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                TravelStationButtonContainer, // Tagged container
            ));
            
            // Close button help
            parent.spawn((
                Text::new("Press 'Esc' to Close"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

/// Update UI State (Show/Hide/Populate)
pub fn update_travel_ui(
    mut commands: Commands,
    stations: Query<&TravelStation>,
    global_state: Res<LevelManagerGlobalState>,
    mut menu_query: Query<(Entity, &mut Node), With<TravelStationMenu>>,
    mut container_query: Query<(Entity, Option<&Children>), With<TravelStationButtonContainer>>,
) {
    let mut is_using_station = false;
    let mut active_station_id = -1;
    let mut active_station_scene = -1;

    for station in stations.iter() {
        if station.using_station {
            is_using_station = true;
            active_station_id = station.current_level_manager_id;
            active_station_scene = station.current_scene_number;
            break;
        }
    }

    if let Some((_menu_entity, mut node)) = menu_query.iter_mut().next() {
        if is_using_station {
            if node.display == Display::None {
                // OPEN MENU
                node.display = Display::Flex;
                info!("Opening Travel Station Menu");

                // Populate buttons
                if let Some((container_entity, children_option)) = container_query.iter_mut().next() {
                    
                    // Clear existing buttons manually
                    if let Some(children) = children_option {
                        for child in children.iter() {
                            commands.entity(child).despawn(); 
                        }
                    }
                    
                    // Add buttons for discovered stations
                    for dest in &global_state.discovered_stations {
                        // Don't show current station as destination
                        if dest.level_manager_id == active_station_id && dest.scene_number == active_station_scene {
                            continue;
                        }

                        commands.entity(container_entity).with_children(|parent| {
                            parent.spawn((
                                Button,
                                Node {
                                    width: Val::Px(200.0),
                                    height: Val::Px(50.0),
                                    margin: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                                TravelStationButton {
                                    target_scene: dest.scene_number,
                                    target_id: dest.level_manager_id,
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(dest.name.clone()),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                        });
                    }
                }
            }
        } else if !is_using_station && node.display != Display::None {
            // CLOSE MENU
            node.display = Display::None;
        }
    }
}

/// Handle Button Clicks
pub fn handle_travel_button_interactions(
    interaction_query: Query<
        (&Interaction, &TravelStationButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut request_queue: ResMut<RequestLevelChangeEventQueue>, 
    mut stations: Query<&mut TravelStation>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("Travel Button Pressed: Target ID {}", button.target_id);
            
            // Send request
            request_queue.0.push(RequestLevelChangeEvent {
                target_scene: button.target_scene,
                target_level_manager_id: button.target_id,
                delay: 1.0, 
            });

            // Close menu (stop using station)
            for mut station in stations.iter_mut() {
                if station.using_station {
                    station.using_station = false;
                }
            }
        }
    }
}
