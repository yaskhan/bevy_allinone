use bevy::prelude::*;
use super::types::{InputAction, InputBinding, BufferedAction, InputContext};
use super::resources::{InputMap, InputBuffer, InputConfig, RebindState, InputContextStack, InputContextRules, ActionState, ActionValue};
use super::components::{InputState, PlayerInputSettings, InputDevice};
use crate::game_manager::types::GameState;
use crate::inventory::InventoryUIRoot;
use crate::character::{CharacterMovementState, Player};
use bevy::input::axis::Axis;
use bevy::input::gamepad::{Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType};

/// Update input state from devices based on current InputMap
pub fn update_input_state(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    input_map: Res<InputMap>,
    mut input_state: ResMut<super::components::InputState>, // Using component as resource here since we derive Resource on it
    mut input_buffer: ResMut<InputBuffer>,
    context_stack: Res<InputContextStack>,
    context_rules: Res<InputContextRules>,
) {
    if !input_state.enabled {
        return;
    }

    let current_context = context_stack.current();
    let is_blocked = |action: InputAction| -> bool {
        context_rules
            .blocked_actions
            .get(&current_context)
            .map(|set| set.contains(&action))
            .unwrap_or(false)
    };

    let check_action = |action: InputAction| -> bool {
        if is_blocked(action) {
            return false;
        }
        if let Some(bindings) = input_map.bindings.get(&action) {
            bindings.iter().any(|binding| match binding {
                InputBinding::Key(code) => keyboard.pressed(code.clone()),
                InputBinding::Mouse(button) => mouse_buttons.pressed(button.clone()),
            })
        } else {
            false
        }
    };

    let check_action_just_pressed = |action: InputAction| -> bool {
        if is_blocked(action) {
            return false;
        }
        if let Some(bindings) = input_map.bindings.get(&action) {
            bindings.iter().any(|binding| match binding {
                InputBinding::Key(code) => keyboard.just_pressed(code.clone()),
                InputBinding::Mouse(button) => mouse_buttons.just_pressed(button.clone()),
            })
        } else {
            false
        }
    };

    let check_action_just_released = |action: InputAction| -> bool {
        if is_blocked(action) {
            return false;
        }
        if let Some(bindings) = input_map.bindings.get(&action) {
            bindings.iter().any(|binding| match binding {
                InputBinding::Key(code) => keyboard.just_released(code.clone()),
                InputBinding::Mouse(button) => mouse_buttons.just_released(button.clone()),
            })
        } else {
            false
        }
    };

    // Buffer certain actions
    let actions_to_buffer = [
        InputAction::Jump,
        InputAction::Interact,
        InputAction::LockOn,
        InputAction::AbilityUse,
    ];

    for action in actions_to_buffer {
        if check_action_just_pressed(action) {
            input_buffer.actions.push(BufferedAction {
                action,
                timestamp: time.elapsed_secs(),
            });
        }
    }

    // Movement
    let mut movement = Vec2::ZERO;
    if check_action(InputAction::MoveForward) { movement.y += 1.0; }
    if check_action(InputAction::MoveBackward) { movement.y -= 1.0; }
    if check_action(InputAction::MoveLeft) { movement.x -= 1.0; }
    if check_action(InputAction::MoveRight) { movement.x += 1.0; }
    input_state.movement = movement.normalize_or_zero();
    
    // Continuous Input
    input_state.crouch_pressed = check_action(InputAction::Crouch);
    input_state.sprint_pressed = check_action(InputAction::Sprint);
    input_state.aim_pressed = check_action(InputAction::Aim);
    input_state.lean_left = check_action(InputAction::LeanLeft);
    input_state.lean_right = check_action(InputAction::LeanRight);
    input_state.block_pressed = check_action(InputAction::Block);
    input_state.fire_pressed = check_action(InputAction::Fire);

    // Just Pressed Input
    input_state.jump_pressed = check_action_just_pressed(InputAction::Jump);
    input_state.interact_pressed = check_action_just_pressed(InputAction::Interact);
    input_state.lock_on_pressed = check_action_just_pressed(InputAction::LockOn);
    input_state.attack_pressed = check_action_just_pressed(InputAction::Attack);
    input_state.switch_camera_mode_pressed = check_action_just_pressed(InputAction::SwitchCameraMode);
    input_state.fire_just_pressed = check_action_just_pressed(InputAction::Fire);
    input_state.reload_pressed = check_action_just_pressed(InputAction::Reload);
    input_state.reset_camera_pressed = check_action_just_pressed(InputAction::ResetCamera);
    input_state.next_weapon_pressed = check_action_just_pressed(InputAction::NextWeapon);
    input_state.prev_weapon_pressed = check_action_just_pressed(InputAction::PrevWeapon);
    input_state.toggle_inventory_pressed = check_action_just_pressed(InputAction::ToggleInventory);
    input_state.side_switch_pressed = check_action_just_pressed(InputAction::SideSwitch);
    
    // Stealth/Advanced
    input_state.hide_pressed = check_action_just_pressed(InputAction::Hide);
    input_state.peek_pressed = check_action_just_pressed(InputAction::Peek);
    input_state.corner_lean_pressed = check_action_just_pressed(InputAction::CornerLean);
    input_state.zoom_in_pressed = check_action_just_pressed(InputAction::ZoomIn);
    input_state.zoom_out_pressed = check_action_just_pressed(InputAction::ZoomOut);
    input_state.ability_use_pressed = check_action_just_pressed(InputAction::AbilityUse);
    input_state.ability_use_released = check_action_just_released(InputAction::AbilityUse);
    input_state.ability_use_held = check_action(InputAction::AbilityUse);

    // Weapon Selection
    input_state.select_weapon = None;
    if check_action_just_pressed(InputAction::SelectWeapon1) { input_state.select_weapon = Some(0); }
    else if check_action_just_pressed(InputAction::SelectWeapon2) { input_state.select_weapon = Some(1); }
    else if check_action_just_pressed(InputAction::SelectWeapon3) { input_state.select_weapon = Some(2); }
    else if check_action_just_pressed(InputAction::SelectWeapon4) { input_state.select_weapon = Some(3); }
    else if check_action_just_pressed(InputAction::SelectWeapon5) { input_state.select_weapon = Some(4); }
    else if check_action_just_pressed(InputAction::SelectWeapon6) { input_state.select_weapon = Some(5); }
    else if check_action_just_pressed(InputAction::SelectWeapon7) { input_state.select_weapon = Some(6); }
    else if check_action_just_pressed(InputAction::SelectWeapon8) { input_state.select_weapon = Some(7); }
    else if check_action_just_pressed(InputAction::SelectWeapon9) { input_state.select_weapon = Some(8); }
    else if check_action_just_pressed(InputAction::SelectWeapon0) { input_state.select_weapon = Some(9); }

    // Ability Selection
    input_state.select_ability = None;
    if check_action_just_pressed(InputAction::AbilitySelect1) { input_state.select_ability = Some(0); }
    else if check_action_just_pressed(InputAction::AbilitySelect2) { input_state.select_ability = Some(1); }
    else if check_action_just_pressed(InputAction::AbilitySelect3) { input_state.select_ability = Some(2); }
    else if check_action_just_pressed(InputAction::AbilitySelect4) { input_state.select_ability = Some(3); }
    else if check_action_just_pressed(InputAction::AbilitySelect5) { input_state.select_ability = Some(4); }
    else if check_action_just_pressed(InputAction::AbilitySelect6) { input_state.select_ability = Some(5); }
    else if check_action_just_pressed(InputAction::AbilitySelect7) { input_state.select_ability = Some(6); }
    else if check_action_just_pressed(InputAction::AbilitySelect8) { input_state.select_ability = Some(7); }

    // Look (handled by mouse events typically, but for this system we'll need to re-enable it if needed)
    // input_state.look = ...
}

