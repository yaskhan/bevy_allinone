use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use std::collections::HashMap;

#[derive(Event, Debug, Clone)]
pub struct ExamineInventoryItemEvent {
    pub owner: Entity,
    pub item_id: String,
    pub source_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct InventoryItemPreviewRegistry {
    pub previews: HashMap<String, Handle<Scene>>,
}

#[derive(Resource, Debug, Clone)]
pub struct InventoryExamineSettings {
    pub render_layer: u8,
    pub rotate_speed: f32,
    pub preview_distance: f32,
    pub camera_fov: f32,
    pub zoom_speed: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub transition_duration: f32,
}

impl Default for InventoryExamineSettings {
    fn default() -> Self {
        Self {
            render_layer: 2,
            rotate_speed: 0.6,
            preview_distance: 2.0,
            camera_fov: 45.0_f32.to_radians(),
            zoom_speed: 0.4,
            min_distance: 0.6,
            max_distance: 6.0,
            transition_duration: 0.5,
        }
    }
}

#[derive(Component)]
pub struct InventoryExamineCamera;

#[derive(Component)]
pub struct InventoryExaminePreview {
    pub item_id: String,
    pub source_entity: Option<Entity>,
}

#[derive(Component)]
pub struct ExaminePreviewAnimation {
    pub timer: Timer,
    pub start_scale: Vec3,
    pub end_scale: Vec3,
    pub start_rotation: Quat,
    pub end_rotation: Quat,
}

#[derive(Component)]
pub struct ExamineUIRoot;

#[derive(Component)]
pub struct ExamineTakeButton;

#[derive(Component)]
pub struct ExamineCloseButton;

pub fn ensure_examine_camera(
    mut commands: Commands,
    settings: Res<InventoryExamineSettings>,
    query: Query<Entity, With<InventoryExamineCamera>>,
) {
    if query.iter().next().is_some() {
        return;
    }

    let layer = RenderLayers::layer(settings.render_layer);
    commands.spawn((
        Name::new("InventoryExamineCamera"),
        Camera3d::default(),
        InventoryExamineCamera,
        Transform::from_xyz(0.0, 0.0, settings.preview_distance),
        GlobalTransform::default(),
        layer,
    ));
}

use super::components::InventoryUIRoot;
use crate::interaction::{InteractionEvent, InteractionEventQueue, InteractionType};
use crate::inventory::types::InventoryItem;

pub fn handle_examine_item(
    mut commands: Commands,
    mut events: ResMut<Events<ExamineInventoryItemEvent>>,
    registry: Res<InventoryItemPreviewRegistry>,
    settings: Res<InventoryExamineSettings>,
    existing_previews: Query<Entity, With<InventoryExaminePreview>>,
    existing_ui: Query<Entity, With<ExamineUIRoot>>,
    asset_server: Res<AssetServer>,
) {
    let Some(event) = events.drain().last() else { return };

    // Cleanup old
    for entity in existing_previews.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in existing_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Spawn 3D Preview
    if let Some(scene) = registry.previews.get(&event.item_id) {
        let layer = RenderLayers::layer(settings.render_layer);
        
        commands.spawn((
            Name::new(format!("InventoryPreview {}", event.item_id)),
            SceneRoot(scene.clone()),
            InventoryExaminePreview {
                item_id: event.item_id.clone(),
                source_entity: event.source_entity,
            },
            Transform::from_scale(Vec3::splat(0.1)), // Start small
            GlobalTransform::default(),
            ExaminePreviewAnimation {
                timer: Timer::from_seconds(settings.transition_duration, TimerMode::Once),
                start_scale: Vec3::splat(0.1),
                end_scale: Vec3::ONE,
                start_rotation: Quat::from_rotation_y(std::f32::consts::PI), 
                end_rotation: Quat::IDENTITY,
            },
            layer,
        ));
    }

    // Spawn UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ExamineUIRoot,
            GlobalZIndex(200), // Above inventory
            // Pickable?
        ))
        .with_children(|parent| {
            // Container for buttons
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
            )).with_children(|buttons| {
                // Take Button (Only if source entity exists)
                if event.source_entity.is_some() {
                    buttons.spawn((
                        Button,
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                        ExamineTakeButton,
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new("Take"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                        ));
                    });
                }

                // Close Button
                buttons.spawn((
                    Button,
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
                    ExamineCloseButton,
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Close"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                    ));
                });
            });
            
            // Instructions
            parent.spawn((
                Text::new("Scroll to Zoom | Drag to Rotate (Not Impl)"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

pub fn update_examine_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut ExaminePreviewAnimation)>,
) {
    for (entity, mut transform, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());
        let t = animation.timer.fraction(); // 0.0 to 1.0 linear
        // Ease out cubic
        let eased_t = 1.0 - (1.0 - t).powi(3);

        transform.scale = animation.start_scale.lerp(animation.end_scale, eased_t);
        transform.rotation = animation.start_rotation.slerp(animation.end_rotation, eased_t);

        if animation.timer.finished() {
            commands.entity(entity).remove::<ExaminePreviewAnimation>();
        }
    }
}

