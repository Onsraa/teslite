use bevy::prelude::*;
use crate::components::car::{CarPhysics, TransmissionMode};
use crate::resources::environment::*;

pub fn update_car_physics(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut CarPhysics)>,
    surface: Res<SurfaceProperties>
) {
    let dt = time.delta_secs();
    for (mut transform, mut physics) in query.iter_mut() {
        // Ajustement de l'angle de braquage
        let angle_diff = physics.target_steering_angle - physics.steering_angle;
        let max_angle_change = physics.steering_angle_speed * dt;
        if angle_diff.abs() < max_angle_change {
            physics.steering_angle = physics.target_steering_angle;
        } else {
            physics.steering_angle += max_angle_change * angle_diff.signum();
        }

        let idle_speed = match physics.mode {
            TransmissionMode::Park => 0.0,
            TransmissionMode::Drive => physics.idle_speed_forward,
            TransmissionMode::Reverse => physics.idle_speed_reverse,
        };

        let mut accel = 0.0;

        if physics.mode == TransmissionMode::Park {
            // Véhicule immobilisé
            physics.speed = 0.0;
        } else {
            let pedal_accel = physics.accelerator * physics.max_acceleration;

            let brake_intensity = 1.0 - (-crate::components::car::BRAKE_COEFFICIENT * physics.brake).exp();
            let pedal_brake = brake_intensity * physics.max_braking;

            if physics.accelerator == 0.0 && physics.brake == 0.0 {
                // Tendre vers idle_speed
                let diff = idle_speed - physics.speed;
                accel = diff;
            } else {
                // Mode Drive ou Reverse
                let direction = if physics.mode == TransmissionMode::Drive { 1.0 } else { -1.0 };
                accel += pedal_accel * direction;

                let brake_direction = if physics.speed > 0.0 { -1.0 } else if physics.speed < 0.0 { 1.0 } else { 0.0 };
                accel += pedal_brake * brake_direction;
            }

            accel *= surface.friction_coefficient * physics.tire_grip;

            let actual_accel = accel / physics.mass;
            physics.speed += actual_accel * dt;

            if physics.speed > physics.max_speed {
                physics.speed = physics.max_speed;
            } else if physics.speed < -physics.max_speed {
                physics.speed = -physics.max_speed;
            }
        }

        if physics.mode != TransmissionMode::Park {
            let yaw_rate = if physics.steering_angle.abs() > 1e-6 {
                (physics.speed / physics.wheelbase) * physics.steering_angle.tan()
            } else {
                0.0
            };

            physics.heading += yaw_rate * dt;
            let dx = physics.speed * physics.heading.cos() * dt;
            let dy = physics.speed * physics.heading.sin() * dt;

            transform.translation.x += dx;
            transform.translation.y += dy;
            transform.rotation = Quat::from_rotation_z(physics.heading);
        }
    }
}

pub fn control_car(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CarPhysics>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for mut physics in query.iter_mut() {
        // Changement de mode
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            physics.mode = TransmissionMode::Park;
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            physics.mode = TransmissionMode::Drive;
        }
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            physics.mode = TransmissionMode::Reverse;
        }

        // Déterminer la cible pour l'accélérateur et le frein
        // Flèche Haut = accélérateur, Flèche Bas = frein
        let accel_target = if keyboard_input.pressed(KeyCode::ArrowUp) { 1.0 } else { 0.0 };
        let brake_target = if keyboard_input.pressed(KeyCode::ArrowDown) { 1.0 } else { 0.0 };

        // Mettre à jour l'accélérateur
        if accel_target > physics.accelerator {
            let diff = accel_target - physics.accelerator;
            let max_change = physics.accel_ramp_up * dt;
            physics.accelerator += diff.min(max_change);
        } else {
            let diff = physics.accelerator - accel_target;
            let max_change = physics.accel_ramp_down * dt;
            physics.accelerator -= diff.min(max_change);
        }

        // Mettre à jour le frein
        if brake_target > physics.brake {
            let diff = brake_target - physics.brake;
            let max_change = physics.brake_ramp_up * dt;
            physics.brake += diff.min(max_change);
        } else {
            let diff = physics.brake - brake_target;
            let max_change = physics.brake_ramp_down * dt;
            physics.brake -= diff.min(max_change);
        }

        // Direction (inchangé)
        let mut steering_target = physics.target_steering_angle;
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            let steering_speed = 0.1 * (1.0 / (1.0 + physics.speed.abs() / physics.max_speed)); // Réduit à haute vitesse
            steering_target -= steering_speed * dt;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            let steering_speed = 0.1 * (1.0 / (1.0 + physics.speed.abs() / physics.max_speed));
            steering_target += steering_speed * dt;
        }

        if steering_target > physics.max_steering_angle {
            steering_target = physics.max_steering_angle;
        } else if steering_target < -physics.max_steering_angle {
            steering_target = -physics.max_steering_angle;
        }
        physics.target_steering_angle = steering_target;
    }
}

pub fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        crate::components::car::ShowAxes,
        Mesh2d(meshes.add(Rectangle::new(100.0, 50.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        CarPhysics {
            speed: 0.0,
            heading: 0.0,
            steering_angle: 0.0,
            target_steering_angle: 0.0,
            max_steering_angle: 0.02,
            steering_angle_speed: 2.0,

            max_speed: 200.0,
            wheelbase: 2.5,

            mass: 1.0,
            tire_grip: 1.0,

            accelerator: 0.0,
            brake: 0.0,

            accel_ramp_up: 8.0,
            accel_ramp_down: 8.0,
            brake_ramp_up: 40.0,
            brake_ramp_down: 10.0,

            max_acceleration: 80.0,
            max_braking: 300.0,

            mode: TransmissionMode::Park,

            idle_speed_forward: 30.0,
            idle_speed_reverse: -30.0,
        },
    ));
}