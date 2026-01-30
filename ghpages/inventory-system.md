# Inventory and Economy

A modular system for managing items, currency, and trading.

## Features

- **Slot-Based Inventory**: Flexible container system for players and storage.
- **Currency System**: Handling of multiple currency types.
- **Vendors**: Buying and selling logic with dynamic pricing.

## Components

- `Inventory`: Attached to entities that can hold items.
- `Item`: Data component for individual items.
- `Currency`: Resource or component for wealth tracking.

## Systems

- `inventory_management_system`: Handles item moves, splits, and usage.
- `vendor_trading_system`: Manages the exchange of items and currency.