pub fn rotate_examine_preview(
    time: Res<Time>,
    settings: Res<InventoryExamineSettings>,
    mut query: Query<&mut Transform, (With<InventoryExaminePreview>, Without<ExaminePreviewAnimation>)>,
) {
    let angle = settings.rotate_speed * time.delta_secs();
    for mut transform in query.iter_mut() {
        transform.rotate_y(angle);
    }
}

pub fn handle_examine_ui_interaction(
    mut commands: Commands,
    mut exposure_events: ResMut<InteractionEventQueue>, 
    player_query: Query<Entity, With<crate::inventory::components::Inventory>>, // Assume player has inventory
    preview_query: Query<(Entity, &InventoryExaminePreview)>,
    ui_query: Query<Entity, With<ExamineUIRoot>>,
    take_btn: Query<&Interaction, (Changed<Interaction>, With<ExamineTakeButton>)>,
    close_btn: Query<&Interaction, (Changed<Interaction>, With<ExamineCloseButton>)>,
) {
    let Ok(player_entity) = player_query.get_single() else { return };

    // Handle Take
    if let Ok(interaction) = take_btn.get_single() {
        if *interaction == Interaction::Pressed {
            if let Ok((_, preview)) = preview_query.get_single() {
                if let Some(source) = preview.source_entity {
                    // Queue Pickup
                    exposure_events.0.push(InteractionEvent {
                        source: player_entity,
                        target: source,
                        interaction_type: InteractionType::Pickup,
                    });
                }
            }
            // Close UI
            close_examine_mode(&mut commands, &preview_query, &ui_query);
        }
    }

    // Handle Close
    if let Ok(interaction) = close_btn.get_single() {
        if *interaction == Interaction::Pressed {
            close_examine_mode(&mut commands, &preview_query, &ui_query);
        }
    }
}

fn close_examine_mode(
    commands: &mut Commands,
    preview_query: &Query<(Entity, &InventoryExaminePreview)>,
    ui_query: &Query<Entity, With<ExamineUIRoot>>,
) {
    for (entity, _) in preview_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_examine_zoom(
    mut mouse_wheel: EventReader<bevy::input::mouse::MouseWheel>,
    mut settings: ResMut<InventoryExamineSettings>,
    mut camera_query: Query<&mut Transform, With<InventoryExamineCamera>>,
) {
    let mut scroll_delta = 0.0;
    for event in mouse_wheel.read() {
        scroll_delta += event.y;
    }

    if scroll_delta.abs() <= f32::EPSILON {
        return;
    }

    settings.preview_distance = (settings.preview_distance - scroll_delta * settings.zoom_speed)
        .clamp(settings.min_distance, settings.max_distance);

    if let Ok(mut transform) = camera_query.single_mut() {
        transform.translation.z = settings.preview_distance;
    }
}
