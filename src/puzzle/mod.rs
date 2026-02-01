//! Puzzle System Module
//!
//! Interactive puzzles and environmental challenges.
//!
//! ## Overview
//!
//! This module provides a comprehensive puzzle system for creating interactive
//! environmental challenges. It supports multiple puzzle types including:
//!
//! - **Logic Puzzles**: Riddles, pattern matching, sequence puzzles
//! - **Physics Puzzles**: Object manipulation and placement
//! - **Timing Puzzles**: Rhythm-based challenges
//! - **Combination Puzzles**: Multi-step solution
//! - **Environmental Puzzles**: Using world elements
//!
//! ## Puzzle Components
//!
//! - **Buttons/Switches**: Activatable objects with toggle states
//! - **Levers**: State-changing objects with position tracking
//! - **Pressure Plates**: Weight-based triggers
//! - **Keys/Locks**: Item-based solutions
//! - **Puzzle Pieces**: Collectible components
//! - **Sequence Puzzles**: Press objects in correct order
//! - **Piano Systems**: Musical puzzle interfaces
//!
//! ## Puzzle States
//!
//! - **Unsolved**: Initial state
//! - **In Progress**: Puzzle being worked on
//! - **Solved**: Puzzle completed successfully
//! - **Failed**: Puzzle failed (optional)
//!
//! ## Integration
//!
//! The puzzle system integrates with:
//! - **Interaction System**: For player interaction
//! - **Event System**: For puzzle state changes
//! - **Inventory System**: For key-based puzzles
//! - **Audio System**: For feedback sounds

use bevy::prelude::*;

pub mod types;
pub mod systems;
pub mod builder;

use types::*;
use systems::*;
use builder::*;

// Re-export specific types for cleaner imports
pub use types::PuzzleSystem;
pub use types::PuzzleButton;
pub use types::PuzzleLever;
pub use types::PuzzlePressurePlate;
pub use types::PuzzleLock;
pub use types::PuzzleKey;
pub use types::PuzzleSequence;
pub use types::PuzzleSequenceItem;
pub use types::PuzzlePiano;
pub use types::PuzzlePianoKey;
pub use types::PuzzleObjectPlacement;
pub use types::PuzzleDraggable;
pub use types::PuzzleProgress;
pub use types::PuzzleHint;
pub use types::PuzzleTimer;
pub use types::PuzzleSound;
pub use types::PuzzleGizmo;
pub use types::PuzzleDebug;
pub use types::PuzzleState;
pub use types::LeverState;
pub use types::LockState;
pub use types::KeyType;
pub use types::PuzzleInteractable;
pub use types::PuzzleInteractionType;

// Re-export builder
pub use builder::PuzzleBuilder;

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components for reflection
            .register_type::<PuzzleSystem>()
            .register_type::<PuzzleButton>()
            .register_type::<PuzzleLever>()
            .register_type::<PuzzlePressurePlate>()
            .register_type::<PuzzleLock>()
            .register_type::<PuzzleKey>()
            .register_type::<PuzzleSequence>()
            .register_type::<PuzzleSequenceItem>()
            .register_type::<PuzzlePiano>()
            .register_type::<PuzzlePianoKey>()
            .register_type::<PuzzleObjectPlacement>()
            .register_type::<PuzzleDraggable>()
            .register_type::<PuzzleState>()
            .register_type::<PuzzleEvent>()
            .register_type::<PuzzleSolvedEvent>()
            .register_type::<PuzzleFailedEvent>()
            .register_type::<PuzzleResetEvent>()
            .register_type::<PuzzleProgress>()
            .register_type::<PuzzleHint>()
            .register_type::<PuzzleTimer>()
            .register_type::<PuzzleSound>()
            .register_type::<PuzzleGizmo>()
            .register_type::<PuzzleDebug>()
            .register_type::<PuzzleInteractable>()
            // Resources
            .init_resource::<PuzzleEventQueue>()
            .init_resource::<PuzzleSolvedEventQueue>()
            .init_resource::<PuzzleFailedEventQueue>()
            .init_resource::<PuzzleResetEventQueue>()
            .init_resource::<PuzzleDebugSettings>()
            .init_resource::<PuzzleUIState>()
            // Systems
            .add_systems(Update, (
                update_puzzle_buttons,
                update_puzzle_levers,
                update_puzzle_pressure_plates,
                update_puzzle_locks,
                update_puzzle_sequences,
                update_puzzle_pianos,
                update_puzzle_object_placements,
                update_puzzle_draggables,
            ).chain())
            .add_systems(Update, (
                update_puzzle_timers,
                process_puzzle_events,
                update_puzzle_ui,
                debug_draw_puzzle_gizmos,
                handle_puzzle_interactions,
            ).chain())
            .add_systems(Startup, setup_puzzle_ui);
    }
}
