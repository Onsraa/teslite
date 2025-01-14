use bevy::{prelude::*, render::primitives::Aabb};

const ACCELERATION_VALUE: f32 = 5.0;
const ROTATION_VALUE: f32 = 5.0;
pub(crate) const BRAKE_COEFFICIENT: f32 = 3.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TransmissionMode {
    Park,
    Drive,
    Reverse,
}

#[derive(Component)]
pub struct CarPhysics {
    pub speed: f32,
    pub heading: f32,

    // Direction
    pub steering_angle: f32,
    pub target_steering_angle: f32,
    pub max_steering_angle: f32,
    pub steering_angle_speed: f32,

    // Caractéristiques
    pub max_speed: f32,
    pub wheelbase: f32,
    pub mass: f32,
    pub tire_grip: f32,

    // Pédales
    pub accelerator: f32,        // entre 0 et 1
    pub brake: f32,              // entre 0 et 1

    // Paramètres de ramp
    pub accel_ramp_up: f32,
    pub accel_ramp_down: f32,
    pub brake_ramp_up: f32,
    pub brake_ramp_down: f32,

    // Forces
    pub max_acceleration: f32,
    pub max_braking: f32,

    pub mode: TransmissionMode,

    pub idle_speed_forward: f32,
    pub idle_speed_reverse: f32,
}

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
pub(crate) struct ShowAxes;
