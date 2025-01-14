use bevy::prelude::*;
use crate::systems;

use systems::ui::car_driving_info::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, update_ui);
    }
}