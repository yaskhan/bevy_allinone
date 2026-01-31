use bevy::prelude::*;
use super::types::*;

// ============================================================================
// Puzzle Builder
// ============================================================================

/// Builder for creating puzzle systems
pub struct PuzzleBuilder<'c, 'a, 'b> {
    commands: &'c mut Commands<'a, 'b>,
    puzzle_entity: Entity,
}

impl<'c, 'a, 'b> PuzzleBuilder<'c, 'a, 'b> {
    /// Create a new puzzle builder
    pub fn new(commands: &'c mut Commands<'a, 'b>) -> Self {
        let puzzle_entity = commands.spawn_empty().id();
        Self {
            commands,
            puzzle_entity,
        }
    }

    /// Add a puzzle system component
    pub fn with_puzzle_system(mut self, system: PuzzleSystem) -> Self {
        self.commands.entity(self.puzzle_entity).insert(system);
        self
    }

    /// Add a puzzle button
    pub fn with_button(mut self, button: PuzzleButton, transform: Transform) -> Self {
        let button_entity = self.commands.spawn((button, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(button_entity);
        self
    }

    /// Add a puzzle lever
    pub fn with_lever(mut self, lever: PuzzleLever, transform: Transform) -> Self {
        let lever_entity = self.commands.spawn((lever, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(lever_entity);
        self
    }

    /// Add a puzzle pressure plate
    pub fn with_pressure_plate(mut self, plate: PuzzlePressurePlate, transform: Transform) -> Self {
        let plate_entity = self.commands.spawn((plate, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(plate_entity);
        self
    }

    /// Add a puzzle lock
    pub fn with_lock(mut self, lock: PuzzleLock, transform: Transform) -> Self {
        let lock_entity = self.commands.spawn((lock, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(lock_entity);
        self
    }

    /// Add a puzzle key
    pub fn with_key(mut self, key: PuzzleKey, transform: Transform) -> Self {
        let key_entity = self.commands.spawn((key, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(key_entity);
        self
    }

    /// Add a puzzle sequence
    pub fn with_sequence(mut self, sequence: PuzzleSequence, transform: Transform) -> Self {
        let sequence_entity = self.commands.spawn((sequence, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(sequence_entity);
        self
    }

    /// Add a puzzle sequence item
    pub fn with_sequence_item(mut self, item: PuzzleSequenceItem, transform: Transform) -> Self {
        let item_entity = self.commands.spawn((item, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(item_entity);
        self
    }

    /// Add a puzzle piano
    pub fn with_piano(mut self, piano: PuzzlePiano, transform: Transform) -> Self {
        let piano_entity = self.commands.spawn((piano, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(piano_entity);
        self
    }

    /// Add a piano key
    pub fn with_piano_key(mut self, key: PuzzlePianoKey, transform: Transform) -> Self {
        let key_entity = self.commands.spawn((key, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(key_entity);
        self
    }

    /// Add a puzzle object placement
    pub fn with_object_placement(mut self, placement: PuzzleObjectPlacement, transform: Transform) -> Self {
        let placement_entity = self.commands.spawn((placement, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(placement_entity);
        self
    }

    /// Add a puzzle draggable
    pub fn with_draggable(mut self, draggable: PuzzleDraggable, transform: Transform) -> Self {
        let draggable_entity = self.commands.spawn((draggable, transform)).id();
        self.commands.entity(self.puzzle_entity).add_child(draggable_entity);
        self
    }

    /// Add puzzle progress
    pub fn with_progress(mut self, progress: PuzzleProgress) -> Self {
        self.commands.entity(self.puzzle_entity).insert(progress);
        self
    }

    /// Add puzzle hint
    pub fn with_hint(mut self, hint: PuzzleHint) -> Self {
        self.commands.entity(self.puzzle_entity).insert(hint);
        self
    }

    /// Add puzzle timer
    pub fn with_timer(mut self, timer: PuzzleTimer) -> Self {
        self.commands.entity(self.puzzle_entity).insert(timer);
        self
    }

    /// Add puzzle sound
    pub fn with_sound(mut self, sound: PuzzleSound) -> Self {
        self.commands.entity(self.puzzle_entity).insert(sound);
        self
    }

    /// Add puzzle gizmo
    pub fn with_gizmo(mut self, gizmo: PuzzleGizmo) -> Self {
        self.commands.entity(self.puzzle_entity).insert(gizmo);
        self
    }

    /// Add puzzle debug
    pub fn with_debug(mut self, debug: PuzzleDebug) -> Self {
        self.commands.entity(self.puzzle_entity).insert(debug);
        self
    }

    /// Build the puzzle and return the entity
    pub fn build(self) -> Entity {
        self.puzzle_entity
    }
}