/// System to handle runtime remapping of actions
pub fn handle_rebinding(
    mut rebind_state: ResMut<RebindState>,
    mut input_map: ResMut<InputMap>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let Some(action) = rebind_state.action else { return };

    let mut new_binding = None;
    if let Some(key) = keyboard.get_just_pressed().next() {
        new_binding = Some(InputBinding::Key(key.clone()));
    } else if let Some(button) = mouse_buttons.get_just_pressed().next() {
        new_binding = Some(InputBinding::Mouse(button.clone()));
    }

    if let Some(binding) = new_binding {
        input_map.bindings.insert(action, vec![binding]);
        rebind_state.action = None;
    }
}

/// Remove expired inputs from the buffer
pub fn cleanup_input_buffer(
    time: Res<Time>,
    config: Res<InputConfig>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    let now = time.elapsed_secs();
    input_buffer.actions.retain(|ba| now - ba.timestamp <= config.buffer_ttl);
}

/// Process movement input (Stub)
pub fn process_movement_input(_input: Res<InputState>) {}

/// Process action input (Stub)
pub fn process_action_input(_input: Res<InputState>) {}

/// System to sync global input state to the player entity's component
pub fn player_input_sync_system(
    input_state: Res<InputState>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    gamepad_axes: Res<Axis<GamepadAxis>>,
    mut query: Query<(&mut InputState, Option<&PlayerInputSettings>), (With<crate::character::Player>, Without<crate::ai::AiController>)>,
) {
    for (mut player_input, settings) in query.iter_mut() {
        let settings = settings.cloned().unwrap_or_default();
        if !settings.enabled {
            player_input.set_input_enabled(false);
            continue;
        }

        let mut next_state = match settings.device {
            InputDevice::KeyboardMouse => input_state.clone(),
            InputDevice::Gamepad { id } => {
                let gamepad = Gamepad::new(id);
                build_gamepad_input_state(gamepad, &gamepad_buttons, &gamepad_axes)
            }
        };

        next_state.apply_locks(&settings.locks);
        *player_input = next_state;
    }
}

