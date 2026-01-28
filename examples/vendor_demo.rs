//! Vendor System Demo
//!
//! This example demonstrates the vendor/shop system for buying and selling items.

use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_vendor_interactions,
            display_vendor_status,
        ))
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Spawn a ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.3, 0.5, 0.3)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn the player with currency and stats (for level)
    commands.spawn((
        Player,
        StatsSystem::new(),
        Currency {
            amount: 500.0,
            currency_type: CurrencyType::Gold,
        },
        Inventory::default(),
    ));

    // Create some items for the vendor
    let sword_item = InventoryItem {
        item_id: "sword_001".to_string(),
        name: "Iron Sword".to_string(),
        quantity: 1,
        max_stack: 1,
        weight: 2.5,
        item_type: ItemType::Weapon,
        icon_path: "".to_string(),
        value: 100.0,
        category: "Weapons".to_string(),
        min_level: 3,
        info: "A sturdy iron sword".to_string(),
    };

    let potion_item = InventoryItem {
        item_id: "potion_001".to_string(),
        name: "Health Potion".to_string(),
        quantity: 1,
        max_stack: 10,
        weight: 0.5,
        item_type: ItemType::Consumable,
        icon_path: "".to_string(),
        value: 20.0,
        category: "Consumables".to_string(),
        min_level: 1,
        info: "Restores 50 HP".to_string(),
    };

    // Create shop items
    let shop_sword = ShopItem::new(sword_item.clone(), 5, 100.0, 50.0);
    let shop_potion = ShopItem::new(potion_item.clone(), 20, 20.0, 10.0);
    
    // Create a blacksmith vendor
    commands.spawn((
        Vendor {
            name: "Blacksmith".to_string(),
            buy_multiplier: 1.0,
            sell_multiplier: 0.5,
            infinite_stock: false,
            add_sold_items: true,
            min_level_to_buy: 1,
            currency_type: CurrencyType::Gold,
        },
        VendorInventory {
            items: vec![
                shop_sword,
                shop_potion,
            ],
            categories: vec![],
        },
        Transform::from_xyz(5.0, 0.0, 5.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.7, 0.7, 0.2)))),
    ));

    info!("=== Vendor System Demo ===");
    info!("Player starts with 500 gold");
    info!("Press SPACE to buy items from Blacksmith");
    info!("Press ENTER to sell items to Merchant (if available)");
}

fn handle_vendor_interactions(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &mut Currency, &mut Inventory, &StatsSystem), With<Player>>,
    mut vendor_query: Query<(Entity, &Vendor, &mut VendorInventory), Without<Player>>,
    mut purchase_events: ResMut<PurchaseItemEventQueue>,
    mut sell_events: ResMut<SellItemEventQueue>,
) {
    let (player_entity, mut player_currency, mut player_inventory, stats) = 
        if let Some(p) = player_query.iter_mut().next() { p } else { return; };
    
    let player_level = stats.get_derived_stat(DerivedStat::Level).copied().unwrap_or(1.0) as u32;

    // Buy from first vendor - Space key
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some((vendor_entity, vendor, mut vendor_inventory)) = vendor_query.iter_mut().next() {
            // Try to buy first available item
            for (index, shop_item) in vendor_inventory.items.iter().enumerate() {
                if shop_item.is_available(player_level) {
                    if player_currency.amount >= shop_item.buy_price {
                        purchase_events.0.push(PurchaseItemEvent {
                            vendor_entity,
                            item_index: index,
                            amount: 1,
                            buyer_entity: player_entity,
                        });
                        info!("Buying {} for {} gold", shop_item.item.name, shop_item.buy_price);
                        break;
                    }
                }
            }
        }
    }
}

fn display_vendor_status(
    player_query: Query<(&Currency, &Inventory, &StatsSystem), With<Player>>,
    time: Res<Time>,
) {
    if time.elapsed_secs() % 5.0 < 0.016 {
        if let Some((currency, inventory, stats)) = player_query.iter().next() {
            info!("Gold: {:.1}, Level: {:.0}", currency.amount, stats.get_derived_stat(DerivedStat::Level).unwrap_or(&1.0));
        }
    }
}
