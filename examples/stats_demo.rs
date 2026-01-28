//! # Stats System Demo
//!
//! This example demonstrates the stats system functionality.

use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_stats_input,
            update_stats_display,
            update_modifier_timers,
        ))
        .run();
}

/// Marker component for demo stats
#[derive(Component)]
struct DemoStats;

/// Setup the demo scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.5, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.3, 0.5, 0.3)))),
        Transform::from_xyz(0.0, -0.25, 0.0),
    ));

    // Spawn player entity with stats system
    commands.spawn((
        StatsSystem::new(),
        DemoStats,
        Name::new("Player"),
    ));

    // Spawn UI text
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Stats System Demo\n\n"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));

        parent.spawn((
            Text::new("Core Attributes:\n"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
        ));

        parent.spawn((
            Text::new("Strength: 10 | Agility: 10 | Intelligence: 10\nConstitution: 10 | Charisma: 10\n\n"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
            DemoStatsTextCore,
        ));

        parent.spawn((
            Text::new("Derived Stats:\n"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
        ));

        parent.spawn((
            Text::new("Health: 100/100 | Stamina: 100/100 | Mana: 100/100\nAttack: 15 | Defense: 8 | Crit: 5%\nSpeed: 1.0x | Attack Speed: 1.0x\nMagic Res: 0.2 | Stealth: 0.1 | Persuasion: 0.2\n\n"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
            DemoStatsTextDerived,
        ));

        parent.spawn((
            Text::new("Active Modifiers:\n"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
        ));

        parent.spawn((
            Text::new("None\n\n"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
            DemoStatsTextModifiers,
        ));

        parent.spawn((
            Text::new("Controls:\n1-5: +1 to attributes\nQ-W-E-R-T: Buffs\nA-S-D: Debuffs\nF: Heal | G: Stamina | H: Mana\nZ: Save | X: Load | C: Reset | V: Toggle\n"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
        ));
    });
}

#[derive(Component)]
struct DemoStatsTextCore;

#[derive(Component)]
struct DemoStatsTextDerived;

#[derive(Component)]
struct DemoStatsTextModifiers;

/// Handle keyboard input for stats
fn handle_stats_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut stats_query: Query<&mut StatsSystem, With<DemoStats>>,
) {
    let mut stats = if let Some(s) = stats_query.iter_mut().next() { s } else { return; };

    // Increase core attributes
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        stats.increase_core_attribute(CoreAttribute::Strength, 1.0);
    }

    if keyboard_input.just_pressed(KeyCode::Digit2) {
        stats.increase_core_attribute(CoreAttribute::Agility, 1.0);
    }

    if keyboard_input.just_pressed(KeyCode::Digit3) {
        stats.increase_core_attribute(CoreAttribute::Intelligence, 1.0);
    }

    if keyboard_input.just_pressed(KeyCode::Digit4) {
        stats.increase_core_attribute(CoreAttribute::Constitution, 1.0);
    }

    if keyboard_input.just_pressed(KeyCode::Digit5) {
        stats.increase_core_attribute(CoreAttribute::Charisma, 1.0);
    }

    // Apply buffs
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        let modifier = StatModifier::temporary_buff(
            "Strength Buff",
            DerivedStat::AttackPower,
            10.0,
            10.0,
        );
        stats.add_modifier(modifier);
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        let modifier = StatModifier::percentage_buff(
            "Agility Buff",
            DerivedStat::MovementSpeed,
            20.0,
            10.0,
        );
        stats.add_modifier(modifier);
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        let modifier = StatModifier::temporary_buff(
            "Intelligence Buff",
            DerivedStat::MaxMana,
            10.0,
            10.0,
        );
        stats.add_modifier(modifier);
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        let modifier = StatModifier::temporary_buff(
            "Constitution Buff",
            DerivedStat::MaxHealth,
            20.0,
            10.0,
        );
        stats.add_modifier(modifier);
    }

    if keyboard_input.just_pressed(KeyCode::KeyT) {
        let modifier = StatModifier::temporary_buff(
            "Charisma Buff",
            DerivedStat::Persuasion,
            10.0,
            10.0,
        );
        stats.add_modifier(modifier);
    }

    // Apply debuffs
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        let modifier = StatModifier::temporary_debuff(
            "Strength Debuff",
            DerivedStat::AttackPower,
            5.0,
            10.0,
        );
        stats.add_modifier(modifier);
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        let modifier = StatModifier::percentage_debuff(
            "Agility Debuff",
            DerivedStat::MovementSpeed,
            20.0,
            10.0,
        );
        stats.add_modifier(modifier);
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        let modifier = StatModifier::temporary_debuff(
            "Constitution Debuff",
            DerivedStat::MaxHealth,
            20.0,
            10.0,
        );
        stats.add_modifier(modifier);
    }

    // Use stats
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        stats.increase_derived_stat(DerivedStat::CurrentHealth, 25.0);
    }

    if keyboard_input.just_pressed(KeyCode::KeyG) {
        stats.use_stat(DerivedStat::CurrentStamina, 25.0);
    }

    if keyboard_input.just_pressed(KeyCode::KeyH) {
        stats.use_stat(DerivedStat::CurrentMana, 25.0);
    }

    // Save/Load template
    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        let mut template = StatTemplate {
            id: 1,
            name: String::from("Demo Template"),
            stat_entries: Vec::new(),
        };
        stats.save_to_template(&mut template);
        info!("Saved stats to template");
    }

    if keyboard_input.just_pressed(KeyCode::KeyX) {
        let template = StatTemplate {
            id: 1,
            name: String::from("Demo Template"),
            stat_entries: vec![
                StatTemplateEntry { name: String::from("Strength"), value: 15.0, bool_state: false },
                StatTemplateEntry { name: String::from("Agility"), value: 12.0, bool_state: false },
                StatTemplateEntry { name: String::from("Intelligence"), value: 14.0, bool_state: false },
                StatTemplateEntry { name: String::from("Constitution"), value: 16.0, bool_state: false },
                StatTemplateEntry { name: String::from("Charisma"), value: 11.0, bool_state: false },
            ],
        };
        stats.load_from_template(&template);
        info!("Loaded stats from template");
    }

    // Reset stats
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        *stats = StatsSystem::new();
        info!("Reset all stats to default");
    }

    // Toggle active
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        let new_state = !stats.active;
        stats.set_active(new_state);
    }
}

