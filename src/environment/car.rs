use bevy::prelude::*;

const ACCELERATION_VALUE: f32 = 5.0;
const ROTATION_VALUE: f32 = 5.0;

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

fn update_car_physics(time: Res<Time>, mut query: Query<(&mut Transform, &mut CarPhysics)>) {
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

        // Déterminer l'idle_speed actuel selon le mode
        let idle_speed = match physics.mode {
            TransmissionMode::Park => 0.0,
            TransmissionMode::Drive => physics.idle_speed_forward,
            TransmissionMode::Reverse => physics.idle_speed_reverse,
        };

        let mut accel = 0.0;

        if physics.mode == TransmissionMode::Park {
            // En mode Park, on force la voiture à s'arrêter
            physics.speed = 0.0;
            accel = 0.0;
        } else {
            // Mode Drive ou Reverse
            // Accélération due aux pédales
            let pedal_accel = physics.accelerator * physics.max_acceleration;
            let pedal_brake = physics.brake * physics.max_braking;

            // Sens de l'idle_speed : en Drive, idle_speed > 0, donc on tend vers l'avant
            // en Reverse, idle_speed < 0, on tend vers l'arrière
            // L'accélérateur augmente la vitesse vers le sens de l'idle_speed
            // Le frein ramène la vitesse vers 0.

            // Si ni accélérateur ni frein n'est activé, on tend vers idle_speed
            if physics.accelerator == 0.0 && physics.brake == 0.0 {
                let diff = idle_speed - physics.speed;
                // Appliquer une petite force pour tendre vers idle_speed
                // On peut par exemple faire un diff * kp avec kp = 1.0
                accel = diff;
            } else {
                // On a soit accélérateur, soit frein (ou les deux)
                // L'accélérateur pousse la vitesse vers idle_speed+ si drive, idle_speed- si reverse
                let direction = if physics.mode == TransmissionMode::Drive { 1.0 } else { -1.0 };

                // L'accélérateur ajoute de la vitesse dans le sens direction
                accel += pedal_accel * direction;

                // Le frein ramène la vitesse vers 0
                // Si la vitesse est positive et on freine, on applique accel négative
                // Si la vitesse est négative et on freine, on applique accel positive vers 0
                let brake_direction = if physics.speed > 0.0 { -1.0 } else if physics.speed < 0.0 { 1.0 } else { 0.0 };
                accel += pedal_brake * brake_direction;
            }

            // Mise à jour de la vitesse
            physics.speed += accel * dt;

            // On limite la vitesse
            // En mode Drive, on ne veut pas de vitesse négative ?
            // On peut autoriser une petite marge, mais logiquement en Drive on ne recule pas si on accélère pas
            // On va clamp la vitesse dans tous les cas entre -max_speed et max_speed
            if physics.speed > physics.max_speed {
                physics.speed = physics.max_speed;
            } else if physics.speed < -physics.max_speed {
                physics.speed = -physics.max_speed;
            }
        }

        // Mise à jour position et orientation
        if physics.mode == TransmissionMode::Park {
            // Pas de déplacement
        } else {
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

fn control_car(
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

fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 50.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        CarPhysics {
            speed: 0.0,
            heading: 0.0,
            steering_angle: 0.0,
            target_steering_angle: 0.0,
            max_steering_angle: 0.05,
            steering_angle_speed: 3.0,

            max_speed: 125.0,
            wheelbase: 2.5,

            accelerator: 0.0,
            brake: 0.0,

            accel_ramp_up: 5.0,
            accel_ramp_down: 5.0,
            brake_ramp_up: 15.0,
            brake_ramp_down: 1.5,

            max_acceleration: 40.0,
            max_braking: 50.0,

            mode: TransmissionMode::Park,

            idle_speed_forward: 1.0,
            idle_speed_reverse: -1.0,
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Startup, spawn_car);
        app.add_systems(Update, control_car.before(update_car_physics));
        app.add_systems(Update, update_car_physics);
    }
}

pub fn get_car_plugin() -> CarPlugin {
    CarPlugin
}