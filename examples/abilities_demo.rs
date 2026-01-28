//! # Abilities System Demo
//!
//! This example demonstrates the abilities system functionality.
//!
//! ## Controls
//!
//! - **1-9**: Select ability by number
//! - **Space**: Activate current ability (Press Down)
//! - **Shift**: Hold current ability (Press Hold)
//! - **Enter**: Release current ability (Press Up)
//! - **Tab**: Toggle ability selection wheel (UI)
//! - **E**: Enable/disable ability
//! - **R**: Deactivate ability
//! - **T**: Toggle abilities mode
//!
//! ## Abilities Included
//!
//! 1. **Teleport** - Instantly move to a new location
//! 2. **Dash** - Quick forward dash
//! 3. **Shield** - Activate protective shield
//! 4. **Heal** - Restore health
//! 5. **Invisibility** - Become invisible
//! 6. **Flight** - Enable flight mode
//! 7. **Super Speed** - Increase movement speed
//! 8. **Time Slow** - Slow down time

use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_ability_input,
            update_ability_display,
            update_abilities_demo,
        ))
        .run();
}

/// Marker component for demo abilities
#[derive(Component)]
struct DemoAbility;

/// Component to track ability activation state
#[derive(Component, Default)]
struct AbilityActivation {
    pub active: bool,
    pub timer: f32,
}

/// Setup the demo scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn ground
    commands.spawn((
        Mesh3d(asset_server.add(Mesh::from(Cuboid::new(20.0, 0.5, 20.0)))),
        MeshMaterial3d(asset_server.add(StandardMaterial::from(Color::srgb(0.3, 0.5, 0.3)))),
        Transform::from_xyz(0.0, -0.25, 0.0),
        Visibility::default(),
        InheritedVisibility::default(),
    ));

    // Spawn player entity with abilities system
    commands.spawn((
        PlayerAbilitiesSystem::new(),
        Name::new("Player"),
    ));

    // Spawn abilities
    spawn_abilities(&mut commands, &asset_server);

    // Spawn UI text
    commands.spawn((
        Text::new("Abilities Demo\n\nCurrent Ability: None\nEnergy: 100/100\n\nControls:\n1-9: Select ability\nSpace: Activate (Press Down)\nShift: Hold (Press Hold)\nEnter: Release (Press Up)\nE: Enable/Disable\nR: Deactivate\nT: Toggle Mode\n"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
             position_type: PositionType::Absolute,
             top: Val::Px(10.0),
             left: Val::Px(10.0),
             ..default()
        },
        Name::new("UI Text"),
    ));
}

