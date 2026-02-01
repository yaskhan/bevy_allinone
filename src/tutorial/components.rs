use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use super::events::TutorialEvent;

/// Component that tracks which tutorials a player has already seen.
#[derive(Component, Debug, Default, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct TutorialLog {
    pub played_tutorials: HashSet<u32>,
}

#[derive(Component)]
pub struct TutorialRoot;

#[derive(Component)]
pub struct TutorialTitleText;

#[derive(Component)]
pub struct TutorialDescriptionText;

#[derive(Component)]
pub struct TutorialPanelImage;

#[derive(Component)]
pub struct TutorialButton(pub TutorialEvent);
