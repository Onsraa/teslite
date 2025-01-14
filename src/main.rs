mod components;
mod params;
mod plugins;
mod resources;
mod systems;

use crate::plugins::environment_plugin::EnvironmentPlugin;
use crate::plugins::ui_plugin::UIPlugin;
use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowMode},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Teslite".into(),
                    name: Some("bevy.app".into()),
                    present_mode: PresentMode::AutoNoVsync,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            EnvironmentPlugin,
            UIPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (make_visible, exit_game))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}

fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}
