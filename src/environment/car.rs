use bevy::prelude::*;

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
    pub idle_speed: f32,         // vitesse "ralenti" avec pédales relâchées

    // Paramètres de ramp
    pub accel_ramp_up: f32,
    pub accel_ramp_down: f32,
    pub brake_ramp_up: f32,
    pub brake_ramp_down: f32,

    // Forces
    pub max_acceleration: f32,
    pub max_braking: f32,
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

        // Calcul de l'accélération
        let mut accel = physics.accelerator * physics.max_acceleration
            - physics.brake * physics.max_braking;

        // Si ni frein ni accélérateur, on se rapproche de idle_speed
        if physics.accelerator == 0.0 && physics.brake == 0.0 {
            if physics.speed < physics.idle_speed {
                // Accélère doucement pour remonter vers idle_speed
                accel = 1.0; // une petite valeur pour tendre vers idle_speed
            } else if physics.speed > physics.idle_speed {
                // Décélère doucement (friction)
                accel = -1.0;
            } else {
                // Vitesse déjà à idle_speed, pas d'accélération
                accel = 0.0;
            }
        }

        // Mise à jour de la vitesse
        physics.speed += accel * dt;
        if physics.speed < 0.0 {
            physics.speed = 0.0;
        }

        // Mise à jour position et orientation
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

fn control_car(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CarPhysics>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for mut physics in query.iter_mut() {
        // Déterminer la cible pour l'accélérateur et le frein
        let accel_target = if keyboard_input.pressed(KeyCode::ArrowUp) { 1.0 } else { 0.0 };
        let brake_target = if keyboard_input.pressed(KeyCode::ArrowDown) { 1.0 } else { 0.0 };

        // Mettre à jour l'accélérateur
        if accel_target > physics.accelerator {
            // On augmente l'accélérateur vers la cible
            let diff = accel_target - physics.accelerator;
            let max_change = physics.accel_ramp_up * dt;
            physics.accelerator += diff.min(max_change);
        } else {
            // On diminue l'accélérateur vers la cible
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

        // Direction
        let mut steering_target = physics.target_steering_angle;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            steering_target -= 0.1 * dt;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            steering_target += 0.1 * dt;
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
            max_steering_angle: 0.5,
            steering_angle_speed: 1.0,

            max_speed: 20.0,
            wheelbase: 2.5,

            accelerator: 0.0,
            brake: 0.0,
            idle_speed: 1.0,

            accel_ramp_up: 0.5,
            accel_ramp_down: 1.0,
            brake_ramp_up: 1.0,
            brake_ramp_down: 1.5,

            max_acceleration: 4.0,
            max_braking: 5.0,
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