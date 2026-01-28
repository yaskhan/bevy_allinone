//! # Stats System Demo
//!
//! This example demonstrates the stats system functionality.
//!
//! ## Features Demonstrated
//!
//! - Core attributes (Strength, Agility, Intelligence, Constitution, Charisma)
//! - Derived stats (Health, Stamina, Mana, Attack Power, Defense, etc.)
//! - Stat modifiers (Buffs and Debuffs)
//! - Stat templates (Save/Load)
//! - Real-time stat updates
//!
//! ## Controls
//!
//! - **1-5**: Increase core attributes (Strength, Agility, Intelligence, Constitution, Charisma)
//! - **Q**: Apply Strength buff (+10 Attack Power, 10s)
//! - **W**: Apply Agility buff (+20% Movement Speed, 10s)
//! - **E**: Apply Intelligence buff (+10 Mana, 10s)
//! - **R**: Apply Constitution buff (+20 Health, 10s)
//! - **T**: Apply Charisma buff (+10 Persuasion, 10s)
//! - **A**: Apply Strength debuff (-5 Attack Power, 10s)
//! - **S**: Apply Agility debuff (-20% Movement Speed, 10s)
//! - **D**: Apply Constitution debuff (-20 Health, 10s)
//! - **F**: Heal (restore 25 Health)
//! - **G**: Use Stamina (consume 25 Stamina)
//! - **H**: Use Mana (consume 25 Mana)
//! - **Z**: Save stats to template
//! - **X**: Load stats from template
//! - **C**: Reset all stats to default
//! - **V**: Toggle stats system active/inactive

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
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    ));

    // Spawn light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 5.0, 2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Spawn ground
    commands.spawn((
        PbrBundle {
            mesh: asset_server.add(Mesh::from(Cuboid::new(20.0, 0.5, 20.0))),
            material: asset_server.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -0.25, 0.0),
            ..Default::default()
        },
    ));

    // Spawn player entity with stats system
    commands.spawn((
        StatsSystem::new(),
        DemoStats,
        Name::new("Player"),
    ));

    // Spawn UI text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Stats System Demo\n\n",
                TextStyle {
                    font: asset_server.add(asset_server.load("fonts/FiraSans-Bold.ttf")),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Core Attributes:\n",
                TextStyle {
                    font: asset_server.add(asset_server.load("fonts/FiraSans-Bold.ttf")),
                    font_size: 20.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "Strength: 10 | Agility: 10 | Intelligence: 10\nConstitution: 10 | Charisma: 10\n\n",
                TextStyle {
                    font: asset_server.add(asset_server.load("fonts/FiraSans-Regular.ttf")),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Derived Stats:\n",
                TextStyle {
                    font: asset_server.add(asset_server.load("fonts/FiraSans-Bold.ttf")),
                    font_size: 20.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "Health: 100/100 | Stamina: 100/100 | Mana: 100/100\nAttack: 15 | Defense: 8 | Crit: 5%\nSpeed: 1.0x | Attack Speed: 1.0x\nMagic Res: 0.2 | Stealth: 0.1 | Persuasion: 0.2\n\n",
                TextStyle {
                    font: asset_server.add(asset_server.load("fonts/FiraSans-Regular.ttf")),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Active Modifiers:\n",
                TextStyle {
                    font: asset_server.add(asset_server.load("fonts/FiraSans-Bold.ttf")),
                    font_size: 20.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "None\n\n",
                TextStyle {
                    font: asset_server.add(asset_server.load("fonts/FiraSans-Regular.ttf")),
                    font_size: 16.0,
                    color: Color::rgba(1.0, 1.0, 1.0, 0.8),
                },
            ),
            TextSection::new(
                "Controls:\n",
                TextStyle {
                    font: asset_server.add(asset_server.load("fonts/FiraSans-Bold.ttf")),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "1-5: +1 to attributes\nQ-W-E-R-T: Buffs\nA-S-D: Debuffs\nF: Heal | G: Stamina | H: Mana\nZ: Save | X: Load | C: Reset | V: Toggle\n",
                TextStyle {
                    font: asset_server.add(asset_server.load("fonts/FiraSans-Regular.ttf")),
                    font_size: 16.0,
                    color: Color::rgba(1.0, 1.0, 1.0, 0.8),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..Default::default()
        }),
        Name::new("UI Text"),
    ));
}

/// Handle keyboard input for stats
fn handle_stats_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut stats_query: Query<&mut StatsSystem, With<DemoStats>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let Ok(mut stats) = stats_query.get_single_mut() else {
        return;
    };

    // Increase core attributes
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        stats.increase_core_attribute(CoreAttribute::Strength, 1.0);
        info!("Strength increased to: {}", stats.get_core_attribute(CoreAttribute::Strength).unwrap());
    }

    if keyboard_input.just_pressed(KeyCode::Digit2) {
        stats.increase_core_attribute(CoreAttribute::Agility, 1.0);
        info!("Agility increased to: {}", stats.get_core_attribute(CoreAttribute::Agility).unwrap());
    }

    if keyboard_input.just_pressed(KeyCode::Digit3) {
        stats.increase_core_attribute(CoreAttribute::Intelligence, 1.0);
        info!("Intelligence increased to: {}", stats.get_core_attribute(CoreAttribute::Intelligence).unwrap());
    }

    if keyboard_input.just_pressed(KeyCode::Digit4) {
        stats.increase_core_attribute(CoreAttribute::Constitution, 1.0);
        info!("Constitution increased to: {}", stats.get_core_attribute(CoreAttribute::Constitution).unwrap());
    }

    if keyboard_input.just_pressed(KeyCode::Digit5) {
        stats.increase_core_attribute(CoreAttribute::Charisma, 1.0);
        info!("Charisma increased to: {}", stats.get_core_attribute(CoreAttribute::Charisma).unwrap());
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
        info!("Applied Strength Buff (+10 Attack Power, 10s)");
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        let modifier = StatModifier::percentage_buff(
            "Agility Buff",
            DerivedStat::MovementSpeed,
            20.0,
            10.0,
        );
        stats.add_modifier(modifier);
        info!("Applied Agility Buff (+20% Movement Speed, 10s)");
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        let modifier = StatModifier::temporary_buff(
            "Intelligence Buff",
            DerivedStat::MaxMana,
            10.0,
            10.0,
        );
        stats.add_modifier(modifier);
        info!("Applied Intelligence Buff (+10 Max Mana, 10s)");
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        let modifier = StatModifier::temporary_buff(
            "Constitution Buff",
            DerivedStat::MaxHealth,
            20.0,
            10.0,
        );
        stats.add_modifier(modifier);
        info!("Applied Constitution Buff (+20 Max Health, 10s)");
    }

    if keyboard_input.just_pressed(KeyCode::KeyT) {
        let modifier = StatModifier::temporary_buff(
            "Charisma Buff",
            DerivedStat::Persuasion,
            10.0,
            10.0,
        );
        stats.add_modifier(modifier);
        info!("Applied Charisma Buff (+10 Persuasion, 10s)");
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
        info!("Applied Strength Debuff (-5 Attack Power, 10s)");
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        let modifier = StatModifier::percentage_debuff(
            "Agility Debuff",
            DerivedStat::MovementSpeed,
            20.0,
            10.0,
        );
        stats.add_modifier(modifier);
        info!("Applied Agility Debuff (-20% Movement Speed, 10s)");
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        let modifier = StatModifier::temporary_debuff(
            "Constitution Debuff",
            DerivedStat::MaxHealth,
            20.0,
            10.0,
        );
        stats.add_modifier(modifier);
        info!("Applied Constitution Debuff (-20 Max Health, 10s)");
    }

    // Use stats
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        let heal_amount = 25.0;
        stats.increase_derived_stat(DerivedStat::CurrentHealth, heal_amount);
        info!("Healed {} HP", heal_amount);
    }

    if keyboard_input.just_pressed(KeyCode::KeyG) {
        let stamina_cost = 25.0;
        stats.use_stat(DerivedStat::CurrentStamina, stamina_cost);
        info!("Used {} Stamina", stamina_cost);
    }

    if keyboard_input.just_pressed(KeyCode::KeyH) {
        let mana_cost = 25.0;
        stats.use_stat(DerivedStat::CurrentMana, mana_cost);
        info!("Used {} Mana", mana_cost);
    }

    // Save/Load template
    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        let mut template = StatTemplate {
            id: 1,
            name: String::from("Demo Template"),
            stat_entries: Vec::new(),
        };
        stats.save_to_template(&mut template);
        info!("Saved stats to template: {}", template.name);
    }

    if keyboard_input.just_pressed(KeyCode::KeyX) {
        let template = StatTemplate {
            id: 1,
            name: String::from("Demo Template"),
            stat_entries: vec![
                StatTemplateEntry {
                    name: String::from("Strength"),
                    value: 15.0,
                    bool_state: false,
                },
                StatTemplateEntry {
                    name: String::from("Agility"),
                    value: 12.0,
                    bool_state: false,
                },
                StatTemplateEntry {
                    name: String::from("Intelligence"),
                    value: 14.0,
                    bool_state: false,
                },
                StatTemplateEntry {
                    name: String::from("Constitution"),
                    value: 16.0,
                    bool_state: false,
                },
                StatTemplateEntry {
                    name: String::from("Charisma"),
                    value: 11.0,
                    bool_state: false,
                },
            ],
        };
        stats.load_from_template(&template);
        info!("Loaded stats from template: {}", template.name);
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
        info!("Stats system: {}", if new_state { "Active" } else { "Inactive" });
    }
}