/// Spawn all demo abilities
fn spawn_abilities(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let abilities = vec![
        (
            "Teleport",
            AbilityInfo {
                name: String::from("Teleport"),
                enabled: true,
                input_types: vec![AbilityInputType::PressDown],
                use_cooldown: true,
                cooldown_duration: 5.0,
                use_energy: true,
                energy_amount: 20.0,
                use_energy_on_press_down: true,
                use_energy_once_on_press_down: true,
                ..Default::default()
            },
        ),
        (
            "Dash",
            AbilityInfo {
                name: String::from("Dash"),
                enabled: true,
                input_types: vec![AbilityInputType::PressDown],
                use_cooldown: true,
                cooldown_duration: 2.0,
                use_time_limit: true,
                time_limit: 0.3,
                use_energy: true,
                energy_amount: 10.0,
                use_energy_on_press_down: true,
                use_energy_once_on_press_down: true,
                ..Default::default()
            },
        ),
        (
            "Shield",
            AbilityInfo {
                name: String::from("Shield"),
                enabled: true,
                input_types: vec![AbilityInputType::PressDown, AbilityInputType::PressUp],
                use_time_limit: true,
                time_limit: 5.0,
                use_energy: true,
                energy_amount: 15.0,
                use_energy_on_press_down: true,
                use_energy_once_on_press_down: true,
                use_energy_on_press_up: true,
                use_energy_once_on_press_up: true,
                ..Default::default()
            },
        ),
        (
            "Heal",
            AbilityInfo {
                name: String::from("Heal"),
                enabled: true,
                input_types: vec![AbilityInputType::PressDown],
                use_cooldown: true,
                cooldown_duration: 10.0,
                use_energy: true,
                energy_amount: 30.0,
                use_energy_on_press_down: true,
                use_energy_once_on_press_down: true,
                ..Default::default()
            },
        ),
        (
            "Invisibility",
            AbilityInfo {
                name: String::from("Invisibility"),
                enabled: true,
                input_types: vec![AbilityInputType::PressDown, AbilityInputType::PressUp],
                use_time_limit: true,
                time_limit: 10.0,
                use_energy: true,
                energy_amount: 25.0,
                use_energy_on_press_down: true,
                use_energy_once_on_press_down: true,
                use_energy_on_press_up: true,
                use_energy_once_on_press_up: true,
                ..Default::default()
            },
        ),
        (
            "Flight",
            AbilityInfo {
                name: String::from("Flight"),
                enabled: true,
                input_types: vec![AbilityInputType::PressDown, AbilityInputType::PressHold, AbilityInputType::PressUp],
                use_time_limit: true,
                time_limit: 15.0,
                use_energy: true,
                energy_amount: 20.0,
                ..Default::default()
            },
        ),
        (
            "Super Speed",
            AbilityInfo {
                name: String::from("Super Speed"),
                enabled: true,
                input_types: vec![AbilityInputType::PressDown, AbilityInputType::PressUp],
                use_time_limit: true,
                time_limit: 8.0,
                use_energy: true,
                energy_amount: 15.0,
                use_energy_on_press_down: true,
                use_energy_once_on_press_down: true,
                use_energy_on_press_up: true,
                use_energy_once_on_press_up: true,
                ..Default::default()
            },
        ),
        (
            "Time Slow",
            AbilityInfo {
                name: String::from("Time Slow"),
                enabled: true,
                input_types: vec![AbilityInputType::PressDown],
                use_cooldown: true,
                cooldown_duration: 15.0,
                use_time_limit: true,
                time_limit: 5.0,
                use_energy: true,
                energy_amount: 40.0,
                use_energy_on_press_down: true,
                use_energy_once_on_press_down: true,
                ..Default::default()
            },
        ),
    ];

    for (name, ability) in abilities {
        commands.spawn((
            ability,
            DemoAbility,
            Name::new(name),
        ));
    }
}

