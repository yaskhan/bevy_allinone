use bevy::prelude::*;

use super::inventory_bank_manager::InventoryBankManager;

/// Root UI node for bank inventory.
#[derive(Component)]
pub struct InventoryBankUIRoot;

/// Displays bank inventory status.
///
/// GKC reference: `inventoryBankUISystem.cs`
pub fn setup_inventory_bank_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Px(360.0),
                height: Val::Px(520.0),
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.08, 0.08, 0.85)),
            InventoryBankUIRoot,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("BANK"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn update_inventory_bank_ui(
    bank_query: Query<&InventoryBankManager>,
    mut ui_query: Query<&mut Visibility, With<InventoryBankUIRoot>>,
) {
    let is_open = bank_query.iter().any(|bank| bank.is_open);
    for mut visibility in ui_query.iter_mut() {
        *visibility = if is_open { Visibility::Visible } else { Visibility::Hidden };
    }
}
