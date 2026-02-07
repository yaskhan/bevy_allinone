# Inventory System To-Do List

Based on the comparison with the legacy C# `inventoryManager.cs`, here is the breakdown of implemented vs. missing features in the Rust/Bevy implementation.

## Features Status

### Basic Management
- [x] **Add Items** (`Inventory::add_item`)
- [x] **Remove/Drop Items** (`inventory_drop_system.rs`)
- [x] **Examine Items** (`inventory_examine_system.rs`)
    - *Note:* 3D preview, rotation, and input-driven zoom are implemented.
- [x] **Combine Items** (`inventory_combine_system.rs`)
- [x] **Stacking Logic** (Merge on add / Split stack)
- [x] **Weight System** (Calculated in `Inventory` component)

### Equipment & Interaction
- [x] **Quick Access Slots** (`InventoryQuickAccessSlotsSystem`)
- [x] **Equipping Weapons** (`melee_weapon_equipment_system.rs`, `weapon_equip_system.rs`)
- [x] **Usage** (`use_inventory_object.rs`)
    - *Status:* Item effects implemented via `item_effects.rs` + `item_usage_system.rs`.

### User Interface (UI) - **Priorities**
The UI layer has the most gaps compared to the Unity implementation.

- [ ] **Visual Polish & Layout**
    - [ ] Implement Scroll View for inventory slots (currently fixed grid).
    - [ ] Create detailed item icons.
    - [ ] Add background panels and blur effects (if desired).
- [ ] **Interactive Features**
    - [x] **Drag & Drop**: Implement drag-and-drop reordering of items.
    - [x] **Selection**: persistent selection state to show details in a side panel.
- [ ] **Info Panels / Tooltips**
    - [x] Show `Description` and `Object Info` in a dedicated UI panel when an item is selected.
    - [x] "Full Inventory" / "Too Heavy" on-screen feedback messages.
- [ ] **Zoom Controls**
    - [x] Implement input handling to zoom in/out with scroll wheel during item examination.

## Missing Backend Features
- [x] **Save/Load System Integration**: Ensure `Inventory` component is serialized/deserialized correctly with the save system.

### Advanced C# Features to Port
- [x] **Dual Weapon Slot**: Logic for handling dual-wielding (left/right weapon assignment) in `InventoryQuickAccessSlotsSystem`.
- [x] **Drop Customization**:
    - [x] `dropSingleObjectOnInfiniteAmount`: Config to drop 1 vs reset stack when dropping infinite items.
    - [x] `setTotalAmountWhenDropObject`: Option to drop entire stack as one physical object vs multiple individual objects.
- [x] **Inventory Slot Options Panel**:
    - [x] Context menu on slot click (Use, Equip, Drop, Combine, Examine, Discard). C# has `inventoryOptionsOnSlotPanel`.
- [ ] **Auto-Equip Rules**:
    - [x] `equipWeaponsWhenPicked`: Auto-equip weapon on pickup.
    - [x] `equipPickedWeaponOnlyItNotPreviousWeaponEquipped`: Logic to avoid replacing currently active weapon.
    - [x] `swapping`: Logic to swap currently held weapon with picked one if full.
- [ ] **Ammo Integration**:
    - [x] `checkIfWeaponUseAmmoFromInventory`: Logic to sync weapon ammo clip with inventory reserves. (Implemented in `ammo_sync_system.rs`, gated by `use_ammo_from_inventory`).
- [ ] **Audio Feedback**:
    - [ ] `useAudioSounds`: Play specific clips on open/close, pickup, drop, use, combine.
- [x] **Examine Mode Polish**:
    - [x] `placeObjectInCameraPosition` / `Rotation`: Smooth coroutine-like transitions for preview object appearing/rotating.
    - [x] `takeObjectInExaminePanelButton`: Logic to pick up an object directly from the examine screen (e.g. reading a note found in world).

