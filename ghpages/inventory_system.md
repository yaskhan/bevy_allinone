# Inventory and Economy System

The **Inventory and Economy System** provides a robust framework for managing items, equipment, currency, and trading in Bevy All-in-One. This unified documentation covers the interconnected subsystems that drive the game's economy and player progression through item acquisition.

## Documentation Contents

### Core Sections
- **Overview** - System introduction and high-level architecture
- **Core Concepts** - Stacking, Weight, Categories, and Currency
- **Component Reference** - Detailed documentation of components:
  - `Inventory` - The main container logic
  - `InventoryItem` - Data structure for individual items
  - `Equipment` - Player gear slots
  - `Currency` - Money management
  - `Vendor` - Shop configuration
  - `VendorInventory` - Shop stock management
- **Systems & Logic** - Deep dive into internal mechanics:
  - Item Pickup & Stacking
  - Inventory UI Construction
  - Vendor Transaction Flow (Buying/Selling)
  - Currency Operations

### Advanced Features
- **Smart Stacking** - Automatic merging of stackable items
- **Weight Management** - Encumbrance logic
- **Dynamic Vendor Pricing** - Multipliers for buy/sell values
- **Infinite Stock** - Support for limitless shop supplies
- **Item Categories** - Automatic organization for shops
- **Event-Driven Architecture** - Decoupled interaction via Bevy Events

### Practical Guides
- **Usage Patterns** - Creating items, setting up vendors, and handling money
- **Integration** - Connecting with Interaction and Input systems
- **Troubleshooting** - Common issues with events and UI

---

## Overview

The Inventory system is designed to be modular and entity-component based. Any entity in the world can have an `Inventory`, not just the player. This allows for chests, loot boxes, and even NPCs to carry items.

The system is split into three main modules:
1.  **Inventory**: Manages storage slots, item data, and UI.
2.  **Currency**: Manages wealth, supporting multiple currency types (Gold, Silver, etc.).
3.  **Vendor**: Manages trading interfaces, pricing logic, and stock interaction.

These modules communicate primarily through **Events**, ensuring that logic remains decoupled. For example, a "Buy" action triggers a `PurchaseItemEvent`, which is then processed by the vendor system to check funds, deduct currency, and transfer items.

---

## Core Concepts

### 1. Slot-Based Storage
Inventory uses a fixed-size slot array (`Vec<Option<InventoryItem>>`). This classic approach allows for:
-   **Grid UI representation**: Easy to map to a 2D grid.
-   **Slot indexing**: Items are accessed by their specific index.
-   **Empty slots**: Represented by `None`, allowing for "holes" in the inventory.

### 2. Item Stacking & Weight
Items have a `max_stack` property.
-   **Stacking**: When adding an item, the system first tries to merge it with existing stacks of the same ID.
-   **Overflow**: If a stack is full, the remainder flows into a new slot.
-   **Weight**: Each item has a weight. The `Inventory` component tracks `current_weight` vs `weight_limit`.

### 3. Economy & Trading
The economy relies on `Currency` components attached to entities.
-   **Vendors** acts as exchange points.
-   **Multipliers**: Vendors have `buy_multiplier` and `sell_multiplier`.
    -   *Example*: A "greedy" merchant might buy your items for 0.2x value but sell them for 2.0x value.
-   **Stock**: Vendors can have specific stock counts or `infinite` supplies (good for basics like arrows or potions).

---

## Component Reference

### Inventory Components

#### `Inventory`
The primary component for any entity that stores items.

```rust
#[derive(Component, Debug, Reflect)]
pub struct Inventory {
    pub items: Vec<Option<InventoryItem>>,
    pub max_slots: usize,
    pub weight_limit: f32,
    pub current_weight: f32,
}
```

-   **`items`**: The dynamic list of slots. Size is usually initialized to `max_slots`.
-   **`max_slots`**: Hard limit on direct inventory size (default: 24).
-   **`weight_limit`**: Maximum carry weight before penalties (logic for penalties handled by gameplay systems).
-   **`current_weight`**: Cached weight sum, updated via `recalculate_weight()`.

#### `InventoryItem`
The data object for an item. Note that this is cloned into the `Inventory` slots.

```rust
#[derive(Debug, Clone, Reflect)]
pub struct InventoryItem {
    pub item_id: String,     // Unique identifier (e.g., "sword_iron")
    pub name: String,        // Display name (e.g., "Iron Sword")
    pub quantity: i32,       // Current stack count
    pub max_stack: i32,      // Max per slot
    pub weight: f32,         // Weight per unit
    pub item_type: ItemType, // Enum: Weapon, Ammo, Consumable, etc.
    pub icon_path: String,   // Asset path for UI
    pub value: f32,          // Base gold value
    pub category: String,    // Sorting category
    pub min_level: u32,      // Usage requirement
    pub info: String,        // Description/Lore
}
```

#### `Equipment`
Separate from the main grid, this component handles active gear.

```rust
#[derive(Component, Debug, Default, Reflect)]
pub struct Equipment {
    pub main_hand: Option<InventoryItem>,
    pub armor: Option<InventoryItem>,
}
```