pub fn update_action_state(
    input_state: Res<InputState>,
    context_stack: Res<InputContextStack>,
    context_rules: Res<InputContextRules>,
    mut action_state: ResMut<ActionState>,
) {
    let current_context = context_stack.current();
    let is_blocked = |action: InputAction| -> bool {
        context_rules
            .blocked_actions
            .get(&current_context)
            .map(|set| set.contains(&action))
            .unwrap_or(false)
    };

    for (action, value) in action_state.actions.iter_mut() {
        if is_blocked(*action) {
            *value = ActionValue::default();
            continue;
        }

        *value = read_action_value(*action, &input_state);
    }
}

fn read_action_value(action: InputAction, input_state: &InputState) -> ActionValue {
    match action {
        InputAction::MoveForward => ActionValue {
            pressed: input_state.movement.y > 0.1,
            value: input_state.movement.y.max(0.0),
            ..default()
        },
        InputAction::MoveBackward => ActionValue {
            pressed: input_state.movement.y < -0.1,
            value: (-input_state.movement.y).max(0.0),
            ..default()
        },
        InputAction::MoveLeft => ActionValue {
            pressed: input_state.movement.x < -0.1,
            value: (-input_state.movement.x).max(0.0),
            ..default()
        },
        InputAction::MoveRight => ActionValue {
            pressed: input_state.movement.x > 0.1,
            value: input_state.movement.x.max(0.0),
            ..default()
        },
        InputAction::Jump => ActionValue { pressed: input_state.jump_pressed, just_pressed: input_state.jump_pressed, ..default() },
        InputAction::Sprint => ActionValue { pressed: input_state.sprint_pressed, ..default() },
        InputAction::Crouch => ActionValue { pressed: input_state.crouch_pressed, ..default() },
        InputAction::Interact => ActionValue { pressed: input_state.interact_pressed, just_pressed: input_state.interact_pressed, ..default() },
        InputAction::Aim => ActionValue { pressed: input_state.aim_pressed, ..default() },
        InputAction::LeanLeft => ActionValue { pressed: input_state.lean_left, ..default() },
        InputAction::LeanRight => ActionValue { pressed: input_state.lean_right, ..default() },
        InputAction::Attack => ActionValue { pressed: input_state.attack_pressed, just_pressed: input_state.attack_pressed, ..default() },
        InputAction::Block => ActionValue { pressed: input_state.block_pressed, ..default() },
        InputAction::SwitchCameraMode => ActionValue { pressed: input_state.switch_camera_mode_pressed, just_pressed: input_state.switch_camera_mode_pressed, ..default() },
        InputAction::Fire => ActionValue { pressed: input_state.fire_pressed, just_pressed: input_state.fire_just_pressed, ..default() },
        InputAction::Reload => ActionValue { pressed: input_state.reload_pressed, just_pressed: input_state.reload_pressed, ..default() },
        InputAction::NextWeapon => ActionValue { pressed: input_state.next_weapon_pressed, just_pressed: input_state.next_weapon_pressed, ..default() },
        InputAction::PrevWeapon => ActionValue { pressed: input_state.prev_weapon_pressed, just_pressed: input_state.prev_weapon_pressed, ..default() },
        InputAction::ToggleInventory => ActionValue { pressed: input_state.toggle_inventory_pressed, just_pressed: input_state.toggle_inventory_pressed, ..default() },
        InputAction::Hide => ActionValue { pressed: input_state.hide_pressed, just_pressed: input_state.hide_pressed, ..default() },
        InputAction::Peek => ActionValue { pressed: input_state.peek_pressed, just_pressed: input_state.peek_pressed, ..default() },
        InputAction::CornerLean => ActionValue { pressed: input_state.corner_lean_pressed, just_pressed: input_state.corner_lean_pressed, ..default() },
        InputAction::ResetCamera => ActionValue { pressed: input_state.reset_camera_pressed, just_pressed: input_state.reset_camera_pressed, ..default() },
        InputAction::LockOn => ActionValue { pressed: input_state.lock_on_pressed, just_pressed: input_state.lock_on_pressed, ..default() },
        InputAction::ZoomIn => ActionValue { pressed: input_state.zoom_in_pressed, just_pressed: input_state.zoom_in_pressed, ..default() },
        InputAction::ZoomOut => ActionValue { pressed: input_state.zoom_out_pressed, just_pressed: input_state.zoom_out_pressed, ..default() },
        InputAction::SideSwitch => ActionValue { pressed: input_state.side_switch_pressed, just_pressed: input_state.side_switch_pressed, ..default() },
        InputAction::AbilityUse => ActionValue {
            pressed: input_state.ability_use_held,
            just_pressed: input_state.ability_use_pressed,
            just_released: input_state.ability_use_released,
            ..default()
        },
        InputAction::SelectWeapon1
        | InputAction::SelectWeapon2
        | InputAction::SelectWeapon3
        | InputAction::SelectWeapon4
        | InputAction::SelectWeapon5
        | InputAction::SelectWeapon6
        | InputAction::SelectWeapon7
        | InputAction::SelectWeapon8
        | InputAction::SelectWeapon9
        | InputAction::SelectWeapon0 => ActionValue {
            pressed: input_state.select_weapon.is_some(),
            just_pressed: input_state.select_weapon.is_some(),
            ..default()
        },
        InputAction::AbilitySelect1
        | InputAction::AbilitySelect2
        | InputAction::AbilitySelect3
        | InputAction::AbilitySelect4
        | InputAction::AbilitySelect5
        | InputAction::AbilitySelect6
        | InputAction::AbilitySelect7
        | InputAction::AbilitySelect8 => ActionValue {
            pressed: input_state.select_ability.is_some(),
            just_pressed: input_state.select_ability.is_some(),
            ..default()
        },
    }
}

