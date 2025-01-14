use bevy::prelude::*;
use crate::components::car::get_car_plugin;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(get_car_plugin());
    }
}
