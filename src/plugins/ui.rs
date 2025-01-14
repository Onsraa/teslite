use crate::systems;
use bevy::prelude::*;

use systems::ui::drive_info::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, update_ui);
    }
}
