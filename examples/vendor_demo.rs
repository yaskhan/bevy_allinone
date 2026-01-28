//! Vendor System Demo
//!
//! This example demonstrates the vendor/shop system for buying and selling items.
//! It shows how to create vendors, add items to their inventory, and handle
//! purchase/sale transactions with currency.

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
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn the player with currency
    let player = commands.spawn((
        Player,
        CharacterLevel { level: 5 },
        Currency {
            amount: 500.0,
            currency_type: CurrencyType::Gold,
        },
        Inventory::default(),
    )).id();

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

    let armor_item = InventoryItem {
        item_id: "armor_001".to_string(),
        name: "Leather Armor".to_string(),
        quantity: 1,
        max_stack: 1,
        weight: 5.0,
        item_type: ItemType::Equipment,
        icon_path: "".to_string(),
        value: 150.0,
        category: "Armor".to_string(),
        min_level: 5,
        info: "Basic leather armor".to_string(),
    };

    let arrow_item = InventoryItem {
        item_id: "arrow_001".to_string(),
        name: "Arrow".to_string(),
        quantity: 1,
        max_stack: 99,
        weight: 0.1,
        item_type: ItemType::Ammo,
        icon_path: "".to_string(),
        value: 2.0,
        category: "Ammo".to_string(),
        min_level: 1,
        info: "Standard arrow".to_string(),
    };

    // Create shop items
    let shop_sword = ShopItem::new(sword_item.clone(), 5, 100.0, 50.0);
    let shop_potion = ShopItem::new(potion_item.clone(), 20, 20.0, 10.0);
    let shop_armor = ShopItem::new(armor_item.clone(), 3, 150.0, 75.0);
    let shop_arrow = ShopItem::new(arrow_item.clone(), 100, 2.0, 1.0);
    
    // Mark arrow as infinite stock
    let mut shop_arrow_infinite = shop_arrow.clone();
    shop_arrow_infinite.infinite = true;

    // Create a blacksmith vendor
    let blacksmith = commands.spawn((
        Vendor {
            name: "Blacksmith".to_string(),
            buy_multiplier: 1.0,
            sell_multiplier: 0.5,
            infinite_stock: false,
            add_sold_items: true,
            min_level_to_buy: 1,
        },
        VendorInventory {
            items: vec![
                shop_sword,
                shop_potion,
                shop_armor,
                shop_arrow_infinite,
            ],
            categories: vec![],
        },
        Transform::from_xyz(5.0, 0.0, 5.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.7, 0.2))),
    )).id();

    // Create a general store vendor
    let merchant = commands.spawn((
        Vendor {
            name: "General Merchant".to_string(),
            buy_multiplier: 1.2,
            sell_multiplier: 0.4,
            infinite_stock: true,
            add_sold_items: false,
            min_level_to_buy: 1,
        },
        VendorInventory {
            items: vec![
                ShopItem::new(potion_item.clone(), 50, 25.0, 8.0),
            ],
            categories: vec![],
        },
        Transform::from_xyz(-5.0, 0.0, 5.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.7, 0.7))),
    )).id();

    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    info!("=== Vendor System Demo ===");
    info!("Player starts with 500 gold");
    info!("Blacksmith has: Iron Sword (100g), Health Potion (20g), Leather Armor (150g), Infinite Arrows (2g)");
    info!("Merchant has: Health Potion (25g)");
    info!("");
    info!("Press SPACE to buy items from Blacksmith");
    info!("Press ENTER to sell items to Merchant");
    info!("Press ESC to exit");
}

/// System to handle vendor interactions (automated demo)
fn handle_vendor_interactions(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Currency, &mut Inventory), With<Player>>,
    mut vendor_query: Query<(&Vendor, &mut VendorInventory), Without<Player>>,
    mut purchase_events: EventWriter<PurchaseItemEvent>,
    mut sell_events: EventWriter<SellItemEvent>,
    player_query_level: Query<&CharacterLevel, With<Player>>,
) {
    let Ok((mut player_currency, mut player_inventory)) = player_query.get_single_mut() else {
        return;
    };
    
    let Ok(player_level) = player_query_level.get_single() else {
        return;
    };

    // Get all vendor entities
    let vendors: Vec<Entity> = vendor_query.iter().map(|(e, _)| e).collect();

    // Buy from first vendor (Blacksmith) - Space key
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(vendor_entity) = vendors.first() {
            let Ok((vendor, vendor_inventory)) = vendor_query.get(*vendor_entity) else {
                return;
            };

            // Try to buy first available item
            for (index, shop_item) in vendor_inventory.items.iter().enumerate() {
                if shop_item.is_available(player_level.level) {
                    // Check if player can afford
                    if player_currency.amount >= shop_item.buy_price {
                        purchase_events.send(PurchaseItemEvent {
                            vendor_entity: *vendor_entity,
                            item_index: index,
                            amount: 1,
                            buyer_entity: player_query.iter().next().unwrap().0.id(),
                        });
                        info!("Buying {} for {} gold", shop_item.item.name, shop_item.buy_price);
                        break;
                    } else {
                        info!("Not enough money to buy {}", shop_item.item.name);
                    }
                }
            }
        }
    }

    // Sell to second vendor (Merchant) - Enter key
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if let Some(vendor_entity) = vendors.get(1) {
            // Check if player has any items to sell
            let mut item_to_sell: Option<InventoryItem> = None;
            
            for slot in player_inventory.items.iter() {
                if let Some(item) = slot {
                    if item.item_type != ItemType::KeyItem && item.item_type != ItemType::Quest {
                        item_to_sell = Some(item.clone());
                        break;
                    }
                }
            }

            if let Some(item) = item_to_sell {
                sell_events.send(SellItemEvent {
                    vendor_entity: *vendor_entity,
                    item: item.clone(),
                    amount: 1,
                    seller_entity: player_query.iter().next().unwrap().0.id(),
                });
                info!("Selling {} to merchant", item.name);
            } else {
                info!("No items to sell!");
            }
        }
    }
}

/// System to display vendor and player status
fn display_vendor_status(
    player_query: Query<(&Currency, &Inventory), With<Player>>,
    vendor_query: Query<&Vendor, Without<Player>>,
    time: Res<Time>,
) {
    // Display every 2 seconds
    if time.elapsed_seconds() % 2.0 < 0.016 {
        if let Ok((currency, inventory)) = player_query.get_single() {
            info!("=== Player Status ===");
            info!("Gold: {:.1}", currency.amount);
            
            let item_count: usize = inventory.items.iter().filter(|i| i.is_some()).count();
            info!("Inventory: {} items", item_count);
            
            for (i, slot) in inventory.items.iter().enumerate() {
                if let Some(item) = slot {
                    info!("  Slot {}: {} x{}", i, item.name, item.quantity);
                }
            }
        }

        info!("=== Vendors ===");
        for vendor in vendor_query.iter() {
            info!("{} - Buy Mult: {:.1}, Sell Mult: {:.1}", 
                vendor.name, vendor.buy_multiplier, vendor.sell_multiplier);
        }
        info!("");
    }
}
