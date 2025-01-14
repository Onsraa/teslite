mod components;
mod params;
mod plugins;
mod resources;
mod systems;

use bevy::prelude::*;
use crate::plugins::setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .run();
}
