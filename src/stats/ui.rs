use bevy::prelude::*;
use super::stats_system::StatsSystem;
use super::types::DerivedStat;

// Markers for UI elements
#[derive(Component)]
pub struct StatsHudRoot;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct StaminaBar;

#[derive(Component)]
pub struct ManaBar;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct StaminaText;

#[derive(Component)]
pub struct ManaText;

/// System to spawn the Stats HUD
pub fn setup_stats_hud(mut commands: Commands) {
    let bar_width = 200.0;
    let bar_height = 20.0;
    let margin = 5.0;

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        StatsHudRoot,
    )).with_children(|parent| {
        // Health Bar Container
        parent.spawn(Node {
            width: Val::Px(bar_width),
            height: Val::Px(bar_height),
            margin: UiRect::bottom(Val::Px(margin)),
            ..default()
        }).with_children(|container| {
            // Background
            container.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            }).insert(BackgroundColor(Color::srgba(0.2, 0.0, 0.0, 0.5)));

            // Fill
            container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.8, 0.1, 0.1, 1.0)),
                HealthBar,
            ));

            // Text
            container.spawn((
                Text::new("HP"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(5.0),
                    top: Val::Px(2.0),
                    ..default()
                },
                HealthText,
            ));
        });

        // Stamina Bar Container
        parent.spawn(Node {
            width: Val::Px(bar_width),
            height: Val::Px(bar_height),
            margin: UiRect::bottom(Val::Px(margin)),
            ..default()
        }).with_children(|container| {
            // Background
            container.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            }).insert(BackgroundColor(Color::srgba(0.0, 0.2, 0.0, 0.5)));

            // Fill
            container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.8, 0.1, 1.0)),
                StaminaBar,
            ));

             // Text
             container.spawn((
                Text::new("STM"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(5.0),
                    top: Val::Px(2.0),
                    ..default()
                },
                StaminaText,
            ));
        });

        // Mana Bar Container (Optional, visible if MaxMana > 0)
        parent.spawn(Node {
            width: Val::Px(bar_width),
            height: Val::Px(bar_height),
            margin: UiRect::bottom(Val::Px(margin)),
            ..default()
        }).with_children(|container| {
             // Background
             container.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            }).insert(BackgroundColor(Color::srgba(0.0, 0.0, 0.2, 0.5)));

            // Fill
            container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.1, 0.9, 1.0)),
                ManaBar,
            ));

             // Text
             container.spawn((
                Text::new("MP"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(5.0),
                    top: Val::Px(2.0),
                    ..default()
                },
                ManaText,
            ));
        });
    });
}

/// System to update the Stats HUD
pub fn update_stats_hud(
    stats_query: Query<&StatsSystem>, // Needs entity identifying player? For now assuming single player
    mut health_bar_query: Query<&mut Node, (With<HealthBar>, Without<StaminaBar>, Without<ManaBar>)>,
    mut stamina_bar_query: Query<&mut Node, (With<StaminaBar>, Without<HealthBar>, Without<ManaBar>)>,
    mut mana_bar_query: Query<&mut Node, (With<ManaBar>, Without<HealthBar>, Without<StaminaBar>)>,
    mut health_text_query: Query<&mut Text, (With<HealthText>, Without<StaminaText>, Without<ManaText>)>,
    mut stamina_text_query: Query<&mut Text, (With<StaminaText>, Without<HealthText>, Without<ManaText>)>,
    mut mana_text_query: Query<&mut Text, (With<ManaText>, Without<HealthText>, Without<StaminaText>)>,
) {
    let Some(stats) = stats_query.iter().next() else { return }; // Get first stats system (likely player)

    // Update Health
    if let (Some(current), Some(max)) = (
        stats.get_derived_stat(DerivedStat::CurrentHealth),
        stats.get_derived_stat(DerivedStat::MaxHealth)
    ) {
        if let Some(mut bar) = health_bar_query.iter_mut().next() {
            let percent = (current / max).clamp(0.0, 1.0) * 100.0;
            bar.width = Val::Percent(percent);
        }
        if let Some(mut text) = health_text_query.iter_mut().next() {
            text.0 = format!("{:.0}/{:.0}", current, max);
        }
    }

    // Update Stamina
    if let (Some(current), Some(max)) = (
        stats.get_derived_stat(DerivedStat::CurrentStamina),
        stats.get_derived_stat(DerivedStat::MaxStamina)
    ) {
        if let Some(mut bar) = stamina_bar_query.iter_mut().next() {
             let percent = (current / max).clamp(0.0, 1.0) * 100.0;
             bar.width = Val::Percent(percent);
        }
        if let Some(mut text) = stamina_text_query.iter_mut().next() {
            text.0 = format!("{:.0}/{:.0}", current, max);
        }
    }

    // Update Mana
    if let (Some(current), Some(max)) = (
        stats.get_derived_stat(DerivedStat::CurrentMana),
        stats.get_derived_stat(DerivedStat::MaxMana)
    ) {
        if let Some(mut bar) = mana_bar_query.iter_mut().next() {
             let percent = (current / max).clamp(0.0, 1.0) * 100.0;
             bar.width = Val::Percent(percent);
        }
        if let Some(mut text) = mana_text_query.iter_mut().next() {
             text.0 = format!("{:.0}/{:.0}", current, max);
        }
    }
}