/// Update the stats display
fn update_stats_display(
    stats_query: Query<&StatsSystem, With<DemoStats>>,
    mut text_query: Query<&mut Text, Without<DemoStats>>,
) {
    let Ok(stats) = stats_query.get_single() else {
        return;
    };

    let mut text = text_query.single_mut();

    // Core attributes
    let strength = stats.get_core_attribute(CoreAttribute::Strength).copied().unwrap_or(0.0);
    let agility = stats.get_core_attribute(CoreAttribute::Agility).copied().unwrap_or(0.0);
    let intelligence = stats.get_core_attribute(CoreAttribute::Intelligence).copied().unwrap_or(0.0);
    let constitution = stats.get_core_attribute(CoreAttribute::Constitution).copied().unwrap_or(0.0);
    let charisma = stats.get_core_attribute(CoreAttribute::Charisma).copied().unwrap_or(0.0);

    text.sections[2].value = format!(
        "Strength: {:.0} | Agility: {:.0} | Intelligence: {:.0}\nConstitution: {:.0} | Charisma: {:.0}\n\n",
        strength, agility, intelligence, constitution, charisma
    );

    // Derived stats
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

    text.sections[4].value = format!(
        "Health: {:.0}/{:.0} | Stamina: {:.0}/{:.0} | Mana: {:.0}/{:.0}\nAttack: {:.1} | Defense: {:.1} | Crit: {:.1}%\nSpeed: {:.2}x | Attack Speed: {:.2}x\nMagic Res: {:.2} | Stealth: {:.2} | Persuasion: {:.2}\n\n",
        current_health, max_health,
        current_stamina, max_stamina,
        current_mana, max_mana,
        attack_power, defense, critical_chance * 100.0,
        movement_speed, attack_speed,
        magic_resistance, stealth, persuasion
    );

    // Modifiers
    let modifiers = stats.get_modifiers();
    if modifiers.is_empty() {
        text.sections[6].value = String::from("None\n\n");
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
                "- {} ({}): {}{}{}\n",
                modifier.name,
                type_str,
                amount_str,
                duration_str,
                if modifier.is_percentage { " of " } else { "" }
            ));
        }
        text.sections[6].value = modifier_text + "\n";
    }
}

/// Update modifier timers (visual feedback)
fn update_modifier_timers(
    time: Res<Time>,
    mut stats_query: Query<&mut StatsSystem, With<DemoStats>>,
) {
    let Ok(mut stats) = stats_query.get_single_mut() else {
        return;
    };

    // Update and apply modifiers
    stats.update_modifiers(time.delta_seconds());
    stats.apply_modifiers();
}
