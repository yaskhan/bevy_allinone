pub mod fly;
pub mod jetpack;
pub mod wall_run;
pub mod swim;
pub mod paraglider;
pub mod roll_on_landing;
pub mod sphere_mode;
pub mod free_fall;

use bevy::prelude::*;

pub struct ExtraMovementsPlugin;

impl Plugin for ExtraMovementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(fly::FlyPlugin)
           .add_plugins(jetpack::JetpackPlugin)
           .add_plugins(wall_run::WallRunPlugin)
           .add_plugins(swim::SwimPlugin)
           .add_plugins(paraglider::ParagliderPlugin)
           .add_plugins(roll_on_landing::RollOnLandingPlugin)
           .add_plugins(sphere_mode::SphereModePlugin)
           .add_plugins(free_fall::FreeFallPlugin);
    }
}
