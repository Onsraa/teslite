use crate::components::car::*;
use crate::components::ui::*;
use crate::resources::ui::*;

use bevy::asset::AssetServer;
use bevy::color::palettes::basic::{GREEN, RED};
use bevy::color::Color;
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let root = commands
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(500.0),
                position_type: PositionType::Absolute,
                left: Val::Px(25.0),
                top: Val::Px(25.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .id();

    let bars_container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .id();

    let accel_bar_container = commands
        .spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(100.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::BLACK),
        ))
        .id();

    let accel_bar = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(0.0),
                ..default()
            },
            BackgroundColor(Color::Srgba(GREEN)),
            AcceleratorBar,
        ))
        .id();

    commands.entity(accel_bar_container).add_child(accel_bar);

    let brake_bar_container = commands
        .spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(100.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::BLACK),
        ))
        .id();

    let brake_bar = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(0.0),
                ..default()
            },
            BackgroundColor(Color::Srgba(RED)),
            BrakeBar,
        ))
        .id();

    commands.entity(brake_bar_container).add_child(brake_bar);

    commands
        .entity(bars_container)
        .add_child(accel_bar_container);
    commands
        .entity(bars_container)
        .add_child(brake_bar_container);

    let angle_text = commands
        .spawn((
            Text::new("Angle : 0°/0°"),
            TextFont {
                font: font.clone(),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();

    let speed_text = commands
        .spawn((
            Text::new("Speed : 0.0"),
            TextFont {
                font: font.clone(),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();

    let transmission_mode_text = commands
        .spawn((
            Text::new("Mode : Park"),
            TextFont {
                font: font.clone(),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();

    commands.entity(root).add_child(bars_container);
    commands.entity(root).add_child(angle_text);
    commands.entity(root).add_child(speed_text);
    commands.entity(root).add_child(transmission_mode_text);

    commands.insert_resource(HudCar {
        accel_bar,
        brake_bar,
        angle_text,
        speed_text,
        transmission_mode_text,
    });
}

pub fn update_ui(
    hud: Res<HudCar>,
    mut q_accel_bar: Query<&mut Node, With<AcceleratorBar>>,
    mut q_brake_bar: Query<&mut Node, (With<BrakeBar>, Without<AcceleratorBar>)>,
    mut q_text: Query<&mut Text>,
    car_query: Query<&CarPhysics>,
) {
    if let Ok(car) = car_query.get_single() {
        let accel_percent = car.accelerator * 100.0;
        let brake_percent = car.brake * 100.0;

        if let Ok(mut accel_bar) = q_accel_bar.get_single_mut() {
            accel_bar.height = Val::Percent(accel_percent);
        }

        if let Ok(mut brake_bar) = q_brake_bar.get_single_mut() {
            brake_bar.height = Val::Percent(brake_percent);
        }

        if let Ok(mut angle_text) = q_text.get_mut(hud.angle_text) {
            angle_text.0 = format!(
                "Angle : {:.4}/{:.4}",
                car.steering_angle, car.max_steering_angle
            );
        }

        if let Ok(mut speed_text) = q_text.get_mut(hud.speed_text) {
            speed_text.0 = format!("Speed : {:.2}", car.speed);
        }

        if let Ok(mut mode_text) = q_text.get_mut(hud.transmission_mode_text) {
            let mode = match car.mode {
                TransmissionMode::Park => "Park",
                TransmissionMode::Drive => "Drive",
                TransmissionMode::Reverse => "Reverse",
            };
            mode_text.0 = format!("Mode : {}", mode);
        }
    }
}