#### `PhysicalItem`
Attached to 3D world entities representing loot on the ground.

```rust
#[derive(Component, Debug, Reflect)]
pub struct PhysicalItem {
    pub item: InventoryItem,
}
```
*Interaction Logic*: When a player interacts with a `PhysicalItem` entity (via `InteractionType::Pickup`), the item data is copied to the player's `Inventory`, and the world entity is despawned.

### Currency Components

#### `Currency`
Tracks the wallet of a player or NPC.

```rust
#[derive(Component, Debug, Clone)]
pub struct Currency {
    pub amount: f32,
    pub currency_type: CurrencyType, // Default: Gold
}
```

### Vendor Components

#### `Vendor`
Configuration for a shop NPC.

```rust
#[derive(Component, Debug, Clone)]
pub struct Vendor {
    pub name: String,
    pub buy_multiplier: f32,     // Price modifier when player BUYS
    pub sell_multiplier: f32,    // Price modifier when player SELLS
    pub infinite_stock: bool,    // If true, new items added are infinite by default
    pub add_sold_items: bool,    // If true, items player sells are added to stock
    pub min_level_to_buy: u32,   // Global level requirement
    pub currency_type: CurrencyType,
}
```

#### `VendorInventory`
The actual stock held by the vendor. Distinct from `Inventory` to support shop-specific features like "Infinite Stock" flags per item.

```rust
#[derive(Component, Debug, Clone)]
pub struct VendorInventory {
    pub items: Vec<ShopItem>,
    pub categories: Vec<VendorCategory>,
}
```

---

## Systems & Logic

### Handing Pickups (`inventory.rs`)

The **Pickup System** listens for `InteractionEvent` where type is `Pickup`.

1.  **Event Detection**: Checks `InteractionEventQueue`.
2.  **Validation**:
    -   Source entity must have `Inventory`.
    -   Target entity must have `PhysicalItem`.
3.  **Addition Logic (`add_item`)**:
    -   **Pass 1 (Stacking)**: Iterates existing slots. If `item_id` matches and `quantity < max_stack`, fills the stack.
    -   **Pass 2 (Empty Slot)**: If items remain, finds the first `None` slot and fills it.
    -   **Fail**: If full, returns the leftover item (logic can drop it back on ground).
4.  **Cleanup**: If added successfully, the world entity is despawned.

### Inventory UI (`inventory.rs`)

The UI uses Bevy's UI (Flexbox) system.

-   **Structure**:
    -   `InventoryUIRoot`: Main container, absolute positioning.
    -   **Grid**: Flex container with `FlexWrap::Wrap`.
    -   **Slots**: Fixed size (`60px`), spawned in a loop (0..24).
-   **Updates (`update_inventory_ui`)**:
    -   Queries `Inventory` and `InventoryUISlot` children.
    -   Synchronizes visual state (Icon color, Count text) with the underlying data.
    -   *Note*: Currently uses colored rectangles as placeholders for icons.

### Trading Transaction Flow (`vendor.rs`)

Buying an item involves a multi-step check:

1.  **Event**: `PurchaseItemEvent` is fired (usually by UI interaction).
2.  **System**: `handle_purchase_events` processes the queue.
3.  **Checks**:
    -   **Stock**: Is `amount > 0` or is it `infinite`?
    -   **Funds**: Does `Currency.amount` cover `cost`? (Cost = `base_value * vendor_multiplier`).
4.  **Transaction**:
    -   Deduct funds from Player `Currency`.
    -   Decrement Vendor `ShopItem` stock (unless infinite).
    -   Add item to Player `Inventory` (Requires separate logic call or event trigger).
    -   *Note*: The current implementation logs the purchase; integration with `Inventory::add_item` would happen here.

---

## Events Reference

The system relies heavily on the "Queue Resource" pattern to handle events, a workaround for certain Bevy event limitations or to ensure persistence across specific frame steps.

### Inventory Events
| Event | Resource Queue | Description |
| :--- | :--- | :--- |
| `InteractionEvent` | `InteractionEventQueue` | Triggered by interacting with world objects. Used for **Pickups**. |

### Currency Events
| Event | Resource Queue | Description |
| :--- | :--- | :--- |
| `AddCurrencyEvent` | `AddCurrencyEventQueue` | Safe way to inject money (rewards, loot). |
| `RemoveCurrencyEvent` | `RemoveCurrencyEventQueue` | Safe debit. Fails if insufficient funds. |

### Vendor Events
| Event | Resource Queue | Description |
| :--- | :--- | :--- |
| `PurchaseItemEvent` | `PurchaseItemEventQueue` | Player attempts to buy from Vendor. |
| `SellItemEvent` | `SellItemEventQueue` | Player attempts to sell to Vendor. |
| `PurchaseFailedEvent` | `PurchaseFailedEventQueue` | Feedback for UI (e.g., "Not enough gold"). |

---

## Usage Patterns

### Spawning a Player with Inventory & Money

```rust
fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player, // Player Tag
        // 1. Add Inventory
        Inventory {
            max_slots: 30,
            weight_limit: 150.0,
            ..default() 
        },
        // 2. Add Equipment Slots
        Equipment::default(),
        // 3. Add Wallet
        Currency {
            amount: 500.0,
            currency_type: CurrencyType::Gold,
        },
    ));
}
```