fn build_gamepad_input_state(
    gamepad: Gamepad,
    buttons: &ButtonInput<GamepadButton>,
    axes: &Axis<GamepadAxis>,
) -> InputState {
    let mut state = InputState::default();

    let axis = |axis_type: GamepadAxisType| -> f32 {
        axes.get(GamepadAxis::new(gamepad, axis_type)).unwrap_or(0.0)
    };

    let button = |button_type: GamepadButtonType| -> bool {
        buttons.pressed(GamepadButton::new(gamepad, button_type))
    };

    let button_just = |button_type: GamepadButtonType| -> bool {
        buttons.just_pressed(GamepadButton::new(gamepad, button_type))
    };

    let button_released = |button_type: GamepadButtonType| -> bool {
        buttons.just_released(GamepadButton::new(gamepad, button_type))
    };

    let movement = Vec2::new(axis(GamepadAxisType::LeftStickX), axis(GamepadAxisType::LeftStickY));
    state.movement = movement.normalize_or_zero();
    state.look = Vec2::new(axis(GamepadAxisType::RightStickX), axis(GamepadAxisType::RightStickY));

    state.jump_pressed = button_just(GamepadButtonType::South);
    state.interact_pressed = button_just(GamepadButtonType::West);
    state.crouch_pressed = button(GamepadButtonType::East);
    state.sprint_pressed = button(GamepadButtonType::LeftStick);
    state.aim_pressed = button(GamepadButtonType::LeftTrigger2);
    state.attack_pressed = button_just(GamepadButtonType::RightShoulder);
    state.fire_pressed = button(GamepadButtonType::RightTrigger2);
    state.fire_just_pressed = button_just(GamepadButtonType::RightTrigger2);
    state.reload_pressed = button_just(GamepadButtonType::North);
    state.block_pressed = button(GamepadButtonType::LeftShoulder);

    state.switch_camera_mode_pressed = button_just(GamepadButtonType::Select);
    state.toggle_inventory_pressed = button_just(GamepadButtonType::Start);
    state.reset_camera_pressed = button_just(GamepadButtonType::DPadUp);

    state.ability_use_pressed = button_just(GamepadButtonType::RightShoulder);
    state.ability_use_released = button_released(GamepadButtonType::RightShoulder);
    state.ability_use_held = button(GamepadButtonType::RightShoulder);

    // TODO: map weapon/ability selection to D-Pad or face buttons based on user config.

    state
}

pub fn update_input_context(
    state: Res<State<GameState>>,
    inventory_query: Query<&Visibility, With<InventoryUIRoot>>,
    player_query: Query<&CharacterMovementState, With<Player>>,
    mut context_stack: ResMut<InputContextStack>,
) {
    let inventory_open = inventory_query
        .iter()
        .any(|visibility| *visibility != Visibility::Hidden);

    let in_vehicle = player_query.iter().any(|movement| movement.is_in_vehicle);

    let desired = if *state == GameState::Paused || inventory_open {
        InputContext::Menu
    } else if in_vehicle {
        InputContext::Vehicle
    } else {
        InputContext::Gameplay
    };

    context_stack.stack.clear();
    context_stack.stack.push(desired);
}
