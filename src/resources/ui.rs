use bevy::prelude::*;

#[derive(Resource)]
pub struct HudCar {
    pub accel_bar: Entity,
    pub brake_bar: Entity,
    pub angle_text: Entity,
    pub speed_text: Entity,
    pub transmission_mode_text: Entity,
}