use bevy::prelude::*;
use super::types::*;
use crate::character::Player;
use crate::ai::{AiController, AiBehaviorState};
use crate::camera::types::CameraController;
use crate::game_manager::types::{GameState, CursorState};
use crate::input::InputState;
use crate::inventory::InventoryUIRoot;

pub fn update_play_time(
    time: Res<Time>,
    mut settings: ResMut<GameManagerSettings>,
    state: Res<State<GameState>>,
) {
    if *state == GameState::Playing {
        settings.play_time += time.delta_secs();
    }
}

pub fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

pub fn switch_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_manager: ResMut<PlayerManager>,
    mut switch_queue: ResMut<SwitchPlayerQueue>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        if !player_manager.players.is_empty() {
            let next_index = (player_manager.current_player_index + 1) % player_manager.players.len();
            switch_queue.0.push(SwitchPlayerEvent {
                target_index: Some(next_index),
                target_entity: None,
            });
        }
    }
}

pub fn handle_switch_player(
    mut commands: Commands,
    mut player_manager: ResMut<PlayerManager>,
    mut switch_queue: ResMut<SwitchPlayerQueue>,
    mut camera_query: Query<&mut CameraController>,
    player_query: Query<Entity, With<Player>>,
) {
    for event in switch_queue.0.drain(..) {
        if player_manager.players.is_empty() {
            continue;
        }

        let target_entity = if let Some(entity) = event.target_entity {
            Some(entity)
        } else if let Some(index) = event.target_index {
            player_manager.players.get(index).copied()
        } else {
            None
        };

        let Some(new_player) = target_entity else { continue };

        let current_player = player_manager
            .get_current_player()
            .or_else(|| player_query.iter().next());

        if current_player == Some(new_player) {
            continue;
        }

        if let Some(old_player) = current_player {
            commands.entity(old_player).remove::<Player>();
            if player_manager.enable_ai_on_inactive {
                commands.entity(old_player).insert(AiController {
                    state: AiBehaviorState::Follow,
                    target: Some(new_player),
                    ..default()
                });
            }
        }

        commands.entity(new_player).insert(Player);
        commands.entity(new_player).remove::<AiController>();

        if let Some(index) = player_manager.players.iter().position(|p| *p == new_player) {
            player_manager.current_player_index = index;
        }

        for mut camera in camera_query.iter_mut() {
            camera.follow_target = Some(new_player);
        }

        info!("Switched active player to {:?}", new_player);
    }
}

pub fn handle_cursor_state(
    mut windows: Query<&mut Window>,
    settings: Res<CursorManagerSettings>,
    state: Res<State<GameState>>,
    inventory_query: Query<&Visibility, With<InventoryUIRoot>>,
    cursor_state: Res<CursorState>,
) {
    let inventory_open = inventory_query
        .iter()
        .any(|visibility| *visibility != Visibility::Hidden);

    let paused = *state == GameState::Paused;
    let show_cursor = (paused && settings.show_cursor_when_paused)
        || (inventory_open && settings.show_cursor_when_inventory_open);

    for mut window in windows.iter_mut() {
        let mut visible = if show_cursor {
            true
        } else if settings.lock_in_game {
            false
        } else {
            true
        };

        let mut grab_mode = if show_cursor {
            CursorGrabMode::None
        } else if settings.lock_in_game {
            CursorGrabMode::Locked
        } else {
            CursorGrabMode::None
        };

        if let Some(override_visible) = cursor_state.visible_override {
            visible = override_visible;
        }
        if let Some(override_grab) = cursor_state.grab_mode_override {
            grab_mode = override_grab;
        }
        // TODO: Fix Cursor API for Bevy 0.18
        /*
        if let Some(icon) = cursor_state.icon_override {
            window.cursor.icon = icon;
        }

        window.cursor.visible = visible;
        window.cursor.grab_mode = grab_mode;
        */
    }
}

pub fn handle_pause_input_state(
    state: Res<State<GameState>>,
    mut input_state: ResMut<InputState>,
) {
    if *state == GameState::Paused {
        input_state.set_input_enabled(false);
    } else {
        input_state.set_input_enabled(true);
    }
}

/// System to spawn a prefab from the registry
pub fn spawn_prefab(
    name: &str,
    position: Vec3,
    registry: &PrefabRegistry,
    commands: &mut Commands,
) {
    if let Some(scene_handle) = registry.prefabs.get(name) {
        commands.spawn((
            SceneRoot(scene_handle.clone()),
            Transform::from_translation(position),
        ));
    } else {
        warn!("Prefab '{}' not found in registry", name);
    }
}