/// Update the stats display
fn update_stats_display(
    stats_query: Query<&StatsSystem, With<DemoStats>>,
    mut core_text_query: Query<&mut Text, (With<DemoStatsTextCore>, Without<DemoStatsTextDerived>, Without<DemoStatsTextModifiers>)>,
    mut derived_text_query: Query<&mut Text, (With<DemoStatsTextDerived>, Without<DemoStatsTextCore>, Without<DemoStatsTextModifiers>)>,
    mut modifiers_text_query: Query<&mut Text, (With<DemoStatsTextModifiers>, Without<DemoStatsTextCore>, Without<DemoStatsTextDerived>)>,
) {
    let stats = if let Some(s) = stats_query.iter().next() { s } else { return; };

    if let Some(mut text) = core_text_query.iter_mut().next() {
        let strength = stats.get_core_attribute(CoreAttribute::Strength).copied().unwrap_or(0.0);
        let agility = stats.get_core_attribute(CoreAttribute::Agility).copied().unwrap_or(0.0);
        let intelligence = stats.get_core_attribute(CoreAttribute::Intelligence).copied().unwrap_or(0.0);
        let constitution = stats.get_core_attribute(CoreAttribute::Constitution).copied().unwrap_or(0.0);
        let charisma = stats.get_core_attribute(CoreAttribute::Charisma).copied().unwrap_or(0.0);

        text.0 = format!(
            "Strength: {:.0} | Agility: {:.0} | Intelligence: {:.0}\nConstitution: {:.0} | Charisma: {:.0}\n\n",
            strength, agility, intelligence, constitution, charisma
        );
    }

    if let Some(mut text) = derived_text_query.iter_mut().next() {
        let max_health = stats.get_derived_stat(DerivedStat::MaxHealth).copied().unwrap_or(0.0);
        let current_health = stats.get_derived_stat(DerivedStat::CurrentHealth).copied().unwrap_or(0.0);
        let max_stamina = stats.get_derived_stat(DerivedStat::MaxStamina).copied().unwrap_or(0.0);
        let current_stamina = stats.get_derived_stat(DerivedStat::CurrentStamina).copied().unwrap_or(0.0);
        let max_mana = stats.get_derived_stat(DerivedStat::MaxMana).copied().unwrap_or(0.0);
        let current_mana = stats.get_derived_stat(DerivedStat::CurrentMana).copied().unwrap_or(0.0);
        let attack_power = stats.get_derived_stat(DerivedStat::AttackPower).copied().unwrap_or(0.0);
        let defense = stats.get_derived_stat(DerivedStat::Defense).copied().unwrap_or(0.0);
        let critical_chance = stats.get_derived_stat(DerivedStat::CriticalChance).copied().unwrap_or(0.0);
        let movement_speed = stats.get_derived_stat(DerivedStat::MovementSpeed).copied().unwrap_or(0.0);
        let attack_speed = stats.get_derived_stat(DerivedStat::AttackSpeed).copied().unwrap_or(0.0);
        let magic_resistance = stats.get_derived_stat(DerivedStat::MagicResistance).copied().unwrap_or(0.0);
        let stealth = stats.get_derived_stat(DerivedStat::Stealth).copied().unwrap_or(0.0);
        let persuasion = stats.get_derived_stat(DerivedStat::Persuasion).copied().unwrap_or(0.0);

        text.0 = format!(
            "Health: {:.0}/{:.0} | Stamina: {:.0}/{:.0} | Mana: {:.0}/{:.0}\nAttack: {:.1} | Defense: {:.1} | Crit: {:.1}%\nSpeed: {:.2}x | Attack Speed: {:.2}x\nMagic Res: {:.2} | Stealth: {:.2} | Persuasion: {:.2}\n\n",
            current_health, max_health,
            current_stamina, max_stamina,
            current_mana, max_mana,
            attack_power, defense, critical_chance * 100.0,
            movement_speed, attack_speed,
            magic_resistance, stealth, persuasion
        );
    }

    if let Some(mut text) = modifiers_text_query.iter_mut().next() {
        let modifiers = stats.get_modifiers();
        if modifiers.is_empty() {
            text.0 = String::from("None\n\n");
        } else {
            let mut modifier_text = String::new();
            for modifier in modifiers {
                let type_str = match modifier.modifier_type {
                    ModifierType::Buff => "Buff",
                    ModifierType::Debuff => "Debuff",
                };
                let amount_str = if modifier.is_percentage {
                    format!("{:+.0}%", modifier.amount)
                } else {
                    format!("{:+.1}", modifier.amount)
                };
                let duration_str = if modifier.duration > 0.0 {
                    format!(" ({:.1}s)", modifier.time_remaining)
                } else {
                    String::from(" (Permanent)")
                };
                modifier_text.push_str(&format!(
                    "- {} ({}): {}{}\n",
                    modifier.name, type_str, amount_str, duration_str
                ));
            }
            text.0 = modifier_text + "\n";
        }
    }
}

/// Update modifier timers
fn update_modifier_timers(
    time: Res<Time>,
    mut stats_query: Query<&mut StatsSystem, With<DemoStats>>,
) {
    if let Some(mut stats) = stats_query.iter_mut().next() {
        stats.update_modifiers(time.delta().as_secs_f32());
        stats.apply_modifiers();
    }
}
