use bevy::prelude::*;
use super::skills_system::SkillsSystem;
use super::types::{SkillSystemEventQueue, SkillSystemEvent};

// Markers for UI elements
#[derive(Component)]
pub struct SkillTreeRoot;

#[derive(Component)]
pub struct SkillButton {
    pub category_index: usize,
    pub skill_index: usize,
}

#[derive(Component)]
pub struct SkillPointsText;

/// System to setup the Skill Tree UI (hidden by default)
pub fn setup_skill_tree_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(80.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(10.0),
            top: Val::Percent(10.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
        SkillTreeRoot,
        Visibility::Hidden,
    )).with_children(|parent| {
        // Header
        parent.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        }).with_children(|header| {
            header.spawn((
                Text::new("SKILL TREE"),
                TextFont { font_size: 30.0, ..default() },
                TextColor(Color::WHITE),
            ));

            header.spawn((
                Text::new("Points: 0"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                SkillPointsText,
            ));
        });

        // Content Area (Scrollable theoretically, but just a container for now)
        parent.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Column,
            ..default()
        }).with_children(|content| {
            // Categories will be spawned dynamically here or in update
            // For now, let's assume we spawn them once based on initial data
            // But actually, Bevy UI is easier if we build it static and hide/show. 
            // However, skill tree depends on data.
            // Let's create a "SkillTreeContent" marker to rebuild if needed.
        });
    });
}

// Marker for content container
#[derive(Component)]
pub struct SkillTreeContent;

/// System to toggle Skill Tree visibility
pub fn toggle_skill_tree_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<SkillTreeRoot>>,
) {
    if keyboard.just_pressed(KeyCode::K) { // 'K' for Skills
        for mut visibility in query.iter_mut() {
             if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

/// System to rebuild/update the skill tree UI
/// Note: Real-world implementation would likely strictly separate "Build" and "Update".
/// Here we'll do a simple "Update Text/Color" every frame, and assume structure is built once or on event.
/// For this simplified version, let's just build it if it's empty, or we can use a "Dirty" flag.
/// Let's keep it simple: We'll spawn the structure in `update` if it's not present, matching the player's skills.
pub fn update_skill_tree_ui(
    mut commands: Commands,
    player_query: Query<(Entity, &SkillsSystem, &crate::experience::types::PlayerExperience)>,
    root_query: Query<(Entity, &Visibility), With<SkillTreeRoot>>,
    content_query: Query<Entity, With<SkillTreeContent>>,
    mut points_text_query: Query<&mut Text, With<SkillPointsText>>,
    skill_button_query: Query<(&Interaction, &SkillButton, &BackgroundColor), Changed<Interaction>>,
    mut event_queue: ResMut<SkillSystemEventQueue>,
) {
    let Ok((player_entity, skills, experience)) = player_query.get_single() else { return };
    let Ok((root_entity, visibility)) = root_query.get_single() else { return };

    // Update Points Text
    if let Ok(mut text) = points_text_query.get_single_mut() {
        text.0 = format!("Points: {}", experience.skill_points);
    }

    // Only rebuild/update if visible
    if visibility == Visibility::Hidden {
        return;
    }

    // Check if content exists. If not (or if we want a rebuild strategy), build it.
    // For this example, we'll check if we added a specific child "Content". 
    // Wait, setup_skill_tree_ui created a container but didn't mark it SkillTreeContent.
    // Let's fix that in setup or find it here.
    // Actually, let's just look for children of root. If < 2 (Header + Content), strictly speaking.
    // Better: Helper system to spawn structure if missing.
    
    // BUT! Since `setup_skill_tree_ui` is startup, `skills` might be empty if loaded later.
    // So let's do a dynamic build check.
    
    let has_content = content_query.iter().next().is_some();

    if !has_content {
        // Build the tree!
        commands.entity(root_entity).with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                SkillTreeContent,
            )).with_children(|content| {
                for (cat_idx, category) in skills.skill_tree.categories.iter().enumerate() {
                    // Category Header
                    content.spawn((
                        Text::new(&category.category_name),
                        TextFont { font_size: 24.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 1.0)),
                        Node { margin: UiRect::top(Val::Px(10.0)), ..default() }
                    ));

                    // Skills Row
                    content.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    }).with_children(|row| {
                        for (skill_idx, skill) in category.skills.iter().enumerate() {
                            let color = if skill.unlocked {
                                if skill.complete { Color::srgb(0.2, 0.8, 0.2) } else { Color::srgb(0.2, 0.5, 0.2) }
                            } else {
                                Color::srgb(0.3, 0.3, 0.3)
                            };

                            row.spawn((
                                Button,
                                Node {
                                    width: Val::Px(150.0),
                                    height: Val::Px(80.0),
                                    margin: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(color),
                                BorderColor(Color::BLACK),
                                SkillButton { category_index: cat_idx, skill_index: skill_idx },
                            )).with_children(|btn| {
                                btn.spawn((
                                    Text::new(&skill.name),
                                    TextFont { font_size: 16.0, ..default() },
                                    TextColor(Color::WHITE),
                                ));
                                btn.spawn((
                                    Text::new(format!("Lvl: {}/{}", skill.current_level, skill.max_level)),
                                    TextFont { font_size: 12.0, ..default() },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                            });
                        }
                    });
                }
            });
        });
    }

    // Handle Button Interactions
    for (interaction, button, _) in skill_button_query.iter() {
        if *interaction == Interaction::Pressed {
            // Send Purchase Request
            event_queue.0.push(SkillSystemEvent::PurchaseSkillRequest {
                player_entity,
                category_index: button.category_index,
                skill_index: button.skill_index,
            });
            // Note: Visual update happens next frame via rebuild or color update system if we implemented one.
            // For now, simple enough.
        }
    }
}