### Creating a Loot Item

To drop an item in the world that can be picked up:

```rust
fn spawn_loot_sword(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 1. Define the Item Data
    let sword_item = InventoryItem {
        item_id: "steel_sword".into(),
        name: "Steel Sword".into(),
        quantity: 1,
        max_stack: 1,
        weight: 5.0,
        item_type: ItemType::Weapon,
        icon_path: "icons/weapon_sword.png".into(),
        value: 150.0,
        category: "Weapons".into(),
        min_level: 5,
        info: "A standard adventurer's blade.".into(),
    };

    // 2. Spawn Entity with Visuals + PhysicalItem component
    commands.spawn((
        // Visuals (Mesh, Material, Transform)
        SceneBundle {
            scene: asset_server.load("models/sword.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        // Logic Component
        PhysicalItem {
            item: sword_item,
        },
        // Make it interactable
        InteractionTarget {
            interaction_type: InteractionType::Pickup,
            label: "Pick up Steel Sword".into(),
            ..default()
        }
    ));
}
```

### Setting up a Merchant

```rust
fn setup_blacksmith(mut commands: Commands) {
    let mut inventory = VendorInventory::default();
    
    // Add an infinite supply of repair hammers
    inventory.add_item(ShopItem {
        item: InventoryItem { 
            name: "Repair Hammer".into(), 
            value: 10.0, 
            ..default() 
        },
        amount: 1,
        buy_price: 15.0, // Markup
        sell_price: 5.0,
        infinite: true,  // Never runs out
        min_level: 0,
        use_vendor_min_level: true,
    });

    commands.spawn((
        // Vendor Identity
        Vendor {
            name: "Village Blacksmith".into(),
            buy_multiplier: 1.2,  // Expensive to buy from
            sell_multiplier: 0.8, // Good place to sell
            infinite_stock: false,
            add_sold_items: true, // You can buy back what you sold
            ..default()
        },
        inventory, // Attach the stock
    ));
}
```

---

## Advanced Features & Implementation Details

### Recursive Stacking Logic
The `add_item` function implements a smart stacking algorithm to maximize inventory efficiency.

1.  **Partial Filling**: It searches for *all* valid stacks first. If you pick up 10 Potions, and you have two stacks of 95/100, it distributes the potions to fill those stacks to 100 first.
2.  **Splitting**: If a partial stack is filled and items remain, the remainder is carried over to the search for an empty slot.
3.  **Safety**: The recursion (or loop) ensures no items are lost unless the entire inventory is absolutely full.

### Dynamic Weight Recalculation
Weight is not calculated every frame to save performance.
-   **Trigger**: `recalculate_weight()` is called only when:
    -   Items are added.
    -   Items are removed.
    -   Stack sizes change.
-   **Calculation**: Sum of `(item.weight * item.quantity)` for all `Some(item)` slots.

### Vendor Category logic
The vendor system includes an auto-categorization feature:
-   **`update_vendor_categories`**: This system iterates through all items in a `VendorInventory`.
-   **Grouping**: It groups items by their `category` string field.
-   **Result**: Generates a list of `VendorCategory` structs, which can be used by the UI to create tabs (e.g., "Weapons", "Potions", "Materials") without manual configuration.

### Currency Safety
The currency system prevents atomic errors (like negative balance) via the `RemoveCurrencyEvent`.
-   If `amount_to_remove > strict balance`, the transaction is **aborted**.
-   A `CurrencyRemovalFailedEvent` is fired instead.
-   This allows the UI to catch the failure and play a "buzzer" sound or show a "Not Enough Gold" prompt, rather than breaking game logic or allowing debt.

## Troubleshooting

### "I picked up an item but it didn't appear!"
-   **Check 1**: Is the inventory full? Check logs for "Inventory full!" warning.
-   **Check 2**: Did `recalculate_weight` update? If weight limit is exceeded, logic might prevent pickup (depending on implementation flags).
-   **Check 3**: Are inputs connected? Ensure `handle_pickup_events` is running in the schedule.

### "Vendor won't buy my items"
-   **Event**: Listen for `SaleFailedEvent`.
-   **Reason**:
    -   `ItemNotFound`: The item data you sent doesn't match the vendor's expected format?
    -   `Vendor configuration`: Does the vendor have `currency_type` matching the player's wallet?

### "UI is not updating"
-   **State**: The UI only updates when the `Inventory` component changes or when `toggle_inventory_ui` sets visibility.
-   **Reactive**: Ensure your UI systems are querying `Changed<Inventory>` for performance, or running every frame if optimization is not yet a concern.

---

## Future Enhancements
-   **Durability**: Adding current/max durability to `InventoryItem`.
-   **Grid Tetris**: Moving from single-slot items to varying sizes (Diablo-style).
-   **Shops Restocking**: Timer-based system to refill `ShopItem` amounts.
-   **Bartering**: Skill-based dynamic adjustment of `buy_multiplier` based on Charisma stats.