/// Handle keyboard input for abilities
fn handle_ability_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut abilities_system: Query<&mut PlayerAbilitiesSystem>,
    mut abilities: Query<&mut AbilityInfo, With<DemoAbility>>,
    mut ability_activation: Query<&mut AbilityActivation, With<DemoAbility>>,
    _time: Res<Time>,
) {
    let Some(mut system) = abilities_system.iter_mut().next() else {
        return;
    };

    // Get current ability
    let current_ability = abilities.iter().find(|a| a.is_current);

    // Select ability by number
    // Fixed: KeyCode is an enum, we can't iterate via integer cast easily with new KeyCode
    // We'll just check specific keys for the demo
    let keys = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
        (KeyCode::Digit5, 4),
        (KeyCode::Digit6, 5),
        (KeyCode::Digit7, 6),
        (KeyCode::Digit8, 7),
        (KeyCode::Digit9, 8),
    ];

    for (key, index) in keys {
        if keyboard_input.just_pressed(key) {
             let mut ability_names: Vec<String> = abilities.iter().map(|a| a.name.clone()).collect();
             // Sort to ensure consistent order
             // Note: in a real system we'd use IDs/Indices better
             // Here we rely on spawn order which might vary unless we sort
             // But we didn't sort ability_names in previous code... oops.
             // Let's just use the order we find them for now or sort them.
             // The original code sorted names.
             ability_names.sort();
             
             if index < ability_names.len() {
                 system.set_current_ability_by_name(&ability_names[index], &mut abilities);
                 info!("Selected ability: {}", ability_names[index]);
             }
        }
    }

    // Activate ability (Press Down)
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
            system.input_press_down_use_current_ability(&mut ability, true);
            
            if let Some(mut activation) = ability_activation.iter_mut().find(|a| a.active) {
                activation.active = ability.active_from_press_down;
                activation.timer = ability.time_limit;
            }
            
            info!("Activated ability: {}", ability.name);
        }
    }

    // Hold ability (Press Hold)
    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
            system.input_press_hold_use_current_ability(&mut ability, true);
        }
    }

    // Release ability (Press Up)
    if keyboard_input.just_released(KeyCode::ShiftLeft) || keyboard_input.just_released(KeyCode::ShiftRight) {
        if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
            system.input_press_up_use_current_ability(&mut ability, true);
            
            if let Some(mut activation) = ability_activation.iter_mut().find(|a| a.active) {
                activation.active = ability.active_from_press_up;
                activation.timer = ability.time_limit;
            }
            
            info!("Released ability: {}", ability.name);
        }
    }

    // Enable/Disable ability
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        let toggle_data = {
            let mut data = None;
            for ability in abilities.iter() {
                if ability.is_current {
                    data = Some((ability.name.clone(), !ability.enabled));
                    break;
                }
            }
            data
        };
        if let Some((name, new_state)) = toggle_data {
            system.enable_or_disable_all_abilities(new_state, &mut abilities);
            info!("{} ability: {}", if new_state { "Enabled" } else { "Disabled" }, name);
        }
    }

    // Deactivate ability
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        let deactivate_name = {
            let mut name = None;
            for ability in abilities.iter() {
                if ability.is_current {
                    name = Some(ability.name.clone());
                    break;
                }
            }
            name
        };
        if let Some(name) = deactivate_name {
            system.deactivate_ability_by_name(&name, &mut abilities);
            info!("Deactivated ability: {}", name);
        }
    }

    // Toggle abilities mode
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        let new_state = !system.is_abilities_mode_active();
        system.set_abilities_mode_active(new_state);
        info!("Abilities mode: {}", if new_state { "Active" } else { "Inactive" });
    }
}

/// Update the ability display
fn update_ability_display(
    abilities_system: Query<&PlayerAbilitiesSystem>,
    abilities: Query<&AbilityInfo, With<DemoAbility>>,
    mut text_query: Query<&mut Text, Without<DemoAbility>>,
) {
    let Some(system) = abilities_system.iter().next() else {
        return;
    };

    let current_ability = abilities.iter().find(|a| a.is_current);
    let current_ability_name = current_ability.map(|a| a.name.clone()).unwrap_or_else(|| String::from("None"));
    
    let energy = system.current_energy;
    let max_energy = system.max_energy;
    
    let status = if let Some(ability) = current_ability {
        if ability.cooldown_in_process {
            format!(" (Cooldown: {:.1}s)", ability.cooldown_timer)
        } else if ability.time_limit_in_process {
            format!(" (Time Limit: {:.1}s)", ability.time_limit_timer)
        } else if ability.active {
            format!(" (Active)")
        } else if ability.enabled {
            format!(" (Ready)")
        } else {
            format!(" (Disabled)")
        }
    } else {
        String::new()
    };

    let mode = if system.is_abilities_mode_active() {
        "Active"
    } else {
        "Inactive"
    };

    if let Some(mut text) = text_query.iter_mut().next() {
    text.0 = format!("Abilities Demo\n\nCurrent Ability: {}{}\nEnergy: {:.0}/{:.0} | Mode: {}\n\nControls:\n1-9: Select ability\nSpace: Activate (Press Down)\nShift: Hold (Press Hold)\nEnter: Release (Press Up)\nE: Enable/Disable\nR: Deactivate\nT: Toggle Mode\n", current_ability_name, status, energy, max_energy, mode);
    }
}

/// Update ability demo effects
fn update_abilities_demo(
    mut abilities: Query<&mut AbilityInfo, With<DemoAbility>>,
    mut ability_activation: Query<&mut AbilityActivation, With<DemoAbility>>,
    time: Res<Time>,
) {
    // Update activation timers
    for mut activation in ability_activation.iter_mut() {
        if activation.active {
            activation.timer -= time.delta_secs();
            if activation.timer <= 0.0 {
                activation.active = false;
                activation.timer = 0.0;
            }
        }
    }
}
