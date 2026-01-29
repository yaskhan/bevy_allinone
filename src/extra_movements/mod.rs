pub mod fly;
pub mod jetpack;

use bevy::prelude::*;

pub struct ExtraMovementsPlugin;

impl Plugin for ExtraMovementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(fly::FlyPlugin)
           .add_plugins(jetpack::JetpackPlugin);
    }
}
