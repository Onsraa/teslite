use bevy::prelude::*;
use crate::components::car::*;
use crate::resources::environment::*;
use crate::systems::car::*;
use crate::systems::ui::drive_info::draw_axes;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SurfaceProperties {
            friction_coefficient: 1.0,
        });
        app.add_systems(Startup, setup_camera);
        app.add_systems(Startup, spawn_car);
        app.add_systems(Update, control_car.before(update_car_physics));
        app.add_systems(Update, (update_car_physics, draw_axes).chain());
    }
}
