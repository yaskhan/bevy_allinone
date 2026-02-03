use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Component, Debug, Clone)]
pub struct DraggableUi {
    pub save_key: String,
    pub default_position: Vec2,
}

#[derive(Resource, Debug, Clone)]
pub struct UiEditSettings {
    pub editing: bool,
    pub save_path: String,
    pub reset_requested: bool,
}

impl Default for UiEditSettings {
    fn default() -> Self {
        Self {
            editing: false,
            save_path: "ui_layout.json".to_string(),
            reset_requested: false,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct UiEditState {
    pub active: Option<Entity>,
    pub last_cursor: Option<Vec2>,
    pub applied_layout: bool,
    pub was_editing: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UiPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Debug, Default, Serialize, Deserialize)]
pub struct UiLayoutStore {
    pub positions: HashMap<String, UiPosition>,
}

pub fn load_ui_layout(mut store: ResMut<UiLayoutStore>, settings: Res<UiEditSettings>) {
    let path = Path::new(&settings.save_path);
    if !path.exists() {
        return;
    }

    if let Ok(data) = fs::read_to_string(path) {
        if let Ok(parsed) = serde_json::from_str::<UiLayoutStore>(&data) {
            *store = parsed;
        }
    }
}

pub fn apply_ui_layout(
    mut state: ResMut<UiEditState>,
    store: Res<UiLayoutStore>,
    mut query: Query<(&mut Style, &DraggableUi)>,
) {
    if state.applied_layout {
        return;
    }

    for (mut style, draggable) in query.iter_mut() {
        if let Some(pos) = store.positions.get(&draggable.save_key) {
            style.left = Val::Px(pos.x);
            style.top = Val::Px(pos.y);
        } else {
            style.left = Val::Px(draggable.default_position.x);
            style.top = Val::Px(draggable.default_position.y);
        }
    }

    state.applied_layout = true;
}

pub fn handle_ui_drag_start(
    settings: Res<UiEditSettings>,
    mut state: ResMut<UiEditState>,
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<(Entity, &Interaction), With<DraggableUi>>,
) {
    if !settings.editing {
        state.active = None;
        state.last_cursor = None;
        return;
    }

    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.get_single() else { return };
    let Some(cursor) = window.cursor_position() else { return };

    for (entity, interaction) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            state.active = Some(entity);
            state.last_cursor = Some(cursor);
            break;
        }
    }
}

pub fn handle_ui_drag_update(
    settings: Res<UiEditSettings>,
    mut state: ResMut<UiEditState>,
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Style, With<DraggableUi>>,
) {
    if !settings.editing {
        return;
    }

    let Some(active) = state.active else { return };
    if !mouse_buttons.pressed(MouseButton::Left) {
        state.active = None;
        state.last_cursor = None;
        return;
    }

    let Ok(window) = windows.get_single() else { return };
    let Some(cursor) = window.cursor_position() else { return };
    let Some(last) = state.last_cursor else {
        state.last_cursor = Some(cursor);
        return;
    };

    let delta = cursor - last;
    if let Ok(mut style) = query.get_mut(active) {
        let current_left = read_px(style.left);
        let current_top = read_px(style.top);
        style.left = Val::Px(current_left + delta.x);
        style.top = Val::Px(current_top + delta.y);
    }

    state.last_cursor = Some(cursor);
}

pub fn handle_ui_drag_end(
    settings: Res<UiEditSettings>,
    mut state: ResMut<UiEditState>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    if !settings.editing {
        state.active = None;
        state.last_cursor = None;
        return;
    }

    if mouse_buttons.just_released(MouseButton::Left) {
        state.active = None;
        state.last_cursor = None;
    }
}

pub fn save_ui_layout(
    settings: Res<UiEditSettings>,
    mut state: ResMut<UiEditState>,
    query: Query<(&Style, &DraggableUi)>,
    mut store: ResMut<UiLayoutStore>,
) {
    if state.was_editing && !settings.editing {
        store.positions.clear();
        for (style, draggable) in query.iter() {
            let pos = UiPosition {
                x: read_px(style.left),
                y: read_px(style.top),
            };
            store.positions.insert(draggable.save_key.clone(), pos);
        }

        if let Ok(serialized) = serde_json::to_string_pretty(&*store) {
            let _ = fs::write(&settings.save_path, serialized);
        }
    }

    state.was_editing = settings.editing;
}

pub fn reset_ui_layout(
    mut settings: ResMut<UiEditSettings>,
    mut store: ResMut<UiLayoutStore>,
    mut query: Query<(&mut Style, &DraggableUi)>,
) {
    if !settings.reset_requested {
        return;
    }

    store.positions.clear();
    for (mut style, draggable) in query.iter_mut() {
        style.left = Val::Px(draggable.default_position.x);
        style.top = Val::Px(draggable.default_position.y);
    }

    if let Ok(serialized) = serde_json::to_string_pretty(&*store) {
        let _ = fs::write(&settings.save_path, serialized);
    }

    settings.reset_requested = false;
}

fn read_px(val: Val) -> f32 {
    match val {
        Val::Px(px) => px,
        _ => 0.0,
    }
}
