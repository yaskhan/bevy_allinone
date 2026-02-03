use bevy::prelude::*;
use bevy::render::view::visibility::RenderLayers;
use std::collections::HashMap;

#[derive(Event, Debug, Clone)]
pub struct ExamineInventoryItemEvent {
    pub owner: Entity,
    pub item_id: String,
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
        }
    }
}

#[derive(Component)]
pub struct InventoryExamineCamera;

#[derive(Component)]
pub struct InventoryExaminePreview {
    pub item_id: String,
}

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

pub fn handle_examine_item(
    mut commands: Commands,
    mut events: ResMut<Events<ExamineInventoryItemEvent>>,
    registry: Res<InventoryItemPreviewRegistry>,
    settings: Res<InventoryExamineSettings>,
    existing: Query<Entity, With<InventoryExaminePreview>>,
) {
    let Some(event) = events.drain().last() else { return };

    for entity in existing.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let Some(scene) = registry.previews.get(&event.item_id) else { return };
    let layer = RenderLayers::layer(settings.render_layer);

    commands.spawn((
        Name::new(format!("InventoryPreview {}", event.item_id)),
        SceneRoot(scene.clone()),
        InventoryExaminePreview {
            item_id: event.item_id.clone(),
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GlobalTransform::default(),
        layer,
    ));
}

pub fn rotate_examine_preview(
    time: Res<Time>,
    settings: Res<InventoryExamineSettings>,
    mut query: Query<&mut Transform, With<InventoryExaminePreview>>,
) {
    let angle = settings.rotate_speed * time.delta_secs();
    for mut transform in query.iter_mut() {
        transform.rotate_y(angle);
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
